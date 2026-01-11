/// Status bar component showing shortcuts and current state
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::parser::Delimiter;

/// Render the status bar at the bottom
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `delimiter` - Current delimiter
/// * `active_tab` - Current tab index
pub fn render_status_bar(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    main_delimiter: Delimiter,
    convert_delimiters: Option<(Delimiter, Delimiter)>,
    active_tab: usize,
    active_panel_info: Option<&str>,
) {
    let copy_label = if cfg!(target_os = "macos") {
        "Cmd+C/V"
    } else {
        "Ctrl+C/V"
    };
    let shortcuts = format!("Alt+1/2/3: Tabs | Tab: Next | {}", copy_label);

    let delim_info = if active_tab == 2 {
        if let Some((src, dst)) = convert_delimiters {
            format!("Src: {} | Dst: {}", src.display_name(), dst.display_name())
        } else {
            "Converter".to_string()
        }
    } else {
        format!("Delim: {}", main_delimiter.display_name())
    };

    let mut spans = vec![
        Span::styled(shortcuts, Style::default().fg(Color::White)),
        Span::raw(" | "),
        Span::styled(delim_info, Style::default().fg(Color::Yellow)),
        Span::raw(" | "),
        Span::styled("?: Help | Esc", Style::default().fg(Color::White)),
    ];

    if let Some(info) = active_panel_info {
        spans.push(Span::raw(" | "));
        spans.push(Span::styled(info, Style::default().fg(Color::Green)));
    }

    let line = Line::from(spans);

    let paragraph = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));

    frame.render_widget(paragraph, area);
}
