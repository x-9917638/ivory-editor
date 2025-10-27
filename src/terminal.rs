use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::{Error, Write, stdout};

#[derive(Copy, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn initialise() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn print(s: impl Display) -> Result<(), Error> {
        Self::queue_cmd(Print(s))
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_cmd(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_cmd(Clear(ClearType::CurrentLine))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_cmd(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_cmd(Show)
    }

    pub fn move_cursor_to(p: Position) -> Result<(), Error> {
        Self::queue_cmd(MoveTo(p.x, p.y))
    }

    pub fn size() -> Result<Size, Error> {
        let size = size()?;
        Ok(Size {
            width: size.0,
            height: size.1,
        })
    }

    fn queue_cmd(c: impl Command) -> Result<(), Error> {
        queue!(stdout(), c)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}
