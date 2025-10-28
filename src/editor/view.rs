use std::{io::Error, path::Path, fs::read_to_string};

use super::{
    buffer::Buffer,
    terminal::{Size, Terminal},
};
use crate::{NAME, VERSION};
#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn load(&mut self, f: &Path) -> Result<(), Error>{
        for line in read_to_string(f)?.lines() {
            self.buffer.append(line);
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if self.buffer.is_empty() {
            Self::render_welcome()
        } else {
            self.render_buffer()
        }
    }

    fn render_buffer(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.text.get(row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::empty_row()?;
            }
        }
        Ok(())
    }

    fn welcome() -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;

        let text = format!("{NAME} - {VERSION}");
        let length = text.len();

        // Doesn't need to be exact.
        #[expect(clippy::integer_division)]
        let amt = ((width.saturating_sub(length)) / 2).saturating_sub(1);

        let padding = " ".repeat(amt);

        let mut msg = format!("~{padding}{text}");
        msg.truncate(width);

        Terminal::print(&msg)?;
        Terminal::execute()
    }

    fn render_welcome() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;

            #[expect(clippy::integer_division)]
            if row == height / 3 {
                Self::welcome()?;
            } else {
                Self::empty_row()?;
            }

            if row == 0 {
                Terminal::print("Hello, World!")?;
            }

            if row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }
}
