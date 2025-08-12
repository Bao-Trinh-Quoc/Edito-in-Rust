use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers, read,
};
use std::{env, io::Error};
mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event);
        }
        Ok(())
    }

    fn evalute_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_quit = true;
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Ending Edito \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        // Draw ~ in every row
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_welcome_msg() -> Result<(), Error> {
        let mut welcom_msg = format!("{NAME} {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcom_msg.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);

        welcom_msg = format!("~{spaces} {welcom_msg}");
        welcom_msg.truncate(width); // to avoid the message is too long
        Terminal::print(welcom_msg)?;
        Ok(())
    }
}
