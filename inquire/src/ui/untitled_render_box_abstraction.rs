use std::cmp::Ordering;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::io;

use fxhash::FxHasher;
use unicode_width::UnicodeWidthChar;

use super::dimension::Dimension;
use super::{Position, Styled};
use crate::ansi::{AnsiAware, AnsiAwareChar};
use crate::terminal::{Terminal, TerminalSize};

#[derive(Debug, Default)]
struct FrameRow {
    content: Vec<Styled<String>>,
    hash: u64,
}

impl FrameRow {
    pub fn new(content: Vec<Styled<String>>, hash: u64) -> Self {
        Self { content, hash }
    }

    pub fn get_content(&self) -> &[Styled<String>] {
        &self.content
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

#[derive(Debug)]
struct FrameState {
    /// terminal size when the frame was rendered
    pub terminal_size: TerminalSize,
    /// resulting frame size
    pub frame_size: Dimension,
    /// cursor position after writing all present content
    pub cursor_position: Position,
    /// position to put cursor after writing all present content
    pub expected_cursor_position: Option<Position>,
    /// content and pre-calculated hashes for each rendered line
    /// the length of this vector should be equal to frame_size.height
    pub finished_rows: Vec<FrameRow>,
    pub current_styled: Styled<String>,
    pub current_line: Vec<Styled<String>>,
    pub current_line_hasher: FxHasher,
}

impl FrameState {
    pub fn new(terminal_size: TerminalSize) -> Self {
        Self {
            terminal_size,
            frame_size: Dimension::new(0, 0),
            cursor_position: Position::default(),
            finished_rows: Vec::new(),
            current_styled: Styled::default(),
            current_line: Vec::new(),
            current_line_hasher: FxHasher::default(),
            expected_cursor_position: None,
        }
    }

    pub fn fit_to_terminal(&mut self, new_size: TerminalSize) {
        if new_size.width() >= self.frame_size.width()
            && new_size.height() >= self.frame_size.height()
        {
            self.terminal_size = new_size;
            return;
        }

        let mut new_state = Self::new(new_size);
        for row in &self.finished_rows {
            for styled in row.get_content() {
                new_state.write(styled);
            }
        }
        for styled in &self.current_line {
            new_state.write(styled);
        }

        *self = new_state;
    }

    pub fn write(&mut self, value: &Styled<impl AsRef<str> + Display>) {
        self.current_styled.style = value.style;

        for piece in value.content.ansi_aware_chars() {
            piece.hash(&mut self.current_line_hasher);
            value.style.hash(&mut self.current_line_hasher);

            let current_char = match piece {
                AnsiAwareChar::Char(c) => c,
                AnsiAwareChar::AnsiEscapeSequence(_) => {
                    // we don't care for escape sequences when calculating cursor position
                    // and box size
                    continue;
                }
            };

            if current_char == '\n' {
                self.finish_line();
                continue;
            }

            let remaining_width_space = self.terminal_size.width() - self.cursor_position.col;
            let character_length = UnicodeWidthChar::width(current_char).unwrap_or(0) as u16;

            if character_length > remaining_width_space {
                // the character will (probably) not fit into the current line
                self.finish_line();
            }

            self.current_styled.content.push(current_char);
            self.cursor_position.col = self.cursor_position.col.saturating_add(character_length);
        }

        if !self.current_styled.content.is_empty() {
            self.current_line
                .push(std::mem::take(&mut self.current_styled));
        }
    }

    pub fn mark_cursor_position(&mut self, offset: isize) {
        let mut expected_position = self.cursor_position;
        expected_position.col = expected_position.col.saturating_add(offset as u16);

        if expected_position.col >= self.terminal_size.width() {
            expected_position.col -= self.terminal_size.width();
            expected_position.row += 1;
        }

        self.expected_cursor_position = Some(expected_position);
    }

    pub fn finish(&mut self) {
        self.finish_line();
    }

    fn finish_line(&mut self) {
        let current_styled = std::mem::take(&mut self.current_styled);
        self.current_styled.style = current_styled.style;

        if !current_styled.content.is_empty() || !current_styled.style.is_empty() {
            self.current_line.push(current_styled);
        }

        let hasher = std::mem::take(&mut self.current_line_hasher);
        let content = std::mem::take(&mut self.current_line);

        if content.is_empty() {
            return;
        }

        self.finished_rows
            .push(FrameRow::new(content, hasher.finish()));

        self.cursor_position = Position {
            col: 0,
            row: self.cursor_position.row.saturating_add(1),
        };

        if !self.current_styled.style.is_empty() {
            self.current_styled
                .style
                .hash(&mut self.current_line_hasher);
        }
    }
}

pub struct UntitledRenderBoxAbstraction<T>
where
    T: Terminal,
{
    terminal: T,
    last_rendered_frame: FrameState,
    current_frame: FrameState,
}

impl<T> UntitledRenderBoxAbstraction<T>
where
    T: Terminal,
{
    pub fn new(terminal: T) -> io::Result<Self> {
        let terminal_size = terminal.get_size()?;
        Ok(Self {
            terminal,
            last_rendered_frame: FrameState::new(terminal_size),
            current_frame: FrameState::new(terminal_size),
        })
    }

    pub fn write(&mut self, value: impl Display) -> io::Result<()> {
        self.write_styled(Styled::new(value))
    }

    pub fn write_styled(&mut self, value: Styled<impl Display>) -> io::Result<()> {
        let formatted = format!("{}", value.content);
        let value = value.with_content(formatted);

        self.current_frame.write(&value);

        Ok(())
    }

    pub fn mark_cursor_position(&mut self, offset: isize) {
        self.current_frame.mark_cursor_position(offset);
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.terminal.cursor_show()?;
        Ok(())
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        // self.terminal.cursor_hide()?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.terminal.flush()
    }

    pub fn finish_current_frame(&mut self) -> io::Result<()> {
        let terminal_size = self.terminal.get_size()?;
        self.last_rendered_frame.fit_to_terminal(terminal_size);
        self.current_frame.finish();

        let rows_to_iterate = std::cmp::max(
            self.last_rendered_frame.finished_rows.len(),
            self.current_frame.finished_rows.len(),
        );

        let cursor_position = self.last_rendered_frame.cursor_position;
        self.terminal.cursor_up(cursor_position.row)?;

        for i in 0..rows_to_iterate {
            let last_row = self.last_rendered_frame.finished_rows.get(i);
            let current_row = self.current_frame.finished_rows.get(i);
            self.terminal.cursor_move_to_column(0)?;

            match (last_row, current_row) {
                (Some(last_row), Some(current_row)) => {
                    if last_row.hash() != current_row.hash() {
                        for styled in current_row.get_content() {
                            self.terminal.write_styled(styled)?;
                        }
                        self.terminal.clear_until_new_line()?;
                    }
                }
                (Some(_), None) => {
                    self.terminal.clear_current_line()?;
                }
                (None, Some(current_row)) => {
                    for styled in current_row.get_content() {
                        self.terminal.write_styled(styled)?;
                    }
                }
                (None, None) => {
                    // unreachable, but we don't want to panic :)
                }
            }

            self.terminal.write("\n")?;
        }

        self.terminal.flush()?;

        self.last_rendered_frame =
            std::mem::replace(&mut self.current_frame, FrameState::new(terminal_size));

        Ok(())
    }

    fn move_cursor_to_end_position(&mut self) -> io::Result<()> {
        let terminal_size = self.terminal.get_size()?;
        self.current_frame.fit_to_terminal(terminal_size);

        // TODO: fit to terminal is not adapted to fix the cursor position, fix it.
        // TODO: fit to terminal is not handling final line correctly, fix it.

        let cursor_position = self.current_frame.cursor_position;
        let end_position = Position {
            col: 0,
            row: self.current_frame.frame_size.height(),
        };
        self.terminal.cursor_move_to_column(0)?;

        match end_position.row.cmp(&cursor_position.row) {
            Ordering::Greater => {
                self.terminal
                    .cursor_down(end_position.row - cursor_position.row)?;
            }
            Ordering::Less => {
                self.terminal
                    .cursor_up(cursor_position.row - end_position.row)?;
            }
            Ordering::Equal => {}
        }

        Ok(())
    }
}

impl<T> Drop for UntitledRenderBoxAbstraction<T>
where
    T: Terminal,
{
    fn drop(&mut self) {
        let _unused = self.move_cursor_to_end_position();
        let _unused = self.show_cursor();
        let _unused = self.flush();
    }
}
