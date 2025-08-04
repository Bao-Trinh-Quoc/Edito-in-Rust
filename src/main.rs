#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    // println!("Hello, edito!");
    Editor::default().run();
}
