/// Results panel component for displaying operation results
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::operations::CompareResult;
use std::collections::HashSet;

/// Render the results panel (summary view for Tab 1)
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `results` - Vector of result lines to display
/// * `scroll_offset` - Current scroll offset
/// * `is_active` - Whether this panel is currently active
pub fn render_results_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    results: &[String],
    scroll_offset: usize,
    is_active: bool,
) {
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let block = Block::default()
        .title("INFO")
        .borders(Borders::ALL)
        .border_style(border_style);

    let lines: Vec<Line> = results
        .iter()
        .skip(scroll_offset)
        .take(area.height as usize - 2) // Account for borders
        .map(|line| Line::from(Span::raw(line.as_str())))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Render a result list panel (detailed view for Tab 2)
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `title` - Title of the panel
/// * `items` - Vector of items to display (one per line)
/// * `is_active` - Whether this panel is currently active
pub fn render_result_list_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    title: &str,
    items: &[String],
    is_active: bool,
) {
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style);

    let lines: Vec<Line> = items
        .iter()
        .take(area.height as usize - 2) // Account for borders
        .map(|item| Line::from(Span::raw(item.as_str())))
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Render a unified diff view of the comparison results
pub fn render_unified_diff_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    results: &CompareResult,
) {
    let block = Block::default()
        .title(" Unified Diff (- L1, + L2) ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    // Create sets for efficient lookup
    let set_l1: HashSet<&String> = results.only_in_first.iter().collect();
    let set_l2: HashSet<&String> = results.only_in_second.iter().collect();
    let set_inter: HashSet<&String> = results.intersection.iter().collect();

    let mut lines = Vec::new();

    // Iterate through the union to show all items
    // Using union and sorting it ensures a stable, unified list
    let mut all_items = results.union.clone();
    // Re-sorting here to ensure consistent order in unified view
    all_items.sort();

    for item in all_items {
        if set_l1.contains(&item) {
            lines.push(Line::from(vec![
                Span::styled("- ", Style::default().fg(Color::Red)),
                Span::styled(item, Style::default().fg(Color::Red)),
            ]));
        } else if set_l2.contains(&item) {
            lines.push(Line::from(vec![
                Span::styled("+ ", Style::default().fg(Color::Green)),
                Span::styled(item, Style::default().fg(Color::Green)),
            ]));
        } else if set_inter.contains(&item) {
            lines.push(Line::from(vec![
                Span::styled("  ", Style::default().fg(Color::Gray)),
                Span::styled(item, Style::default().fg(Color::Gray)),
            ]));
        } else {
            // This should not happen if union is correct
            lines.push(Line::from(vec![Span::raw("? "), Span::raw(item)]));
        }
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
