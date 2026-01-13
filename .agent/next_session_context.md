# Next Session Context: list-utils

## Current Status
The application has been significantly enhanced with JSON-to-CSV conversion, a new Unified Diff view, and a more robust layout. All major function keys have been reorganized for better UX.

## Accomplishments (Recent Session)
1.  **JSON to CSV Conversion**:
    *   Implemented `parse_json_to_list` with support for array of objects or single objects.
    *   Added **Lax JSON Repair**: Automatically adds quotes to unquoted keys during conversion.
    *   Input area is automatically updated with the "repaired" JSON.
2.  **Unified Diff View**:
    *   Added a git-style diff view to the Results tab (toggle with `F12`).
    *   Colors: Green for additions, Red for removals, Gray for common items.
3.  **UI/UX Improvements**:
    *   **Vim Mode Implementation**:
        *   Added **NORMAL**/**INSERT** modes.
        *   Movement: `hjkl`, `wb`, `0$`, `gG` supported in Normal mode.
        *   Switch with `i` (Insert) and `Esc` (Normal).
    *   **Quit Shortcut**: Now only `Esc` in Normal mode exits the app.
    *   **INFO Panel**: Added a 4-line fixed panel at the bottom displaying context-aware hints.
    *   **Improved Visibility**: White text for editors ensures quotes and special characters are clearly visible.
4.  **Stability**:
    *   Fixed "clipboard dropped quickly" error on Linux by making the `Clipboard` instance persistent in the `App` struct.

## Suggested Next Steps
- Implement **Export to JSON**: Allow converting a flat list or CSV back into a JSON array.
- Add **Filter/Search**: A way to search within the current list or filtered results.
- **Multiselect in Results**: Ability to select multiple result items to copy specifically.

## References
- `src/parser/mod.rs`: Contains the JSON repair and parse logic.
- `src/ui/results_panel.rs`: Contains the Unified Diff rendering logic.
- `src/main.rs`: Keyboard event handling for F10-F12 orchestration.
