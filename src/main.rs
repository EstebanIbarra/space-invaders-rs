use crossterm::event;
use crossterm::event::Event;
use space_invaders::game::Game;
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let mut game = Game::new()?;
    loop {
        if event::poll(Duration::from_millis(100))? {
            let key = event::read()?;
            if let Event::Key(key) = key {
                game.handle_input(key);
            }
        }
        game.update();
        game.draw();
        sleep(Duration::from_millis(16));
    }
}
