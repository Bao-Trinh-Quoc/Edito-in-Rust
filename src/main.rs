use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    // println!("Hello, edito!");
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{c}");
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
        // println!("{}", c);
    }
}
