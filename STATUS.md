# Zrd Project Status

## ✅ Completed Features

### Core Engine (zrd-core)
- ✅ Platform-agnostic EditorEngine with EditorAction enum
- ✅ All navigation actions (character, word, line)
- ✅ All selection actions (character, word, line, select all)
- ✅ All delete combinations (8 different Ctrl/Alt/Cmd combinations)
- ✅ Undo/Redo with proper state management
- ✅ Tab/Outdent operations
- ✅ Line operations (move up/down, delete)
- ✅ File I/O (load_from_file, save_to_file)
- ✅ 26/26 tests passing

### TUI (zrd)
- ✅ Full Ratatui implementation
- ✅ Installed globally as `zrd` command
- ✅ Persistent state via `~/.config/zrd/default.txt`
- ✅ Live file reloading (100ms polling)
- ✅ Auto-save on every action
- ✅ Cursor with reverse video highlighting (no extra space)
- ✅ Visible selection highlighting (dark gray background)
- ✅ All keyboard shortcuts implemented
- ✅ Complex multi-line selection rendering
- ✅ Debug mode (commented out by default)

### GPUI (zrd-gui)
- ✅ Basic GPUI implementation
- ✅ Persistent state via `~/.config/zrd/default.txt`
- ✅ Live file reloading (per-frame check)
- ✅ Auto-save on every action
- ✅ All keyboard shortcuts mapped

## ⚠️ Known Issues

### Terminal Configuration Required
**Issue**: Alt+arrow keys may not work without terminal configuration.

**Symptom**: Terminal sends `Char('f')` instead of `Right` with `ALT` modifier.

**Solution**: Configure terminal (see FEATURES.md or KEYBOARD_DEBUG.md)

### Cmd+Arrow Keys
**Status**: Properly configured in code, but may require terminal configuration verification.

**Expected behavior**:
- Cmd+Left → Move to beginning of line (no selection)
- Cmd+Right → Move to end of line (no selection)

## 📝 Technical Details

### File Watching Strategy

**TUI Approach**:
- Polls every 100ms with `poll(Duration::from_millis(100))`
- Checks file modification time
- Reloads if timestamp changed
- Updates last_modified after successful reload

**GPUI Approach**:
- Checks on every render frame
- Compares file modification time
- Reloads if timestamp changed
- Calls `cx.notify()` to trigger re-render

### Auto-Save Strategy
Both interfaces save immediately after every action to ensure synchronization.

### Selection Rendering (TUI)
Complex logic to handle:
1. Cursor on line with selection
2. Multi-line selections
3. Selection start/end on same line
4. Selection spanning multiple lines
5. Cursor at selection boundaries

Renders using:
- `Style::default().bg(Color::DarkGray)` for selection
- `Style::default().add_modifier(Modifier::REVERSED)` for cursor
- Both effects can overlap when cursor is within selection

## 📊 Test Coverage

### zrd-core
- Total: 26 tests
- Status: All passing
- Coverage: Core editing logic

### zrd
- No dedicated tests (manual testing only)

### zrd-gui
- No dedicated tests (manual testing only)

## 🚀 Performance

### Build Configuration
```toml
[profile.dev]
opt-level = 3  # GPUI requires optimization even in dev mode

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
```

## 📂 File Structure

```
zrd/
├── zrd-core/
│   ├── src/
│   │   ├── engine.rs    # 500+ lines, core logic
│   │   ├── actions.rs   # Action enum
│   │   └── lib.rs       # Module exports
│   └── Cargo.toml
├── zrd/
│   ├── src/
│   │   └── main.rs      # 334 lines, TUI implementation
│   └── Cargo.toml
├── zrd-gui/
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── editor.rs    # GPUI editor
│   │   ├── actions.rs   # GPUI actions
│   │   └── theme.rs     # AtomOneDark theme
│   └── Cargo.toml
└── Cargo.toml           # Workspace configuration
```

## 🔧 Development Commands

```bash
# Build everything
cargo build

# Build specific package
cargo build -p zrd-core
cargo build -p zrd
cargo build -p zrd-gui

# Run tests
cargo test -p zrd-core

# Install TUI globally
cargo install --path zrd

# Run GUI
cargo run -p zrd-gui

# Check for errors (faster than build)
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📚 Documentation Files

- `FEATURES.md` - Complete feature list and keyboard shortcuts
- `KEYBOARD_DEBUG.md` - How to debug keyboard issues
- `TEST_SELECTIONS.md` - How to test selection highlighting
- `STATUS.md` - This file (project status and known issues)
- `CLAUDE.md` - Instructions for Claude Code
- `REFACTOR_PLAN.md` - Original refactor plan (mostly complete)

## 🎯 Future Enhancements

Potential additions (not currently planned):
- [ ] Syntax highlighting
- [ ] Search and replace
- [ ] Multiple buffers/files
- [ ] Line numbers
- [ ] Status bar with cursor position
- [ ] Scroll indicators
- [ ] Mouse support (TUI)
- [ ] Copy/paste with system clipboard
- [ ] Configuration file for keybindings
- [ ] Theme customization
