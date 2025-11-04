use gpui::*;
use std::ops::Range;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualPosition {
    pub visual_row: usize,
    pub column: usize,
}

impl VisualPosition {
    pub fn new(visual_row: usize, column: usize) -> Self {
        Self { visual_row, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapType {
    SoftWrap,
    HardWrap,
    Hyphenated,
}

#[derive(Debug, Clone)]
pub struct VisualLine {
    pub byte_range: Range<usize>,
    pub wrap_type: WrapType,
}

pub struct TextBuffer {
    lines: Vec<String>,
    line_layouts: Vec<Option<CachedLineLayout>>,
}

pub struct CachedLineLayout {
    pub shaped_line: ShapedLine,
    pub font_size: Pixels,
    pub visual_lines: Vec<VisualLine>,
    pub wrap_width: Pixels,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            line_layouts: vec![None],
        }
    }

    pub fn from_string(content: String) -> Self {
        if content.is_empty() {
            return Self::new();
        }

        let lines: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();
        let line_count = lines.len();
        Self {
            lines,
            line_layouts: (0..line_count).map(|_| None).collect(),
        }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
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

    pub fn insert_char(&mut self, pos: BufferPosition, ch: char) {
        if pos.row >= self.lines.len() {
            return;
        }

        if ch == '\n' {
            let line = &self.lines[pos.row];
            let before = line[..pos.column].to_string();
            let after = line[pos.column..].to_string();
            self.lines[pos.row] = before;
            self.lines.insert(pos.row + 1, after);
            self.line_layouts.insert(pos.row + 1, None);
            self.invalidate_layout(pos.row);
        } else {
            self.lines[pos.row].insert(pos.column, ch);
            self.invalidate_layout(pos.row);
        }
    }

    pub fn insert_str(&mut self, pos: BufferPosition, text: &str) {
        if pos.row >= self.lines.len() {
            return;
        }

        if !text.contains('\n') {
            self.lines[pos.row].insert_str(pos.column, text);
            self.invalidate_layout(pos.row);
        } else {
            let new_lines: Vec<&str> = text.split('\n').collect();
            let line = &self.lines[pos.row];
            let before = line[..pos.column].to_string();
            let after = line[pos.column..].to_string();

            self.lines[pos.row] = before + new_lines[0];
            self.invalidate_layout(pos.row);

            for (i, new_line) in new_lines.iter().enumerate().skip(1) {
                if i == new_lines.len() - 1 {
                    self.lines
                        .insert(pos.row + i, new_line.to_string() + &after);
                } else {
                    self.lines.insert(pos.row + i, new_line.to_string());
                }
                self.line_layouts.insert(pos.row + i, None);
            }
        }
    }

    pub fn delete_char(&mut self, pos: BufferPosition) -> bool {
        if pos.row >= self.lines.len() {
            return false;
        }

        let line = &self.lines[pos.row];

        if pos.column >= line.len() {
            if pos.row + 1 < self.lines.len() {
                let next_line = self.lines.remove(pos.row + 1);
                self.lines[pos.row].push_str(&next_line);
                self.line_layouts.remove(pos.row + 1);
                self.invalidate_layout(pos.row);
                return true;
            }
            return false;
        }

        let mut new_line = line.clone();
        new_line.remove(pos.column);
        self.lines[pos.row] = new_line;
        self.invalidate_layout(pos.row);
        true
    }

    pub fn backspace(&mut self, pos: BufferPosition) -> bool {
        if pos.column > 0 {
            let line = &self.lines[pos.row];
            let mut new_pos = pos.column - 1;
            while new_pos > 0 && !line.is_char_boundary(new_pos) {
                new_pos -= 1;
            }

            let mut new_line = line.clone();
            new_line.remove(new_pos);
            self.lines[pos.row] = new_line;
            self.invalidate_layout(pos.row);
            true
        } else if pos.row > 0 {
            let line = self.lines.remove(pos.row);
            self.lines[pos.row - 1].push_str(&line);
            self.line_layouts.remove(pos.row);
            self.invalidate_layout(pos.row - 1);
            true
        } else {
            false
        }
    }

    pub fn delete_range(&mut self, start: BufferPosition, end: BufferPosition) {
        if start == end {
            return;
        }

        let (start, end) =
            if start.row > end.row || (start.row == end.row && start.column > end.column) {
                (end, start)
            } else {
                (start, end)
            };

        if start.row == end.row {
            let line = &self.lines[start.row];
            let before = &line[..start.column];
            let after = &line[end.column..];
            self.lines[start.row] = before.to_string() + after;
            self.invalidate_layout(start.row);
        } else {
            let start_line = &self.lines[start.row][..start.column];
            let end_line = &self.lines[end.row][end.column..];
            self.lines[start.row] = start_line.to_string() + end_line;

            for _ in start.row + 1..=end.row {
                if start.row + 1 < self.lines.len() {
                    self.lines.remove(start.row + 1);
                    self.line_layouts.remove(start.row + 1);
                }
            }
            self.invalidate_layout(start.row);
        }
    }

    pub fn position_to_byte_offset(&self, pos: BufferPosition) -> usize {
        let mut offset = 0;
        for row in 0..pos.row.min(self.lines.len()) {
            offset += self.lines[row].len() + 1;
        }
        if pos.row < self.lines.len() {
            offset += pos.column.min(self.lines[pos.row].len());
        }
        offset
    }

    fn invalidate_layout(&mut self, row: usize) {
        if row < self.line_layouts.len() {
            self.line_layouts[row] = None;
        }
    }

    pub fn invalidate_all_layouts(&mut self) {
        for layout in &mut self.line_layouts {
            *layout = None;
        }
    }

    pub fn get_or_shape_line(
        &mut self,
        row: usize,
        font_size: Pixels,
        wrap_width: Pixels,
        text_system: &WindowTextSystem,
    ) -> Option<&ShapedLine> {
        if row >= self.lines.len() {
            return None;
        }

        let needs_reshaping = self.line_layouts[row].as_ref().map_or(true, |cached| {
            cached.font_size != font_size || cached.wrap_width != wrap_width
        });

        if needs_reshaping {
            let line = &self.lines[row];
            let text = SharedString::from(line.clone());

            let run = TextRun {
                len: line.len(),
                font: Font {
                    family: "Monaco".into(),
                    features: Default::default(),
                    weight: FontWeight::NORMAL,
                    style: FontStyle::Normal,
                    fallbacks: None,
                },
                color: Hsla::default(),
                background_color: None,
                underline: None,
                strikethrough: None,
            };

            let shaped = text_system.shape_line(text, font_size, &[run], None);
            let visual_lines = self.compute_visual_lines(line, &shaped, wrap_width);

            self.line_layouts[row] = Some(CachedLineLayout {
                shaped_line: shaped,
                font_size,
                visual_lines,
                wrap_width,
            });
        }

        self.line_layouts[row]
            .as_ref()
            .map(|cached| &cached.shaped_line)
    }

    fn compute_visual_lines(
        &self,
        line: &str,
        shaped: &ShapedLine,
        wrap_width: Pixels,
    ) -> Vec<VisualLine> {
        if line.is_empty() {
            return vec![VisualLine {
                byte_range: 0..0,
                wrap_type: WrapType::SoftWrap,
            }];
        }

        let mut visual_lines = Vec::new();
        let mut current_start = 0;
        let mut last_word_boundary = None;

        let chars: Vec<(usize, char)> = line.char_indices().collect();

        for i in 0..chars.len() {
            let (byte_idx, ch) = chars[i];
            let next_byte_idx = chars.get(i + 1).map(|(idx, _)| *idx).unwrap_or(line.len());
            let x_pos_absolute = shaped.x_for_index(next_byte_idx);
            let current_start_x = shaped.x_for_index(current_start);
            let x_pos_relative = x_pos_absolute - current_start_x;

            if ch.is_whitespace() {
                last_word_boundary = Some(next_byte_idx);
            }

            if x_pos_relative > wrap_width {
                if let Some(boundary) = last_word_boundary {
                    if boundary > current_start {
                        visual_lines.push(VisualLine {
                            byte_range: current_start..boundary,
                            wrap_type: WrapType::SoftWrap,
                        });
                        current_start = boundary;
                        while current_start < line.len()
                            && line.as_bytes()[current_start].is_ascii_whitespace()
                        {
                            current_start += 1;
                        }
                        last_word_boundary = None;
                        continue;
                    }
                }

                if byte_idx > current_start {
                    let word = &line[current_start..next_byte_idx];
                    if let Some(hyphenated_segments) = self.try_hyphenate_word(
                        word,
                        wrap_width - shaped.x_for_index(current_start),
                        shaped,
                        current_start,
                    ) {
                        for segment in hyphenated_segments {
                            visual_lines.push(segment);
                        }
                        current_start = next_byte_idx;
                    } else {
                        visual_lines.push(VisualLine {
                            byte_range: current_start..byte_idx,
                            wrap_type: WrapType::HardWrap,
                        });
                        current_start = byte_idx;
                    }
                }
                last_word_boundary = None;
            }
        }

        if current_start < line.len() {
            visual_lines.push(VisualLine {
                byte_range: current_start..line.len(),
                wrap_type: WrapType::SoftWrap,
            });
        }

        if visual_lines.is_empty() {
            visual_lines.push(VisualLine {
                byte_range: 0..line.len(),
                wrap_type: WrapType::SoftWrap,
            });
        }

        visual_lines
    }

    fn try_hyphenate_word(
        &self,
        _word: &str,
        _available_width: Pixels,
        _shaped: &ShapedLine,
        _start_byte: usize,
    ) -> Option<Vec<VisualLine>> {
        // Hyphenation disabled for now - would require loading dictionary data
        // Future enhancement: implement proper hyphenation with embedded dictionary
        None
    }

    pub fn get_visual_lines(&self, row: usize) -> Option<&Vec<VisualLine>> {
        self.line_layouts
            .get(row)?
            .as_ref()
            .map(|layout| &layout.visual_lines)
    }

    pub fn visual_line_count(&self) -> usize {
        self.line_layouts
            .iter()
            .filter_map(|layout| layout.as_ref())
            .map(|layout| layout.visual_lines.len())
            .sum()
    }

    pub fn buffer_to_visual(&self, buffer_pos: BufferPosition) -> VisualPosition {
        let mut visual_row = 0;

        for row in 0..buffer_pos.row.min(self.lines.len()) {
            if let Some(visual_lines) = self.get_visual_lines(row) {
                visual_row += visual_lines.len();
            } else {
                visual_row += 1;
            }
        }

        if let Some(visual_lines) = self.get_visual_lines(buffer_pos.row) {
            for (visual_line_idx, visual_line) in visual_lines.iter().enumerate() {
                if buffer_pos.column >= visual_line.byte_range.start
                    && buffer_pos.column < visual_line.byte_range.end
                {
                    return VisualPosition::new(
                        visual_row + visual_line_idx,
                        buffer_pos.column - visual_line.byte_range.start,
                    );
                }
                if buffer_pos.column == visual_line.byte_range.end
                    && visual_line_idx == visual_lines.len() - 1
                {
                    return VisualPosition::new(
                        visual_row + visual_line_idx,
                        buffer_pos.column - visual_line.byte_range.start,
                    );
                }
            }
        }

        VisualPosition::new(visual_row, 0)
    }

    pub fn visual_to_buffer(&self, visual_pos: VisualPosition) -> BufferPosition {
        let mut visual_row_counter = 0;

        for (buffer_row, _line) in self.lines.iter().enumerate() {
            if let Some(visual_lines) = self.get_visual_lines(buffer_row) {
                for (_visual_line_idx, visual_line) in visual_lines.iter().enumerate() {
                    if visual_row_counter == visual_pos.visual_row {
                        let buffer_column = visual_line.byte_range.start
                            + visual_pos.column.min(visual_line.byte_range.len());
                        return BufferPosition::new(buffer_row, buffer_column);
                    }
                    visual_row_counter += 1;
                }
            } else {
                if visual_row_counter == visual_pos.visual_row {
                    return BufferPosition::new(
                        buffer_row,
                        visual_pos.column.min(self.line_len(buffer_row)),
                    );
                }
                visual_row_counter += 1;
            }
        }

        let last_row = self.lines.len().saturating_sub(1);
        let last_col = self.line_len(last_row);
        BufferPosition::new(last_row, last_col)
    }

    pub fn get_visual_line_at_position(
        &self,
        buffer_pos: BufferPosition,
    ) -> Option<(usize, &VisualLine)> {
        if let Some(visual_lines) = self.get_visual_lines(buffer_pos.row) {
            for (idx, visual_line) in visual_lines.iter().enumerate() {
                if buffer_pos.column >= visual_line.byte_range.start
                    && buffer_pos.column <= visual_line.byte_range.end
                {
                    return Some((idx, visual_line));
                }
            }
        }
        None
    }

    pub fn move_visual_up(&self, buffer_pos: BufferPosition) -> BufferPosition {
        let visual_pos = self.buffer_to_visual(buffer_pos);
        if visual_pos.visual_row == 0 {
            return buffer_pos;
        }

        let target_visual_pos = VisualPosition::new(visual_pos.visual_row - 1, visual_pos.column);
        self.visual_to_buffer(target_visual_pos)
    }

    pub fn move_visual_down(&self, buffer_pos: BufferPosition) -> BufferPosition {
        let visual_pos = self.buffer_to_visual(buffer_pos);
        let max_visual_row = self.visual_line_count().saturating_sub(1);

        if visual_pos.visual_row >= max_visual_row {
            return buffer_pos;
        }

        let target_visual_pos = VisualPosition::new(visual_pos.visual_row + 1, visual_pos.column);
        self.visual_to_buffer(target_visual_pos)
    }

    pub fn visual_line_start(&self, buffer_pos: BufferPosition) -> BufferPosition {
        if let Some((_idx, visual_line)) = self.get_visual_line_at_position(buffer_pos) {
            return BufferPosition::new(buffer_pos.row, visual_line.byte_range.start);
        }
        BufferPosition::new(buffer_pos.row, 0)
    }

    pub fn visual_line_end(&self, buffer_pos: BufferPosition) -> BufferPosition {
        if let Some((_idx, visual_line)) = self.get_visual_line_at_position(buffer_pos) {
            return BufferPosition::new(buffer_pos.row, visual_line.byte_range.end);
        }
        let line_len = self.line_len(buffer_pos.row);
        BufferPosition::new(buffer_pos.row, line_len)
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
