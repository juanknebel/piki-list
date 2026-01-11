# Project Context: list-utils

## Overview
`list-utils` is a terminal-based UI (TUI) application designed for manipulating, comparing, and converting lists of items. It allows users to input two different lists, perform set operations (intersection, union, differences), and convert between various delimiters (comma, newline, space, etc.).

## Key Features
- **Set Operations**: Compare two lists to find unique items in each, their intersection, and their union.
- **Unified Diff View**: Git-style visualization of list comparisons (toggle with F10 in Results tab).
- **List Processing**: Sort (alphabetical/numeric), trim whitespace, and deduplicate items.
- **Improved Converter**: Convert between delimiters including **JSON to CSV** support.
- **Lax JSON Repair**: Automatically handles unquoted JSON keys during conversion.
- **Help System**: Centered help modal with categorized shortcuts (toggle with ?).
- **Persistent Clipboard**: Stable clipboard handling on Linux to prevent data loss.
- **Case Sensitivity & Trimming**: Configurable comparison options.

## Technology Stack
- **Languages**: Rust
- **UI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui) (successor to tui-rs)
- **Terminal Backend**: [Crossterm](https://github.com/cormacrelph/crossterm)
- **Text Input**: [tui-textarea](https://github.com/rhysd/tui-textarea)
- **Clipboard**: [arboard](https://github.com/hecrj/arboard)

## Project Structure
- `src/main.rs`: Entry point, setup of the terminal, main event loop, and high-level command handling.
- `src/app.rs`: Defines the `App` struct, which holds the global state (active tab, panel, list contents, comparison options).
- `src/operations/`: Contains the logic for processing lists (sorting, deduping) and comparing two lists.
  - `src/operations/mod.rs`: Logic for `process_single_list` and `compare_lists`.
- `src/parser/`: Handles list parsing.
  - `src/parser/mod.rs`: Logic for `parse_list` and the `Delimiter` enum.
- `src/ui/`: Contains TUI rendering components.
  - `src/ui/mod.rs`: Layout and component rendering.
- `src/events/`: Handles keyboard and mouse events.
- `src/clipboard/`: Abstracts clipboard interactions.
- `tests/`: Contains integration tests for core logic (`parser_test.rs`, `operations_test.rs`, `compare_test.rs`).

## Key Shortcuts (within the TUI)
- `Esc`: Quit
- `Alt+1/2/3`: Switch Tabs (Input, Results, Convert)
- `Tab`: Switch between panels
- `?`: Toggle Help Modal
- `F8`: Trim & Dedup
- `F10 / F11`: Cycle Source/Target Delimiters (Convert tab)
- `F12`: Primary Action (Compare / Toggle Diff View / Convert)
- `Ctrl+C / Ctrl+V`: Copy / Paste

## Configuration
- `LIST_UTILS_DIR`: Environment variable used as the base directory for file Save/Load operations (defaults to `.`).

## Development Notes
- The project follows a clean separation between state (`App`), logic (`operations`, `parser`), and presentation (`ui`).
- Tests can be run with `cargo test`.
- Built for the 2021 Rust edition.
