use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use std::io::Error;


use crate::terminal::{Position, Size, Terminal};

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
        let Size{height, ..} = Terminal::size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if row != height - 1 {
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
            Terminal::move_cursor_to(Position{
                x: 0,
                y:0
            })?;
        Terminal::show_cursor()?;
        Terminal::execute()?;
        }
        Ok(())
    }
}
