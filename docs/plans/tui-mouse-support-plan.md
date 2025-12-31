# TUI Mouse Support Implementation Plan

## Overview

Add mouse support to the TUI (ratatui/crossterm) editor enabling click-to-position cursor, drag-to-select text, and scroll wheel navigation. This complements the recently added scroll support and provides a complete pointing device experience.

## Success Criteria

- [ ] Single-click positions cursor at clicked location
- [ ] Click outside text area is handled gracefully (cursor moves to nearest valid position)
- [ ] Drag with left button creates text selection
- [ ] Scroll wheel moves viewport up/down
- [ ] Mouse capture is properly enabled on startup and disabled on exit
- [ ] All existing keyboard functionality remains unchanged
- [ ] Coordinate translation correctly accounts for UI padding and scroll offset

## Technical Approach

The implementation uses crossterm's built-in mouse support. Mouse events are captured via `EnableMouseCapture`/`DisableMouseCapture` and handled alongside keyboard events in the existing event loop. A new `EditorAction::SetCursorPosition` variant allows the TUI to communicate mouse-derived positions to the core engine.

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Mouse event granularity | Handle Down/Drag/Scroll only | Up events not needed; Move would be noisy |
| Scroll amount | 3 lines per wheel tick | Standard convention, matches most editors |
| Selection model | Drag initiates selection from click point | Matches native text editors |
| Core engine changes | Minimal - single new action + method | Keep platform-agnostic core clean |
| Coordinate validation | Clamp to valid document bounds | Prevents crashes on edge clicks |

## Implementation Phases

### Phase 1: Enable Mouse Capture (estimated: 15min)

**Goal**: Terminal captures mouse events without breaking existing functionality.

**Pre-conditions**:
- Existing keyboard handling works correctly

**Steps**:

1. [ ] Add mouse event imports to `/Users/douglance/Developer/lv/dright/zlyph-tui/src/main.rs`:
   ```rust
   use crossterm::{
       event::{self, poll, Event, KeyCode, KeyEvent, KeyModifiers,
               MouseEvent, MouseEventKind, MouseButton, EnableMouseCapture, DisableMouseCapture},
       // ... rest unchanged
   };
   ```

2. [ ] Enable mouse capture in `TuiEditor::run()` method (around line 108):
   ```rust
   execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
   ```

3. [ ] Disable mouse capture in cleanup (around line 116):
   ```rust
   execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
   ```

**Verification**:
```bash
cargo build -p zlyph-tui && cargo run -p zlyph-tui
# Verify: Editor starts/stops normally, keyboard still works
```

**Rollback**:
Remove `EnableMouseCapture`/`DisableMouseCapture` from execute calls.

---

### Phase 2: Add Core Engine Support for Cursor Positioning (estimated: 20min)

**Goal**: Core engine can set cursor to arbitrary valid position.

**Pre-conditions**:
- Phase 1 complete

**Steps**:

1. [ ] Add new action variant to `/Users/douglance/Developer/lv/dright/zlyph-core/src/actions.rs`:
   ```rust
   pub enum EditorAction {
       // ... existing variants ...

       // Mouse-driven cursor positioning
       SetCursorPosition { row: usize, column: usize },
       StartSelection { row: usize, column: usize },
       ExtendSelection { row: usize, column: usize },
   }
   ```

2. [ ] Add handler methods to `/Users/douglance/Developer/lv/dright/zlyph-core/src/engine.rs`:
   ```rust
   impl EditorEngine {
       // Add after existing movement methods (around line 340)

       /// Set cursor to specific position, clamping to valid bounds
       fn set_cursor_position(&mut self, row: usize, column: usize) {
           self.clear_selection();
           let row = row.min(self.state.lines.len().saturating_sub(1));
           let column = column.min(self.state.lines[row].len());
           self.state.cursor = BufferPosition::new(row, column);
       }

       /// Start a new selection at position
       fn start_selection(&mut self, row: usize, column: usize) {
           let row = row.min(self.state.lines.len().saturating_sub(1));
           let column = column.min(self.state.lines[row].len());
           self.state.cursor = BufferPosition::new(row, column);
           self.state.selection_anchor = Some(self.state.cursor);
       }

       /// Extend selection to position
       fn extend_selection(&mut self, row: usize, column: usize) {
           if self.state.selection_anchor.is_none() {
               self.state.selection_anchor = Some(self.state.cursor);
           }
           let row = row.min(self.state.lines.len().saturating_sub(1));
           let column = column.min(self.state.lines[row].len());
           self.state.cursor = BufferPosition::new(row, column);
       }
   }
   ```

3. [ ] Add action dispatch in `handle_action()` method (around line 100):
   ```rust
   EditorAction::SetCursorPosition { row, column } => self.set_cursor_position(row, column),
   EditorAction::StartSelection { row, column } => self.start_selection(row, column),
   EditorAction::ExtendSelection { row, column } => self.extend_selection(row, column),
   ```

