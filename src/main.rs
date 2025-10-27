// Warn for using print and println macros
#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
mod terminal;

use editor::Editor;

fn main() {
    let mut editor = Editor::default();

    editor.run();
}
