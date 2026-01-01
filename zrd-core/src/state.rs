//! Platform-agnostic editor state

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPosition {
    pub row: usize,
    pub column: usize,
}

impl BufferPosition {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn zero() -> Self {
        Self { row: 0, column: 0 }
    }
}

#[derive(Clone)]
pub struct EditorState {
    /// Lines of text in the buffer
    pub lines: Vec<String>,
    /// Cursor position (row, column in bytes)
    pub cursor: BufferPosition,
    /// Selection anchor for text selection
    pub selection_anchor: Option<BufferPosition>,
    /// Font size (may be ignored by TUI)
    pub font_size: f32,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor: BufferPosition::zero(),
            selection_anchor: None,
            font_size: 14.0,
        }
    }

    pub fn clone_for_undo(&self) -> Self {
        Self {
            lines: self.lines.clone(),
            cursor: self.cursor,
            selection_anchor: self.selection_anchor,
            font_size: self.font_size,
        }
    }

    /// Get the content as a single string
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    /// Create from a string
    pub fn from_string(content: String) -> Self {
        let lines: Vec<String> = if content.is_empty() {
            vec![String::new()]
        } else {
            content.split('\n').map(|s| s.to_string()).collect()
        };

        Self {
            lines,
            cursor: BufferPosition::zero(),
            selection_anchor: None,
            font_size: 14.0,
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line(&self, row: usize) -> Option<&str> {
        self.lines.get(row).map(|s| s.as_str())
    }

    pub fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|s| s.len()).unwrap_or(0)
    }
}
