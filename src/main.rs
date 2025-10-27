// Warn for using print and println macros
#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
mod terminal;

use editor::Editor;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut editor = Editor::default();

    editor.run();
}