**Verification**:
```bash
cargo check -p zlyph-core
cargo test -p zlyph-core  # If tests exist
```

**Rollback**:
Remove the three new action variants and their handlers.

---

### Phase 3: Coordinate Translation (estimated: 30min)

**Goal**: Convert screen coordinates to valid document positions.

**Pre-conditions**:
- Phase 2 complete

**Steps**:

1. [ ] Add coordinate translation method to `TuiEditor` in `/Users/douglance/Developer/lv/dright/zlyph-tui/src/main.rs`:
   ```rust
   impl TuiEditor {
       /// Convert screen coordinates to document position
       /// Returns None if click is outside the text area
       fn screen_to_document(&self, screen_col: u16, screen_row: u16, area: Rect) -> Option<(usize, usize)> {
           // Calculate padded area boundaries (matches render() logic)
           let text_x_start = area.x + 2;
           let text_y_start = area.y + 1;
           let text_x_end = area.x + area.width.saturating_sub(2);
           let text_y_end = area.y + area.height.saturating_sub(1);

           // Check if click is within text area
           if screen_col < text_x_start || screen_col >= text_x_end {
               return None;
           }
           if screen_row < text_y_start || screen_row >= text_y_end {
               return None;
           }

           // Convert to document coordinates
           let doc_col = (screen_col - text_x_start) as usize;
           let doc_row = (screen_row - text_y_start) as usize + self.scroll_offset as usize;

           Some((doc_row, doc_col))
       }

       /// Clamp document position to valid bounds
       fn clamp_to_document(&self, row: usize, column: usize) -> (usize, usize) {
           let state = self.engine.state();
           let row = row.min(state.lines.len().saturating_sub(1));
           let column = column.min(state.lines[row].len());
           (row, column)
       }
   }
   ```

2. [ ] Store terminal size for coordinate translation. Add field to `TuiEditor`:
   ```rust
   struct TuiEditor {
       // ... existing fields ...
       terminal_size: Rect,  // Add this field
   }
   ```

3. [ ] Initialize in `new()`:
   ```rust
   fn new(file_path: std::path::PathBuf) -> Self {
       // ... existing code ...
       Self {
           engine,
           file_path,
           last_modified,
           scroll_offset: 0,
           terminal_size: Rect::default(),  // Add this
       }
   }
   ```

4. [ ] Update terminal size before rendering in `run_loop()`:
   ```rust
   // Before terminal.draw()
   self.terminal_size = terminal.size()?;
   ```

**Verification**:
```bash
cargo check -p zlyph-tui
```

**Rollback**:
Remove the `screen_to_document`, `clamp_to_document` methods and `terminal_size` field.

---

### Phase 4: Handle Mouse Events (estimated: 45min)

**Goal**: Mouse clicks, drags, and scroll wheel are fully functional.

**Pre-conditions**:
- Phases 1-3 complete

**Steps**:

1. [ ] Add mouse event translation method to `TuiEditor`:
   ```rust
   fn translate_mouse_event(&self, event: MouseEvent) -> Option<EditorAction> {
       match event.kind {
           MouseEventKind::Down(MouseButton::Left) => {
               if let Some((row, col)) = self.screen_to_document(event.column, event.row, self.terminal_size) {
                   let (row, col) = self.clamp_to_document(row, col);
                   Some(EditorAction::StartSelection { row, column: col })
               } else {
                   None
               }
           }
           MouseEventKind::Drag(MouseButton::Left) => {
               if let Some((row, col)) = self.screen_to_document(event.column, event.row, self.terminal_size) {
                   let (row, col) = self.clamp_to_document(row, col);
                   Some(EditorAction::ExtendSelection { row, column: col })
               } else {
                   None
               }
           }
           MouseEventKind::ScrollUp => {
               // Scroll viewport up (content moves down)
               None  // Handled separately - modifies scroll_offset directly
           }
           MouseEventKind::ScrollDown => {
               // Scroll viewport down (content moves up)
               None  // Handled separately - modifies scroll_offset directly
           }
           _ => None,
       }
   }

   fn handle_scroll(&mut self, direction: i16) {
       const SCROLL_LINES: u16 = 3;
       if direction < 0 {
           // Scroll up
           self.scroll_offset = self.scroll_offset.saturating_sub(SCROLL_LINES);
       } else {
           // Scroll down
           let max_scroll = self.engine.state().lines.len().saturating_sub(1) as u16;
           self.scroll_offset = (self.scroll_offset + SCROLL_LINES).min(max_scroll);
       }
   }
   ```

