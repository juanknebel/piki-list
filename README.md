# List Utils

A terminal-based application for manipulating and comparing lists of elements, built with Rust and ratatui.

## Features

### Single List Operations
- **Save (F1) / Load (F2)**: Persist or hydrate the active panel; defaults to `LIST_UTILS_DIR` or current directory with sensible filenames.
- **Toggle Case (F3)** and **Toggle Trim (F4)**: Control comparison behavior for list comparisons.
- **Cycle Delimiter (F5)**: Switch parsing delimiter for list ops and comparisons.
- **Sort Ascending (F6)** / **Sort Descending (F7)**: Smart sorting (numeric if all numbers, alphabetic otherwise). Replaces panel content.
- **Trim & Dedup (F8)**: Remove whitespace and duplicates in one step. Replaces panel content.
- **Compare lists (F12)**: Find differences and common elements between two lists.
- **Vim Mode & Input Control**: Toggle between **NORMAL** (navigation) and **INSERT** (typing) modes using `i` and `Esc`.
- **Copy to clipboard (Ctrl+C)**: Copy the entire content of the active panel to clipboard.

### Smart Numeric Sorting
When all items in a list are numbers, sorting is done numerically:
- Input: `10, 9, 11, 4`
- Sort ↑: `4, 9, 10, 11` (not alphabetically: `10, 11, 4, 9`)
- Sort ↓: `11, 10, 9, 4`

- **Toggle Case (F3)** and **Toggle Trim (F4)**: Control comparison behavior for list comparisons.
- **Save/Load**: Save any active panel to a file (F1) or load List 1/2 from file (F2). Uses `LIST_UTILS_DIR` as base dir (defaults to current directory) and sensible filenames per panel.
- **Compare (F12)**: Execute comparison and switch to Results tab.
- **Results**:
  - Items only in List 1
  - Items only in List 2
  - Intersection (items in both lists)
  - Union (all unique items from both lists)
  - Each panel shows its item count in the title

### Supported Delimiters (F5 to cycle)
- Newline (`\n`) - default
- Tab (`\t`)
- Comma (`,`)
- Semicolon (`;`)
- **JSON** (Source only): Supports JSON arrays of strings, objects, or even single objects.

### Delimiter Converter Tab (Alt+3, F10)
- Two panels: input (editable/loadable) and output (read-only)
- **JSON to CSV**: If the source delimiter is set to `Json`, the converter generates a CSV based on all unique keys found in the JSON objects.
- **Lax JSON Repair**: Automatically wraps unquoted keys in quotes (e.g., `{id:1}` becomes `{"id":1}`) when converting, making it extremely tolerant.
- **Improved Shortcuts**: Use `F10` to cycle the source delimiter and `F11` for the target delimiter.
- **Convert with F12**.

**Note**: Trailing empty lines are automatically ignored when parsing.

## Installation

### Prerequisites
- Rust 1.82.0 or later
- Cargo

### Build from Source

```bash
git clone <repository-url>
cd list-utils
cargo build --release
```

The binary will be located at `target/release/list-utils`.

## Usage

Run the application:

```bash
cargo run
# or
./target/release/list-utils
```

### Interface Layout

The application uses a tabbed interface with three tabs:

**Tab 1 - Input:**
```
┌─────────────────────┬─────────────────────┐
│       LIST 1        │       LIST 2        │
│  (editable panel)   │  (editable panel)   │
│                     │                     │
├─────────────────────┴─────────────────────┤
│              INFO (hints & results)       │
│  L1: 10 items | L2: 8 items | Result: OK  │
│                                           │
├───────────────────────────────────────────┤
│ ^1/^2/^3: Tabs | Tab | F1: Save | F2: Load | F5: Delim | F12: Compare | ?: Help │
└───────────────────────────────────────────┘
```

**Tab 2 - Results:**
- **Two View Modes**: Toggle between **Grid View** (4 panels) and **Unified Diff View** (git-style) using **F12**.
- **Unified Diff**: Shows additions (+) in green and removals (-) in red.
- **Grid View**:
    - **Only in List 1**: Items unique to the first list.
    - **Only in List 2**: Items unique to the second list.
    - **Intersection**: Items present in both.
    - **Union**: All unique items combined.

