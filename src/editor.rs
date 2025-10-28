use crossterm::event::KeyCode::{self, Char, Down, End, Home, Left, PageDown, PageUp, Right, Up};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyEventKind, KeyModifiers, read};
use std::cmp::min;
use std::io::Error;
use std::path::Path;

use terminal::{Position, Size, Terminal};
use view::View;

mod view;
mod buffer;
mod terminal;


#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor_location: Position,
    view: View
}

impl Editor {
    pub fn new(f: &Path) -> Result<Self, Error> {
        let mut new = Self::default();
        new.view.load(f)?;
        Ok(new)
    }

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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                Up | Down | Left | Right | PageDown | PageUp | End | Home => Self::move_caret(self, *code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(self.cursor_location)?;
            Terminal::show_cursor()?;
            Terminal::execute()?;
        }
        Ok(())
    }

    fn move_caret(&mut self, k: KeyCode) -> Result<(), Error> {
        let Position { mut x, mut y } = self.cursor_location;
        let Size { width, height } = Terminal::size()?;
        match k {
            // THe following need to account for terminal dimensions, hence min. (Max x, y is width, height - 1)
            Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            // These two don't need it because sat sub won't wrap, and the min in dimensions is 0 anyway.
            Up => y = y.saturating_sub(1),
            Left => x = x.saturating_sub(1),

            PageUp => y = 0,
            PageDown => y = height.saturating_sub(1),

            Home => x = 0,
            End => x = width.saturating_sub(1),
            
            _ => ()
        }
        self.cursor_location = Position { x, y };
        Ok(())
    }
}
