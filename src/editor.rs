use crossterm::event::KeyCode::{self, Char, Down, End, Home, Left, PageDown, PageUp, Right, Up};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyEventKind, KeyModifiers, read};
use std::cmp::min;
use std::io::Error;
use std::panic::{set_hook, take_hook};

use terminal::{Position, Size, Terminal};
use view::View;

mod buffer;
mod terminal;
mod view;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    cursor_location: Position,
    view: View,
}

impl Editor {
    pub fn new(f: Option<&String>) -> Result<Self, Error> {
        let hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            hook(panic_info);
        }));
        Terminal::initialise()?;
        let mut new = Self::default();
        if let Some(filepath) = f {
            new.view.load(filepath)?;
        }
        Ok(new)
    }

    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {e}");
                    }
                }
            }
        }
    }

    // Event isn't extremely big
    #[expect(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (Up | Down | Left | Right | PageDown | PageUp | End | Home, _) => {
                    Self::move_caret(self, code);
                }
                _ => (),
            },
            Event::Resize(x, y) => {
                self.view.resize(
                    // u16 -> usize should not raise problems unless on an extremely outdated system (where usize::MAX < u16::MAX)
                    #[expect(clippy::as_conversions)]
                    Size {
                        width: x as usize,
                        height: y as usize,
                    },
                );
            }
            _ => (),
        }
    }

    fn refresh_screen(&mut self) {
        _ = Terminal::hide_cursor();
        self.view.render();
        _ = Terminal::move_cursor_to(self.cursor_location);
        _ = Terminal::show_cursor();
        _ = Terminal::execute();
    }

    fn move_caret(&mut self, k: KeyCode) {
        let Position { mut x, mut y } = self.cursor_location;
        let Size { width, height } = Terminal::size().unwrap_or_default();
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

            _ => (),
        }
        self.cursor_location = Position { x, y };
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
