/// List Utils - Terminal UI application for manipulating and comparing lists
mod app;
mod clipboard;
mod events;
mod operations;
mod parser;
mod ui;

use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, fs, io, path::PathBuf};

use crate::events::{is_alt_number, is_copy_paste_key, is_key, read_event, InputEvent};
use crate::operations::{compare_lists, process_single_list};
use crate::parser::{parse_list, Delimiter};
use crate::ui::{
    create_layout_with_tabs, create_results_grid, render_list_panel, render_result_list_panel,
    render_results_panel, render_status_bar, render_tabs,
};
use tui_textarea::Input;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create application
    let mut app = App::new();

    // Main event loop
    loop {
        terminal.draw(|f| {
            let (tabs_area, list1_area, list2_area, results_area, status_area, content_area_tab2) =
                create_layout_with_tabs(f.area());

            // Render tabs
            render_tabs(f, tabs_area, app.active_tab);

            // Render content based on active tab
            if app.active_tab == 0 {
                // Tab 1: Input view
                render_list_panel(
                    f,
                    list1_area,
                    "LIST 1",
                    &mut app.list1,
                    app.active_panel == 0,
                );
                render_list_panel(
                    f,
                    list2_area,
                    "LIST 2",
                    &mut app.list2,
                    app.active_panel == 1,
                );
                // Render INFO panel with dynamic hints
                let info_hints = match app.active_panel {
                    0 => vec![
                        "List 1: Ctrl+C (Copy) | Ctrl+V (Paste) | F2 (Load)".to_string(),
                        "Press F12 to Compare with List 2 | F5 (Delim)".to_string(),
                    ],
                    1 => vec![
                        "List 2: Ctrl+C (Copy) | Ctrl+V (Paste) | F2 (Load)".to_string(),
                        "Press F12 to Compare with List 1 | F5 (Delim)".to_string(),
                    ],
                    _ => {
                        // Show current app results (success messages, stats) or default tips
                        if !app.results.is_empty() && !app.results[0].contains("Welcome") {
                            app.results.clone()
                        } else {
                            vec![
                                "INFO: Compare: F9 | Sort: F6/F7 | Dedup: F8".to_string(),
                                "Save: F1 | Load: F2 | Tab: Next Panel".to_string(),
                            ]
                        }
                    }
                };
                render_results_panel(f, results_area, &info_hints, 0, app.active_panel == 2);
            } else if app.active_tab == 1 {
                // Tab 2: Results view
                if app.diff_view_mode == 1 {
                    // Unified Diff View
                    if let Some(ref compare_results) = app.compare_results {
                        crate::ui::render_unified_diff_panel(f, content_area_tab2, compare_results);
                    } else {
                        crate::ui::render_result_list_panel(
                            f,
                            content_area_tab2,
                            "Unified Diff (0 items)",
                            &[],
                            false,
                        );
                    }
                } else {
                    // Grid View: use split layout
                    let (only_l1_area, only_l2_area, intersection_area, union_area) =
                        create_results_grid(content_area_tab2);

                    if let Some(ref compare_results) = app.compare_results {
                        let only_l1_title = format!(
                            "Only in List 1 ({} items)",
                            compare_results.only_in_first.len()
                        );
                        let only_l2_title = format!(
                            "Only in List 2 ({} items)",
                            compare_results.only_in_second.len()
                        );
                        let intersection_title = format!(
                            "Intersection ({} items)",
                            compare_results.intersection.len()
                        );
                        let union_title = format!("Union ({} items)", compare_results.union.len());

                        render_result_list_panel(
                            f,
                            only_l1_area,
                            &only_l1_title,
                            &compare_results.only_in_first,
                            app.active_panel == 0,
                        );
                        render_result_list_panel(
                            f,
                            only_l2_area,
                            &only_l2_title,
                            &compare_results.only_in_second,
                            app.active_panel == 1,
                        );
                        render_result_list_panel(
                            f,
                            intersection_area,
                            &intersection_title,
                            &compare_results.intersection,
                            app.active_panel == 2,
                        );
                        render_result_list_panel(
                            f,
                            union_area,
                            &union_title,
                            &compare_results.union,
                            app.active_panel == 3,
                        );
                    } else {
                        // No results yet
                        render_result_list_panel(
                            f,
                            only_l1_area,
                            "Only in List 1 (0 items)",
                            &[],
                            app.active_panel == 0,
                        );
                        render_result_list_panel(
                            f,
                            only_l2_area,
                            "Only in List 2 (0 items)",
                            &[],
                            app.active_panel == 1,
                        );
                        render_result_list_panel(
                            f,
                            intersection_area,
                            "Intersection (0 items)",
                            &[],
                            app.active_panel == 2,
                        );
                        render_result_list_panel(
                            f,
                            union_area,
                            "Union (0 items)",
                            &[],
                            app.active_panel == 3,
                        );
                    }
                }
                // Render INFO panel for Results tab
                let results_info = vec![
                    "Results: Tab (Next Panel) | F12 (Toggle View: Diff/Grid)".to_string(),
                    "F1 (Save Panel) | Alt+1 (Go back to inputs) | ?: Help".to_string(),
                ];
                render_results_panel(f, results_area, &results_info, 0, false);
            } else {
                // Tab 3: Convert delimiters
                render_list_panel(
                    f,
                    list1_area,
                    "CONVERT INPUT",
                    &mut app.convert_input,
                    app.active_panel == 0,
                );

                render_result_list_panel(
                    f,
                    list2_area,
                    "CONVERT OUTPUT",
                    &app.convert_output_items,
                    app.active_panel == 1,
                );

                let convert_info = match app.active_panel {
                    0 => vec![
                        format!(
                            "Src: [ ({}) ] | Dst: [ ({}) ] | Convert: F12",
                            app.convert_source_delimiter.display_name(),
                            app.convert_target_delimiter.display_name()
                        ),
                        "Paste: Ctrl+V | Load: F2 | Cycle Src: F10".to_string(),
                    ],
                    _ => vec![
                        format!(
                            "Result: {} items | Dst: {}",
                            app.convert_output_items.len(),
                            app.convert_target_delimiter.display_name()
                        ),
                        "Copy: Ctrl+C | Save: F1 | Cycle Dst: F11".to_string(),
                    ],
                };
                render_results_panel(f, results_area, &convert_info, 0, false);
            }

            let active_panel_info = active_panel_label(&app);
            let convert_delims = if app.active_tab == 2 {
                Some((app.convert_source_delimiter, app.convert_target_delimiter))
            } else {
                None
            };
            render_status_bar(
                f,
                status_area,
                app.delimiter,
                convert_delims,
                app.active_tab,
                active_panel_info.as_deref(),
            );

            if app.show_help {
                crate::ui::render_help_modal(f);
            }
        })?;

        // Handle events
        match read_event()? {
            InputEvent::Key(key_event) => {
                // Handle keyboard shortcuts
                if app.show_help {
                    app.show_help = false;
                } else if is_key(&key_event, KeyCode::Esc) {
                    app.should_quit = true;
                } else if is_key(&key_event, KeyCode::Char('?')) {
                    app.toggle_help();
                } else if is_alt_number(&key_event, 1) {
                    app.go_to_tab(0);
                } else if is_alt_number(&key_event, 2) {
                    app.go_to_tab(1);
                } else if is_alt_number(&key_event, 3) {
                    app.go_to_tab(2);
                } else if is_key(&key_event, KeyCode::Tab) {
                    app.switch_panel();
                } else if is_key(&key_event, KeyCode::F(1)) {
                    handle_save_to_file(&mut app)?;
                } else if is_key(&key_event, KeyCode::F(2)) {
                    handle_load_from_file(&mut app)?;
                } else if is_key(&key_event, KeyCode::F(3)) {
                    app.toggle_case_sensitivity();
                    let state = if app.compare_options.case_sensitive {
                        "ON"
                    } else {
                        "OFF"
                    };
                    app.results = vec![format!("Case sensitivity {}", state)];
                } else if is_key(&key_event, KeyCode::F(4)) {
                    app.toggle_trim_spaces();
                    let state = if app.compare_options.trim_spaces {
                        "ON"
                    } else {
                        "OFF"
                    };
                    app.results = vec![format!("Trim spaces {}", state)];
                } else if is_key(&key_event, KeyCode::F(5)) {
                    app.cycle_delimiter();
                } else if is_key(&key_event, KeyCode::F(6)) {
                    handle_sort_asc(&mut app)?;
                } else if is_key(&key_event, KeyCode::F(7)) {
                    handle_sort_desc(&mut app)?;
                } else if is_key(&key_event, KeyCode::F(8)) {
                    handle_trim_dedup(&mut app)?;
                } else if is_key(&key_event, KeyCode::F(10)) {
                    if app.active_tab == 2 {
                        app.cycle_convert_source_delimiter();
                        app.results = vec![format!(
                            "Source delimiter: {}",
                            app.convert_source_delimiter.display_name()
                        )];
                    }
                } else if is_key(&key_event, KeyCode::F(11)) {
                    if app.active_tab == 2 {
                        app.cycle_convert_target_delimiter();
                        app.results = vec![format!(
                            "Target delimiter: {}",
                            app.convert_target_delimiter.display_name()
                        )];
                    }
                } else if is_key(&key_event, KeyCode::F(12)) {
                    if app.active_tab == 0 {
                        handle_compare_operations(&mut app)?;
                    } else if app.active_tab == 1 {
                        app.toggle_diff_view();
                        let mode = if app.diff_view_mode == 1 {
                            "Unified View"
                        } else {
                            "Grid View"
                        };
                        app.results = vec![format!("Diff mode: {}", mode)];
                    } else if app.active_tab == 2 {
                        handle_convert_operation(&mut app)?;
                    }
                } else if is_copy_paste_key(&key_event, KeyCode::Char('v')) {
                    // Paste from clipboard
                    if app.active_tab == 0 || (app.active_tab == 2 && app.active_panel == 0) {
                        match crate::clipboard::get_from_clipboard(app.clipboard.as_mut()) {
                            Ok(text) => {
                                if let Some(textarea) = app.active_textarea() {
                                    textarea.insert_str(&text);
                                }
                            }
                            Err(e) => {
                                app.results = vec![format!("Error pasting: {}", e)];
                            }
                        }
                    }
                } else if is_copy_paste_key(&key_event, KeyCode::Char('c')) {
                    // Copy active panel to clipboard (Ctrl+C on Linux, Cmd+C on macOS)
                    let (text, panel_name) = active_panel_content(&app);
                    match crate::clipboard::copy_to_clipboard(app.clipboard.as_mut(), &text) {
                        Ok(_) => {
                            if app.active_tab == 0 && app.active_panel != 2 {
                                app.results = vec![format!("Copied {} to clipboard", panel_name)];
                            }
                        }
                        Err(e) => {
                            app.results = vec![format!("Error copying: {}", e)];
                        }
                    }
                } else {
                    // Pass other keys to the active textarea (Tab 1 and converter input)
                    if app.active_tab == 0 || (app.active_tab == 2 && app.active_panel == 0) {
                        if let Some(textarea) = app.active_textarea() {
                            let input = Input::from(key_event);
                            textarea.input(input);
                        }
                    }
                }
            }
            InputEvent::Mouse(mouse_event) => {
                // Handle mouse events for textarea (only in Tab 1)
                if app.active_tab == 0 {
                    if let Some(textarea) = app.active_textarea() {
                        let input = Input::from(mouse_event);
                        textarea.input(input);
                    }
                }
            }
            InputEvent::Resize(_, _) => {
                // Terminal was resized, will be handled in next draw
            }
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// Handle trim and dedup operation - replaces panel content
fn handle_trim_dedup(app: &mut App) -> Result<(), io::Error> {
    if app.active_tab != 0 {
        return Ok(());
    }

    let delimiter = app.delimiter;
    let Some(textarea) = app.active_textarea() else {
        app.results = vec!["Please select List 1 or List 2".to_string()];
        return Ok(());
    };

    let active_text = join_lines_with_delimiter(textarea.lines(), delimiter);
    let items = parse_list(&active_text, delimiter);

    if items.is_empty() {
        app.results = vec!["No items to process".to_string()];
        return Ok(());
    }

    // Count BEFORE processing to show original stats
    let original_total = items.len();
    let original_unique = items.iter().collect::<std::collections::HashSet<_>>().len();

    // Apply trim and dedup (no sorting)
    let result = process_single_list(&items, true, true, false, false);

    // Replace panel content with processed items
    let new_content: Vec<String> = result.items.clone();
    textarea.select_all();
    textarea.cut();
    textarea.insert_str(&new_content.join("\n"));

    // Show stats in results
    app.results = vec![format!(
        "Trim & Dedup: {} → {} items",
        original_total, original_unique
    )];

    Ok(())
}

/// Handle sort ascending operation - replaces panel content
fn handle_sort_asc(app: &mut App) -> Result<(), io::Error> {
    if app.active_tab != 0 {
        return Ok(());
    }

    let delimiter = app.delimiter;
    let Some(textarea) = app.active_textarea() else {
        app.results = vec!["Please select List 1 or List 2".to_string()];
        return Ok(());
    };

    let active_text = join_lines_with_delimiter(textarea.lines(), delimiter);
    let items = parse_list(&active_text, delimiter);

    if items.is_empty() {
        app.results = vec!["No items to sort".to_string()];
        return Ok(());
    }

    // Apply sort ascending (no trim, no dedup)
    let result = process_single_list(&items, false, false, true, false);

    // Replace panel content with sorted items
    let new_content: Vec<String> = result.items.clone();
    textarea.select_all();
    textarea.cut();
    textarea.insert_str(&new_content.join("\n"));

    // Show stats in results
    app.results = vec![format!("Sorted ↑ {} items", items.len())];

    Ok(())
}

/// Handle sort descending operation - replaces panel content
fn handle_sort_desc(app: &mut App) -> Result<(), io::Error> {
    if app.active_tab != 0 {
        return Ok(());
    }

    let delimiter = app.delimiter;
    let Some(textarea) = app.active_textarea() else {
        app.results = vec!["Please select List 1 or List 2".to_string()];
        return Ok(());
    };

    let active_text = join_lines_with_delimiter(textarea.lines(), delimiter);
    let items = parse_list(&active_text, delimiter);

    if items.is_empty() {
        app.results = vec!["No items to sort".to_string()];
        return Ok(());
    }

    // Apply sort descending (no trim, no dedup)
    let result = process_single_list(&items, false, false, false, true);

    // Replace panel content with sorted items
    let new_content: Vec<String> = result.items.clone();
    textarea.select_all();
    textarea.cut();
    textarea.insert_str(&new_content.join("\n"));

    // Show stats in results
    app.results = vec![format!("Sorted ↓ {} items", items.len())];

    Ok(())
}

/// Handle compare operations
fn handle_compare_operations(app: &mut App) -> Result<(), io::Error> {
    let list1_text = join_lines_with_delimiter(app.list1.lines(), app.delimiter);
    let list2_text = join_lines_with_delimiter(app.list2.lines(), app.delimiter);

    let list1_items = parse_list(&list1_text, app.delimiter);
    let list2_items = parse_list(&list2_text, app.delimiter);

    if list1_items.is_empty() && list2_items.is_empty() {
        app.results = vec!["Both lists are empty".to_string()];
        return Ok(());
    }

    // Use current options (case sensitivity / trim) selected by the user
    let result = compare_lists(&list1_items, &list2_items, app.compare_options);

    // Store detailed results for Tab 2
    app.compare_results = Some(result.clone());

    // Format summary results for Tab 1 (2 lines max)
    let summary = format!(
        "Only L1: {} | Only L2: {} | Inter: {} | Union: {}",
        result.only_in_first.len(),
        result.only_in_second.len(),
        result.intersection.len(),
        result.union.len()
    );
    app.results = vec![
        summary,
        "Compare complete. Details available in Results tab.".to_string(),
    ];

    // Switch to Results tab
    app.go_to_tab(1);

    Ok(())
}

/// Convert input in the Convert tab using selected source/target delimiters.
/// The source delimiter is applied to parse the input; the target delimiter is used to render and save the output.
fn handle_convert_operation(app: &mut App) -> Result<(), io::Error> {
    if app.active_tab != 2 {
        return Ok(());
    }

    let source_text = if app.convert_source_delimiter == Delimiter::Json {
        // For JSON, join all lines with newline to preserve structure
        app.convert_input.lines().join("\n")
    } else {
        join_lines_with_delimiter(app.convert_input.lines(), app.convert_source_delimiter)
    };

    let (items, _repaired_json) = if app.convert_source_delimiter == Delimiter::Json {
        match crate::parser::parse_json_to_list(
            &source_text,
            app.convert_target_delimiter.as_char(),
        ) {
            Ok((list, repaired)) => {
                // Update the input area with the (possibly repaired) JSON
                // so the user can see the quotes if they were added
                app.convert_input =
                    tui_textarea::TextArea::from(repaired.lines().map(String::from));
                (list, repaired)
            }
            Err(e) => {
                app.results = vec![format!("JSON Error: {}", e)];
                app.convert_output_items.clear();
                app.convert_output_serialized.clear();
                return Ok(());
            }
        }
    } else {
        (
            parse_list(&source_text, app.convert_source_delimiter),
            source_text,
        )
    };

    if items.is_empty() {
        app.results = vec!["Nothing to convert".to_string()];
        app.convert_output_items.clear();
        app.convert_output_serialized.clear();
        return Ok(());
    }

    // Special handling for JSON source: it already formatted CSV rows if needed
    if app.convert_source_delimiter == Delimiter::Json {
        app.convert_output_serialized = items.join("\n");
        app.convert_output_items = items.clone();
    } else {
        let target_sep = app.convert_target_delimiter.as_char().to_string();
        app.convert_output_serialized = items.join(&target_sep);
        app.convert_output_items = if app.convert_target_delimiter == Delimiter::Newline {
            items.clone()
        } else {
            vec![app.convert_output_serialized.clone()]
        };
    }

    app.active_panel = 1; // focus output
    app.results = vec![format!(
        "Converted {} item(s) to {}",
        items.len(),
        app.convert_target_delimiter.display_name()
    )];

    Ok(())
}

fn active_panel_label(app: &App) -> Option<String> {
    if app.active_tab == 0 {
        let label = match app.active_panel {
            0 => "List 1",
            1 => "List 2",
            _ => "Results",
        };
        return Some(label.to_string());
    }

    if app.active_tab == 2 {
        let label = match app.active_panel {
            0 => format!(
                "Convert Input (Src {})",
                app.convert_source_delimiter.display_name()
            ),
            1 => format!(
                "Convert Output (Dst {}) [{} items]",
                app.convert_target_delimiter.display_name(),
                app.convert_output_items.len()
            ),
            _ => "Convert".to_string(),
        };
        return Some(label);
    }

    if let Some(ref compare_results) = app.compare_results {
        let (label, count) = match app.active_panel {
            0 => ("Only in List 1", compare_results.only_in_first.len()),
            1 => ("Only in List 2", compare_results.only_in_second.len()),
            2 => ("Intersection", compare_results.intersection.len()),
            _ => ("Union", compare_results.union.len()),
        };
        Some(format!("{} ({} items)", label, count))
    } else {
        let label = match app.active_panel {
            0 => "Only in List 1",
            1 => "Only in List 2",
            2 => "Intersection",
            _ => "Union",
        };
        Some(format!("{} (0 items)", label))
    }
}

/// Join lines using the given delimiter so parsing respects the selected separator.
fn join_lines_with_delimiter(lines: &[String], delimiter: Delimiter) -> String {
    let sep = delimiter.as_char().to_string();
    lines.join(&sep)
}

/// Extract the current panel content and a friendly name
fn active_panel_content(app: &App) -> (String, String) {
    if app.active_tab == 0 {
        match app.active_panel {
            0 => (
                join_lines_with_delimiter(app.list1.lines(), app.delimiter),
                "List 1".to_string(),
            ),
            1 => (
                join_lines_with_delimiter(app.list2.lines(), app.delimiter),
                "List 2".to_string(),
            ),
            _ => (app.results.join("\n"), "Results".to_string()),
        }
    } else if app.active_tab == 2 {
        match app.active_panel {
            0 => (
                join_lines_with_delimiter(app.convert_input.lines(), app.convert_source_delimiter),
                "Convert Input".to_string(),
            ),
            1 => (
                app.convert_output_serialized.clone(),
                "Convert Output".to_string(),
            ),
            _ => ("".to_string(), "Results".to_string()),
        }
    } else if let Some(ref compare_results) = app.compare_results {
        let (items, name) = match app.active_panel {
            0 => (&compare_results.only_in_first, "Only in List 1"),
            1 => (&compare_results.only_in_second, "Only in List 2"),
            2 => (&compare_results.intersection, "Intersection"),
            _ => (&compare_results.union, "Union"),
        };
        (items.join("\n"), name.to_string())
    } else {
        ("".to_string(), "Results".to_string())
    }
}

/// Resolve a default file path for the active panel, allowing a base directory override
fn file_path_for_panel(app: &App) -> Option<PathBuf> {
    let base_dir = env::var("LIST_UTILS_DIR").unwrap_or_else(|_| ".".to_string());

    let filename = match app.active_tab {
        0 => match app.active_panel {
            0 => Some("list1.txt"),
            1 => Some("list2.txt"),
            2 => Some("results.txt"),
            _ => None,
        },
        1 => match app.active_panel {
            0 => Some("only_in_list1.txt"),
            1 => Some("only_in_list2.txt"),
            2 => Some("intersection.txt"),
            3 => Some("union.txt"),
            _ => None,
        },
        2 => match app.active_panel {
            0 => Some("convert_input.txt"),
            1 => Some("convert_output.txt"),
            _ => None,
        },
        _ => None,
    }?;

    Some(PathBuf::from(base_dir).join(filename))
}

/// Pick content to persist based on active panel and delimiter rules
fn content_for_save(app: &App) -> (String, String) {
    if app.active_tab == 0 {
        match app.active_panel {
            0 => (
                join_lines_with_delimiter(app.list1.lines(), app.delimiter),
                "List 1".to_string(),
            ),
            1 => (
                join_lines_with_delimiter(app.list2.lines(), app.delimiter),
                "List 2".to_string(),
            ),
            _ => (app.results.join("\n"), "Results".to_string()),
        }
    } else if app.active_tab == 2 {
        match app.active_panel {
            0 => (
                join_lines_with_delimiter(app.convert_input.lines(), app.convert_source_delimiter),
                "Convert Input".to_string(),
            ),
            1 => (
                app.convert_output_serialized.clone(),
                "Convert Output".to_string(),
            ),
            _ => ("".to_string(), "Results".to_string()),
        }
    } else if let Some(ref compare_results) = app.compare_results {
        let (items, name) = match app.active_panel {
            0 => (&compare_results.only_in_first, "Only in List 1"),
            1 => (&compare_results.only_in_second, "Only in List 2"),
            2 => (&compare_results.intersection, "Intersection"),
            _ => (&compare_results.union, "Union"),
        };
        (items.join("\n"), name.to_string())
    } else {
        ("".to_string(), "Results".to_string())
    }
}

/// Load content from a file into the active editable panel (List 1 or List 2)
fn handle_load_from_file(app: &mut App) -> Result<(), io::Error> {
    if !((app.active_tab == 0 && (app.active_panel == 0 || app.active_panel == 1))
        || (app.active_tab == 2 && app.active_panel == 0))
    {
        app.results = vec!["Select a loadable panel (List 1/2 or Convert Input)".to_string()];
        return Ok(());
    }

    let Some(path) = file_path_for_panel(app) else {
        app.results = vec!["No target file for this panel".to_string()];
        return Ok(());
    };

    match fs::read_to_string(&path) {
        Ok(content) => {
            let delimiter = if app.active_tab == 2 {
                app.convert_source_delimiter
            } else {
                app.delimiter
            };
            let items = parse_list(&content, delimiter);
            let Some(textarea) = app.active_textarea() else {
                app.results = vec!["No active panel".to_string()];
                return Ok(());
            };
            textarea.select_all();
            textarea.cut();
            textarea.insert_str(&items.join("\n"));

            let count = items.len();
            app.results = vec![format!("Loaded {} item(s) from {}", count, path.display())];
            if app.active_tab == 2 {
                app.convert_output_items.clear();
                app.convert_output_serialized.clear();
            }
        }
        Err(err) => {
            app.results = vec![format!("Failed to load {}: {}", path.display(), err)];
        }
    }

    Ok(())
}

/// Save the active panel content to a file
fn handle_save_to_file(app: &mut App) -> Result<(), io::Error> {
    let Some(path) = file_path_for_panel(app) else {
        app.results = vec!["No target file for this panel".to_string()];
        return Ok(());
    };

    let (text, panel_name) = content_for_save(app);
    if text.is_empty() {
        app.results = vec![format!("Nothing to save from {}", panel_name)];
        return Ok(());
    }

    match fs::write(&path, text) {
        Ok(_) => {
            app.results = vec![format!("Saved {} to {}", panel_name, path.display())];
        }
        Err(err) => {
            app.results = vec![format!("Failed to save {}: {}", path.display(), err)];
        }
    }

    Ok(())
}
