use crossterm::event::{Event::Key, KeyCode::Char, read, KeyEvent, KeyModifiers, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::execute;
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit : false
        }
    }
     
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?; 
            self.evalute_event(&event);
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
   
    fn evalute_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event {
            self.should_quit = true;
        } 
    }
    
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;                     
            println!("Ending Edito \r\n");
        }
        Ok(())
    }
}
