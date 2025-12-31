# zrd

A fast, minimal text editor for the terminal. That's it.

## Install

```bash
cargo install zrd
```

## Use

```bash
zrd myfile.txt      # Edit a file
zrd                 # Edit default scratch file (~/.config/zrd/default.txt)
```

Press `Esc` to quit. Changes save automatically.

## Why zrd?

- **Fast** - Opens instantly, no lag
- **Minimal** - Does one thing: edit text
- **Familiar** - Standard keyboard shortcuts (Ctrl+Z, Ctrl+A, etc.)
- **Mouse support** - Click to position, drag to select, scroll wheel works
- **Auto-save** - Never lose work

## Keyboard Shortcuts

| Action | Keys |
|--------|------|
| Quit | `Esc` or `Ctrl+W` |
| Undo / Redo | `Ctrl+Z` / `Ctrl+Shift+Z` |
| Select all | `Ctrl+A` |
| Start/End of line | `Home` / `End` or `Cmd+←/→` |
| Word left/right | `Alt+←/→` |
| Delete word | `Alt+Backspace` / `Alt+Delete` |
| Delete line | `Ctrl+Shift+K` |
| Move line up/down | `Alt+↑/↓` |

Full list: [FEATURES.md](FEATURES.md)

## Mouse

- **Click** - Position cursor
- **Drag** - Select text
- **Scroll** - Navigate document

## GUI Version (macOS)

There's also a native GUI version using GPUI:

```bash
cargo install zrd-gui
zrd-gui myfile.txt
```

Both share the same editing engine and can edit the same file simultaneously with live sync.

## Terminal Setup

If `Alt+arrow` keys don't work:

- **Terminal.app**: Preferences → Profiles → Keyboard → "Use Option as Meta key"
- **iTerm2**: Preferences → Profiles → Keys → Left Option → "Esc+"

## Build from Source

```bash
git clone https://github.com/douglance/zrd.git
cd zrd
cargo install --path zrd
```

## License

MIT