**Tab 3 - Convert:**
- Two panels: left input (editable), right output (read-only).
- Keys: `F10` cycle source, `F11` cycle target, `F12` convert.
- Supports **Lax JSON**: Can parse JSON with unquoted keys and automatically repairs the input.
- Layout includes an **INFO** panel at the bottom for quick hints.

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Alt+1/2/3` | Switch between Tabs (Input, Results, Convert) |
| `Tab` | Cycle between panels within current tab |
| `?` | Toggle Help Modal |
| `i` | (Normal Mode) Enter **INSERT mode** |
| `Esc` | (Insert) Return to **Normal Mode** \| (Normal) **Quit** the application |
| `h, j, k, l` | (Normal Mode) Move cursor Left, Down, Up, Right |
| `w, b` | (Normal Mode) Move Word Forward / Back |
| `0, $` | (Normal Mode) Move cursor to Line Start / End |
| `g, G` | (Normal Mode) Move cursor to Top / Bottom of list |
| `Ctrl+C / Ctrl+V` | Copy / Paste (Cross-platform support) |
| `F1` | Save active panel to file |
| `F2` | Load file into active list (List 1, List 2, or Convert Input) |
| `F3 / F4` | Toggle Case-sensitivity / Trim spaces (Comparison) |
| `F5` | Cycle through Delimiters |
| `F6 / F7` | Sort Ascending / Descending |
| `F8` | Trim & Dedup (clean current list) |
| `F10 / F11` | (Convert Tab) Cycle Source / Target Delimiters |
| `F12` | **PRIMARY ACTION**: Compare (Tab 1), Toggle View (Tab 2), Convert (Tab 3) |
| `Esc` | Quit the application |

Default filenames (relative to `LIST_UTILS_DIR` or current directory):
- `list1.txt`, `list2.txt` when saving/loading the input lists
- `results.txt` from the summary panel in Tab 1
- `only_in_list1.txt`, `only_in_list2.txt`, `intersection.txt`, `union.txt` when saving panels in Tab 2
- `convert_input.txt`, `convert_output.txt` when saving panels in Tab 3

### Mouse Support

The application supports mouse interaction:
- Click to position cursor
- Drag to select text
- Scroll to navigate through lists

## Example Workflow

1. **Paste a list** into List 1 using `Ctrl+V`:
   ```
   apple
   banana
   Apple
   cherry
   banana
   ```

2. **Press F8** (Trim & Dedup) - List 1 becomes:
   ```
   apple
   banana
   Apple
   cherry
   ```
   Results show: `Trim & Dedup: 5 → 4 items`

3. **Press F6** (Sort ↑) - List 1 becomes:
   ```
   Apple
   apple
   banana
   cherry
   ```

4. **Paste another list** into List 2, then **press F9** to compare them. This automatically switches to the Results tab.

5. **Convert delimiters**: Go to the Convert tab (`Alt+3`), load or paste content into the input panel, choose source (`[`) and target (`]`) delimiters, then press `F10` to generate the output with the new delimiter. Output shows one line per item if the target delimiter is newline; otherwise it shows the serialized string with the chosen delimiter.

6. In the **Results tab**, you'll see 4 panels:
   - **Only in List 1**: Items unique to the first list
   - **Only in List 2**: Items unique to the second list
   - **Intersection**: Items present in both lists
   - **Union**: All unique items from both lists combined

7. **Press Tab** to navigate between result panels, then **Ctrl+C** to copy any panel's content.

## Architecture

The application is structured into several modules:

- **`app.rs`**: Main application state, tab management, and panel management
- **`parser/`**: List parsing by delimiter with smart trailing line handling
- **`operations/`**: List manipulation operations
  - `single_list.rs`: Trim, dedup, sort (with smart numeric detection)
  - `compare.rs`: Comparison operations between two lists
- **`ui/`**: User interface components
  - `layout.rs`: Main layout management with tabs and results grid
  - `tabs.rs`: Tab navigation component
  - `list_panel.rs`: Editable list panel component
  - `results_panel.rs`: Results display panels (summary and detailed list views)
  - `status_bar.rs`: Status bar with shortcuts
- **`clipboard/`**: Cross-platform clipboard operations
- **`events/`**: Keyboard and mouse event handling

## Development

### Running Tests

```bash
cargo test
```

### Building Documentation

```bash
cargo doc --open
```

## Platform Support

- macOS
- Linux

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
