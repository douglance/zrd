# Testing Selection Highlighting in Zrd

## ✅ What Was Fixed

1. **Visible selection highlighting** - Selected text now has a dark gray background
2. **Cmd+arrow key behavior** - Properly move without selecting
3. **Selection detection** - All selection operations now show visual feedback

## Test Instructions

### Test 1: Basic Selection (Shift+Arrows)

```bash
zrd
```

1. Type: `The quick brown fox jumps over the lazy dog`
2. Press `Home` to go to start
3. Hold `Shift+Right` and arrow through text
4. **Expected**: Text should highlight with **dark gray background** as you select
5. Press `Delete` - selected text should be deleted

### Test 2: Select All (Ctrl+A)

1. Type several lines of text
2. Press `Ctrl+A`
3. **Expected**: All text should have **dark gray background**
4. Press any key - all text should be replaced

### Test 3: Cmd+Left/Right (Line Navigation)

1. Type: `Hello world this is a test line`
2. Press `Cmd+Right`
3. **Expected**: Cursor jumps to END of line (no selection, no highlight)
4. Press `Cmd+Left`
5. **Expected**: Cursor jumps to START of line (no selection, no highlight)

### Test 4: Word Selection

1. Type: `one two three four five`
2. Position cursor at `t` in `two`
3. Hold `Shift+Alt+Right`
4. **Expected**: Selects word `two` with dark gray background
5. Continue holding - should select next word `three`

### Test 5: Multi-Line Selection

1. Type 3 lines:
   ```
   First line
   Second line
   Third line
   ```
2. Go to start of "First"
3. Hold `Shift+Down` twice
4. **Expected**: All three lines highlighted with dark gray background

## Visual Indicators

- **Cursor**: Character under cursor has reversed colors (white on black)
- **Selection**: Dark gray background
- **Both**: When cursor is within selection, you see both effects

## Known Terminal Issues

### Alt+Arrow Keys

If Alt+arrows don't work, you need to configure your terminal:

**Terminal.app:**
- Preferences → Profiles → Keyboard → Check "Use Option as Meta key"

**iTerm2:**
- Preferences → Profiles → Keys → Set "Left Option key" to "Esc+"

### If Still Not Working

Try adding debug output:

1. Edit `/Users/douglance/Developer/lv/dright/zrd/src/main.rs`
2. Find line with `// eprintln!("Key: {:?}, Mods: {:?}", event.code, event.modifiers);`
3. Uncomment it (remove the `//`)
4. Rebuild: `cargo install --path zrd`
5. Run zrd and press keys - you'll see what the terminal sends

Example output:
```
Key: Left, Mods: SUPER           # Cmd+Left (works)
Key: Right, Mods: SUPER          # Cmd+Right (works)
Key: Left, Mods: ALT             # Alt+Left (if configured)
Key: Char('b'), Mods: ALT        # Terminal intercepted Alt+Left
```

If you see `Char('b')` instead of `Left` with `ALT`, your terminal isn't passing Alt+arrows through correctly.

## Expected Behavior Summary

| Action | Visual Result |
|--------|---------------|
| Type text | Cursor moves (reversed char) |
| Shift+Arrow | Dark gray highlight appears |
| Ctrl+A | Everything highlighted dark gray |
| Cmd+Left/Right | Cursor moves, NO highlight |
| Alt+Left/Right | Cursor jumps words (needs terminal config) |
| Cmd+Backspace | Line deleted |

If selections work but you can't see them, you may have a terminal color scheme issue. Try a different terminal profile or color scheme.
