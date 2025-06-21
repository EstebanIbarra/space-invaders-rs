#[cfg(test)]
mod tests;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::error::Error;
use std::io::stdout;

pub struct Guard;

impl Guard {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut out = stdout();
        execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;
        execute!(out, Hide)?;
        Ok(Guard)
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        let mut out = stdout();
        let _ = execute!(out, Show, DisableMouseCapture, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
