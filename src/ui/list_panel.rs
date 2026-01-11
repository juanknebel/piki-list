/// List panel component for displaying and editing lists
use ratatui::{
    style::{Color, Style},
    widgets::Block,
    Frame,
};
use tui_textarea::TextArea;

/// Render a list panel with title and text area
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `title` - The title of the panel
/// * `textarea` - The text area widget
/// * `is_active` - Whether this panel is currently active
pub fn render_list_panel<'a>(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    title: &'a str,
    textarea: &mut TextArea<'a>,
    is_active: bool,
) {
    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    // Create block - tui-textarea accepts ratatui::widgets::Block
    let block = Block::default()
        .title(title)
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(border_style);

    textarea.set_block(block);
    textarea.set_style(Style::default().fg(Color::White));
    frame.render_widget(textarea.widget(), area);
}
