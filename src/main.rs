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
    let mut editor = Editor::default();

    editor.run();
}
