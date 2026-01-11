/// Application state and main event loop supporting three tabs:
/// Input (lists + summary), Results (diff panels), and Convert (delimiter conversion).
use crate::operations::{CompareOptions, CompareResult};
use crate::parser::Delimiter;
use arboard::Clipboard;
use tui_textarea::TextArea;

/// Main application state
pub struct App {
    /// First list text area
    pub list1: TextArea<'static>,
    /// Second list text area
    pub list2: TextArea<'static>,
    /// Converter input text area (Tab 3)
    pub convert_input: TextArea<'static>,
    /// Converter output items (displayed as lines)
    pub convert_output_items: Vec<String>,
    /// Serialized converter output with target delimiter (for saving)
    pub convert_output_serialized: String,
    /// Currently selected delimiter
    pub delimiter: Delimiter,
    /// Converter source delimiter (Tab 3)
    pub convert_source_delimiter: Delimiter,
    /// Converter target delimiter (Tab 3)
    pub convert_target_delimiter: Delimiter,
    /// Options that control list comparison
    pub compare_options: CompareOptions,
    /// Currently active tab (0 = Input, 1 = Results)
    pub active_tab: usize,
    /// Currently active panel (relative to tab: Tab1: 0-2, Tab2: 0-3)
    pub active_panel: usize,
    /// Results text to display (summary for Tab 1)
    pub results: Vec<String>,
    /// Detailed compare results for Tab 2
    pub compare_results: Option<CompareResult>,
    /// Whether the application should exit
    pub should_quit: bool,
    /// Whether the help modal is being displayed
    pub show_help: bool,
    /// View mode for the results tab (0 = Grid, 1 = Unified Diff)
    pub diff_view_mode: usize,
    /// Clipboard instance for persistent selection on Linux
    pub clipboard: Option<Clipboard>,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        Self {
            list1: TextArea::default(),
            list2: TextArea::default(),
            convert_input: TextArea::default(),
            convert_output_items: Vec::new(),
            convert_output_serialized: String::new(),
            delimiter: Delimiter::Newline,
            convert_source_delimiter: Delimiter::Newline,
            convert_target_delimiter: Delimiter::Comma,
            compare_options: CompareOptions::default(),
            active_tab: 0,
            active_panel: 0,
            results: vec![
                "Welcome to List Utils! Press ? for help.".to_string(),
                "Ready to process lists.".to_string(),
            ],
            compare_results: None,
            should_quit: false,
            show_help: false,
            diff_view_mode: 0,
            clipboard: Clipboard::new().ok(),
        }
    }

    /// Get the currently active text area (only for editable panels)
    pub fn active_textarea(&mut self) -> Option<&mut TextArea<'static>> {
        match (self.active_tab, self.active_panel) {
            (0, 0) => Some(&mut self.list1),
            (0, 1) => Some(&mut self.list2),
            (2, 0) => Some(&mut self.convert_input),
            _ => None,
        }
    }

    /// Switch to the next panel within the current tab
    pub fn switch_panel(&mut self) {
        self.active_panel = match self.active_tab {
            0 => (self.active_panel + 1) % 3, // Tab 1: list1 -> list2 -> results
            1 => (self.active_panel + 1) % 4, // Tab 2: Only L1 -> Only L2 -> Intersection -> Union
            2 => (self.active_panel + 1) % 2, // Tab 3: Converter input -> output
            _ => 0,
        };
    }

    /// Go to a specific tab
    pub fn go_to_tab(&mut self, tab: usize) {
        if tab < 3 {
            self.active_tab = tab;
            self.active_panel = 0; // Reset to first panel in new tab
        }
    }

    /// Cycle to the next delimiter
    pub fn cycle_delimiter(&mut self) {
        self.delimiter = self.delimiter.next();
    }

    /// Cycle converter source delimiter
    pub fn cycle_convert_source_delimiter(&mut self) {
        self.convert_source_delimiter = self.convert_source_delimiter.next();
    }

    /// Cycle converter target delimiter
    pub fn cycle_convert_target_delimiter(&mut self) {
        self.convert_target_delimiter = self.convert_target_delimiter.next();
    }

    /// Toggle case sensitivity for comparisons
    pub fn toggle_case_sensitivity(&mut self) {
        self.compare_options.case_sensitive = !self.compare_options.case_sensitive;
    }

    /// Toggle trimming behavior for comparisons
    pub fn toggle_trim_spaces(&mut self) {
        self.compare_options.trim_spaces = !self.compare_options.trim_spaces;
    }

    /// Toggle help modal visibility
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Toggle between different result view modes
    pub fn toggle_diff_view(&mut self) {
        self.diff_view_mode = (self.diff_view_mode + 1) % 2;
    }
}
