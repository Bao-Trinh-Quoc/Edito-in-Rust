use crossterm::event::{
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers, read,
};
mod terminal;
use terminal::Terminal;

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

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
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

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Ending Edito \r\n");
        } else {
            Terminal::hide_cursor()?;
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
            Terminal::show_cursor()?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let terminal_size_y = Terminal::size()?.1;
        // Draw ~ in every row
        for current_row in 0..terminal_size_y {
            print!("~");
            if current_row + 1 < terminal_size_y {
                print!("\r\n");
            }
        }
        Ok(())
    }
}
