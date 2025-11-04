use std::{fs::read_to_string, io::Error};

use super::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};
use crate::{NAME, VERSION, editor::terminal::Position};
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}

impl View {
    pub fn load(&mut self, f: &str) -> Result<(), Error> {
        for line in read_to_string(f)?.lines() {
            self.buffer.append(line);
        }
        Ok(())
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;

        if height == 0 || width == 0 {
            return;
        }

        #[expect(clippy::integer_division)]
        let welcome_row = height / 3;

        for row in 0..height {
            if let Some(line) = self.buffer.text.get(row) {
                let line = if line.len() > width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, line);
            } else if row == welcome_row && self.buffer.is_empty() {
                Self::render_line(row, &Self::construct_welcome_msg(width));
            } else {
                Self::render_line(row, "~");
            }
        }
    }

    fn render_line(pos: usize, text: &str) {
        let result = Terminal::print_line(pos, text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    fn construct_welcome_msg(width: usize) -> String {
        let text = format!("{NAME} - {VERSION}");
        let length = text.len();

        // Abort printing welcome if their terminal is too small.
        if width <= length {
            return String::from("~");
        }

        // Doesn't need to be exact.
        #[expect(clippy::integer_division)]
        let amt = ((width.saturating_sub(length)) / 2).saturating_sub(1);

        let padding = " ".repeat(amt);

        let mut msg = format!("~{padding}{text}");
        msg.truncate(width);
        msg
    }

    pub fn resize(&mut self, new: Size) {
        self.size = new;
        self.needs_redraw = true;
    }
}
