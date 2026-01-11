use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Render a centered help modal
pub fn render_help_modal(frame: &mut Frame) {
    let area = frame.area();

    // Create a centered rectangle for the modal
    let help_area = centered_rect(60, 70, area);

    // Clear the background of the modal area
    frame.render_widget(Clear, help_area);

    let block = Block::default()
        .title(" Help - Keyboard Shortcuts ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Black));

    let mut text = Vec::new();

    // Section: Navigation
    text.push(Line::from(vec![Span::styled(
        "Navigation",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  Alt+1/2/3  ", Style::default().fg(Color::Yellow)),
        Span::raw("Switch between Tabs (Input, Results, Convert)"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  Tab        ", Style::default().fg(Color::Yellow)),
        Span::raw("Switch between panels"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  Esc        ", Style::default().fg(Color::Yellow)),
        Span::raw("Quit application / Close Help"),
    ]));
    text.push(Line::from(""));

    // Section: Data Operations
    text.push(Line::from(vec![Span::styled(
        "Data Operations",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  F5         ", Style::default().fg(Color::Yellow)),
        Span::raw("Cycle global delimiter"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F6 / F7    ", Style::default().fg(Color::Yellow)),
        Span::raw("Sort Ascending / Descending (replaces content)"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F8         ", Style::default().fg(Color::Yellow)),
        Span::raw("Trim spaces & Deduplicate (replaces content)"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F12        ", Style::default().fg(Color::Yellow)),
        Span::raw("Compare List 1 and List 2"),
    ]));
    text.push(Line::from(""));

    // Section: Configuration
    text.push(Line::from(vec![Span::styled(
        "Configuration",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  F3         ", Style::default().fg(Color::Yellow)),
        Span::raw("Toggle Case Sensitivity"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F4         ", Style::default().fg(Color::Yellow)),
        Span::raw("Toggle Trim Spaces"),
    ]));
    text.push(Line::from(""));

    // Section: Files & Clipboard
    text.push(Line::from(vec![Span::styled(
        "Files & Clipboard",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  F1 / F2    ", Style::default().fg(Color::Yellow)),
        Span::raw("Save / Load active panel from file"),
    ]));
    let copy_key = if cfg!(target_os = "macos") {
        "Cmd+C/V"
    } else {
        "Ctrl+C/V"
    };
    text.push(Line::from(vec![
        Span::styled(
            format!("  {}   ", copy_key),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("Copy panel / Paste into input"),
    ]));
    text.push(Line::from(""));

    // Section: Convert Tab
    text.push(Line::from(vec![Span::styled(
        "Convert Tab (Alt+3)",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  F10        ", Style::default().fg(Color::Yellow)),
        Span::raw("Cycle Source Delimiter (JSON support)"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F11        ", Style::default().fg(Color::Yellow)),
        Span::raw("Cycle Target Delimiter"),
    ]));
    text.push(Line::from(vec![
        Span::styled("  F12        ", Style::default().fg(Color::Yellow)),
        Span::raw("Execute delimiter conversion"),
    ]));
    text.push(Line::from(""));

    // Section: Results Tab (Alt+2)
    text.push(Line::from(vec![Span::styled(
        "Results Tab (Alt+2)",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )]));
    text.push(Line::from(vec![
        Span::styled("  F12        ", Style::default().fg(Color::Yellow)),
        Span::raw("Toggle between Grid and Unified Diff view"),
    ]));
    text.push(Line::from(""));

    text.push(Line::from(vec![Span::styled(
        "Press any key or '?' to close",
        Style::default()
            .add_modifier(Modifier::ITALIC)
            .fg(Color::DarkGray),
    )]));

    let paragraph = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, help_area);
}

/// Helper function to create a centered rect using up certain percentage of available area
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
