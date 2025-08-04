use crossterm::cursor::Hide;
use crossterm::cursor::MoveTo;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn init() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        execute!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        execute!(stdout(), Show)?;
        Ok(())
    }
}