2. [ ] Modify event loop in `run_loop()` to handle mouse events (around line 136):
   ```rust
   if poll(Duration::from_millis(100))? {
       match event::read()? {
           Event::Key(key) => {
               if let Some(action) = self.translate_key_event(key) {
                   if matches!(action, EditorAction::Quit) {
                       let _ = self.engine.save_to_file(&self.file_path);
                       break;
                   }
                   self.engine.handle_action(action);
                   // Auto-save logic...
               }
           }
           Event::Mouse(mouse) => {
               match mouse.kind {
                   MouseEventKind::ScrollUp => {
                       self.handle_scroll(-1);
                   }
                   MouseEventKind::ScrollDown => {
                       self.handle_scroll(1);
                   }
                   _ => {
                       if let Some(action) = self.translate_mouse_event(mouse) {
                           self.engine.handle_action(action);
                           // Auto-save after mouse actions
                           if self.engine.save_to_file(&self.file_path).is_ok() {
                               if let Ok(metadata) = std::fs::metadata(&self.file_path) {
                                   if let Ok(modified) = metadata.modified() {
                                       self.last_modified = Some(modified);
                                   }
                               }
                           }
                       }
                   }
               }
           }
           _ => {}
       }
   }
   ```

**Verification**:
```bash
cargo build -p zlyph-tui && cargo run -p zlyph-tui
# Test: Click to position cursor
# Test: Drag to select text
# Test: Scroll wheel moves viewport
# Test: Click outside text area doesn't crash
```

**Rollback**:
Revert event loop to only handle `Event::Key`, remove mouse handler methods.

---

### Phase 5: Edge Cases and Polish (estimated: 30min)

**Goal**: Handle edge cases gracefully, ensure cursor visibility after mouse actions.

**Pre-conditions**:
- Phase 4 complete and functional

**Steps**:

1. [ ] Ensure cursor visibility after click in `run_loop()`:
   After handling mouse action that moves cursor:
   ```rust
   // After handling mouse click/drag that moves cursor
   let visible_height = self.terminal_size.height.saturating_sub(2);
   self.ensure_cursor_visible(visible_height);
   ```

2. [ ] Handle click-then-type to clear selection:
   This should already work via the existing selection clearing in `type_character()`.
   Verify: click to select, then type - selection should be replaced.

3. [ ] Handle single click (click without drag) to position cursor:
   Modify `StartSelection` to only set anchor but show cursor without visible selection:
   The selection will only become visible when cursor moves to a different position.
   This already works because anchor == cursor means no visual selection.

4. [ ] Verify byte-accurate column positioning:
   The current implementation uses column as byte offset. For ASCII text this works.
   Add a note/TODO for future UTF-8 character-width handling if needed.

5. [ ] Add mouse event debug logging (commented out, like key debug):
   ```rust
   // Debug: Uncomment to see mouse events (redirects to stderr)
   // eprintln!("Mouse: kind={:?}, col={}, row={}", event.kind, event.column, event.row);
   ```

**Verification**:
```bash
cargo build -p zlyph-tui && cargo run -p zlyph-tui
# Test: Click at various positions including:
#   - First character of line
#   - Last character of line
#   - Empty line
#   - Beyond end of line (should snap to end)
#   - Beyond last line (should snap to last line)
# Test: Click and type to replace selection
# Test: Scroll then click to verify coordinates account for scroll offset
```

**Rollback**:
Revert individual edge case changes as needed.

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Terminal doesn't support mouse | Low | Medium | Mouse feature degrades gracefully - keyboard still works |
| Coordinate calculation off by one | Medium | Low | Manual testing with debug logging; visual feedback makes issues obvious |
| UTF-8 column mismatch | Medium | Medium | Document as known limitation; works for ASCII; fix later if needed |
| Performance with rapid mouse events | Low | Low | Event polling already throttled at 100ms |
| Selection anchor not cleared properly | Low | Medium | Existing `clear_selection()` tested with keyboard; reuse same logic |

## Dependencies

- crossterm 0.27 (already in Cargo.toml - provides mouse support)
- ratatui 0.26 (already in Cargo.toml - provides Rect for coordinate calculation)
- No new external dependencies required

## Out of Scope

- Double-click to select word (requires click timing detection)
- Triple-click to select line
- Right-click context menu
- Middle-click paste
- Horizontal scroll (document currently has no horizontal scroll)
- Variable-width character handling (CJK, emoji)
- Mouse hover effects or tooltips
- Resizable split panes

## File Summary

Files to modify:
1. `/Users/douglance/Developer/lv/dright/zlyph-core/src/actions.rs` - Add 3 new action variants
2. `/Users/douglance/Developer/lv/dright/zlyph-core/src/engine.rs` - Add 3 handler methods + dispatch
3. `/Users/douglance/Developer/lv/dright/zlyph-tui/src/main.rs` - Mouse capture, event handling, coordinate translation

Estimated total implementation time: ~2-2.5 hours
