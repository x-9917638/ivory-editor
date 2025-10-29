use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::{Error, Write as _, stdout};

#[derive(Default, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
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

    pub fn print(s: &str) -> Result<(), Error> {
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
    

    /// Moves the cursor to the given Position.
    /// # Arguments
    /// * `Position` - the  `Position`to move the cursor to. Will be truncated to `u16::MAX` if bigger.
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    pub fn move_cursor_to(p: Position) -> Result<(), Error> {
        Self::queue_cmd(MoveTo(p.x as u16, p.y as u16))
    }

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let size = size()?;
        Ok(Size {
            #[allow(clippy::as_conversions)]
            width: size.0 as usize,
            #[allow(clippy::as_conversions)]
            height: size.1 as usize,
        })
    }

    fn queue_cmd(c: impl Command) -> Result<(), Error> {
        queue!(stdout(), c)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }
}
