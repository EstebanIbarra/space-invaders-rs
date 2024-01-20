use crossterm::cursor::Hide;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType};
use crossterm::ExecutableCommand;
use std::io::{stdout, Result, Stdout};

#[allow(dead_code)]
pub struct Game {
    stdout: Stdout,
}

impl Game {
    pub fn new() -> Result<Self> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(Hide)?;
        Ok(Game { stdout })
    }

    pub fn handle_input(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                println!("Quitting");
                std::process::exit(0);
            }
            _ => {
                println!("Unhandled key: {:?}", key);
            }
        }
    }

    pub fn update(&mut self) {
        println!("Updating");
    }

    pub fn draw(&mut self) {
        println!("Drawing");
    }
}
