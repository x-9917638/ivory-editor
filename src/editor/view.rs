use std::io::Error;

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
    pub fn render(&mut self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;

            if let Some(line) = self.buffer.text.get(row) {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }

            #[expect(clippy::integer_division)]
            if row == height / 3 {
                Self::welcome()?;
            } else {
                Terminal::print("~")?;
            }

            if row == 0 {
                self.buffer.append("Hello, World!");
            }
            
            if row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
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
}
