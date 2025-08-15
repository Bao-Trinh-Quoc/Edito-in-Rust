use core::cmp::min;
use crossterm::event::{
    Event::{self, Key},
    KeyCode::{self, Char},
    KeyEvent, KeyEventKind, KeyModifiers, read,
};
use std::{env, io::Error};
mod terminal;
use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    // Read - Eval - Print loop
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evalute_event(&event)?;
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
        match key_code {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = min(height.saturating_sub(1), y.saturating_add(1)),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = min(width.saturating_sub(1), x.saturating_add(1)),
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height.saturating_sub(1),
            KeyCode::Home => x = 0,
            KeyCode::End => x = width.saturating_sub(1),
            _ => (),
        }
        self.location = Location { x, y };
        Ok(())
    }
    fn evalute_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageUp
                | KeyCode::PageDown => self.move_point(*code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Ending Edito \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_caret_to(Position {
                col: self.location.x,
                row: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        // Draw ~ in every row
        for current_row in 0..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_msg()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row.saturating_add(1) < height {
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
        let width = Terminal::size()?.width;
        let len = welcom_msg.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcom_msg = format!("~{spaces} {welcom_msg}");
        welcom_msg.truncate(width); // to avoid the message is too long
        Terminal::print(welcom_msg)?;
        Ok(())
    }
}
