use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    // pub fn new(opts) -> Self

    pub fn run(&self) {
        enable_raw_mode().unwrap();

        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code
                        && c == 'q'
                    {
                        break;
                    }
                }
                Err(e) => println!("Error: {e}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
