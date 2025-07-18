use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

fn main() {
    // println!("Hello, edito!");
    // Keep unwrap here because we want to panic if enabling raw mode fails.
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
              
                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }

                if c == 'q' {
                    println!("Exiting...");
                    break;
                }
            },
            Err(e) => println!("Error reading input: {}", e),
        }
    }
    disable_raw_mode().unwrap();
}
