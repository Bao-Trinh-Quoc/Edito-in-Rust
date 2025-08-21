#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Some(first_arg) = args.get(1) {
        println!("Do something with the argument");
    } else {
        Editor::default().run();
    }
    // println!("Hello, edito!");
}
