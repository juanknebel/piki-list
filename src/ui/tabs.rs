/// Tabs component for navigation between Input and Results views
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Tabs},
    Frame,
};

/// Render the tabs bar
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `active_tab` - Currently active tab index (0 = Input, 1 = Results)
pub fn render_tabs(frame: &mut Frame, area: ratatui::layout::Rect, active_tab: usize) {
    let titles = vec![" Input ", " Results ", " Convert "];

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(active_tab)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(ratatui::style::Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}
