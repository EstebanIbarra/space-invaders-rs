#[cfg(test)]
mod tests;

pub mod invader;
pub mod obstacle;
pub mod player;
pub mod projectile;
pub mod shield;

use invader::Invader;
use obstacle::Obstacle;
use player::Player;
use projectile::Projectile;

pub type Position = (usize, usize);
pub type Sprite = [[char; 3]; 2];

pub trait SetSprite {
    fn set_sprite(&mut self);
}

pub const NO_SPRITE: Sprite = [
    ['\u{0020}', '\u{0020}', '\u{0020}'],
    ['\u{0020}', '\u{0020}', '\u{0020}'],
];

#[derive(Debug, PartialEq)]
pub enum Owner {
    Player,
    Invader,
}

pub enum State {
    Start,
    Playing,
    GameOver,
}

pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
    Nightmare,
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
}

pub struct Game<'a> {
    pub state: State,
    pub difficulty: DifficultyLevel,
    pub score: usize,
    pub level: u16,
    pub tick: usize,
    pub screen: &'a mut Screen,
    pub player: &'a mut Player,
    pub invaders: Vec<&'a mut Invader>,
    pub projectiles: Vec<&'a mut Projectile>,
    pub obstacles: Vec<&'a mut Obstacle>,
}
