use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;

use crate::terminal::{Position, Size, Terminal};
use crate::{NAME, VERSION};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    // pub fn new(opts) -> Self

    pub fn run(&mut self) {
        Terminal::initialise().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn welcome() -> Result<(), Error> {
        let Size { width, .. } = Terminal::size()?;

        let text = format!("{NAME} - {VERSION}");
        let length = match u16::try_from(text.len()) {
            Ok(n) => n,
            Err(e) => panic!("Internal error: {e:#?}"),
        };
        
        // Doesn't need to be exact.
        #[allow(clippy::integer_division)]
        let amt = ((width - length) / 2 - 1) as usize;

        let padding = " ".repeat(amt);

        let mut msg = format!("~{padding}{text}");
        msg.truncate(width as usize);

        Terminal::print(&msg)?;
        Terminal::execute()
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
            if row == height / 3 {
                Self::welcome()?;
            } else {
                Terminal::print("~")?;
            }
            if row < height.saturating_sub(1) {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }
}
