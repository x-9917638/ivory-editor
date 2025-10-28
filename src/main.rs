#![deny(clippy::correctness)]
#![warn(
    clippy::pedantic,
    clippy::print_stdout,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
#![allow(clippy::empty_docs)]
mod editor;

use editor::Editor;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut editor;

    if let Some(path) = args.get(1) {
        let path = std::path::Path::new(path);
        
        editor = match Editor::new(path) {
            Ok(editor) => editor,
            Err(e) => panic!("An error occured: {e:#?}")
        } 

    } else {
        editor = Editor::default();
    }

    editor.run();
}
