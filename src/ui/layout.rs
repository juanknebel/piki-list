/// Main layout component that arranges panels
use ratatui::layout::{Constraint, Layout, Rect};

/// Create the main layout with tabs, three sections: list1, list2, and results
///
/// # Arguments
/// * `area` - The area to divide
///
/// # Returns
/// Tuple of (tabs_area, list1_area, list2_area, results_area, status_area, content_area_for_tab2)
/// content_area_for_tab2 is the combined area for Tab 2 (everything except tabs and status)
pub fn create_layout_with_tabs(area: Rect) -> (Rect, Rect, Rect, Rect, Rect, Rect) {
    let vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs area
            Constraint::Min(10),   // Lists area
            Constraint::Length(4), // INFO area
            Constraint::Length(1), // Status bar at bottom
        ])
        .split(area);

    let tabs_area = vertical[0];
    let lists_area = vertical[1];
    let results_area = vertical[2];
    let status_area = vertical[3];

    // Combined area for Tab 1 (Results) - now just lists_area to leave room for INFO
    let content_area_for_tab2 = lists_area;

    let horizontal = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(lists_area);

    let list1_area = horizontal[0];
    let list2_area = horizontal[1];

    (
        tabs_area,
        list1_area,
        list2_area,
        results_area,
        status_area,
        content_area_for_tab2,
    )
}

/// Create the main layout with three sections: list1, list2, and results (legacy, for Tab 1)
///
/// # Arguments
/// * `area` - The area to divide
///
/// # Returns
/// Tuple of (list1_area, list2_area, results_area, status_area)
#[allow(dead_code)]
pub fn create_layout(area: Rect) -> (Rect, Rect, Rect, Rect) {
    let vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Min(10),    // Lists area
            Constraint::Length(12), // Results area (larger)
            Constraint::Length(1),  // Status bar
        ])
        .split(area);

    let lists_area = vertical[0];
    let results_area = vertical[1];
    let status_area = vertical[2];

    let horizontal = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(lists_area);

    let list1_area = horizontal[0];
    let list2_area = horizontal[1];

    (list1_area, list2_area, results_area, status_area)
}

/// Create a 2x2 grid layout for results panels
///
/// # Arguments
/// * `area` - The area to divide
///
/// # Returns
/// Tuple of (only_l1_area, only_l2_area, intersection_area, union_area)
pub fn create_results_grid(area: Rect) -> (Rect, Rect, Rect, Rect) {
    let vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let top_row = vertical[0];
    let bottom_row = vertical[1];

    let top_horizontal = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_row);

    let bottom_horizontal = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bottom_row);

    (
        top_horizontal[0],    // Only in List 1
        top_horizontal[1],    // Only in List 2
        bottom_horizontal[0], // Intersection
        bottom_horizontal[1], // Union
    )
}
