#[cfg(test)]
mod tests;

pub mod invader;
pub mod obstacle;
pub mod player;
pub mod projectile;

use invader::Invader;
use obstacle::Obstacle;
use player::Player;
use projectile::Projectile;

pub type Position = (usize, usize);

pub enum State {
    Start,
    Playing,
    GameOver,
}

pub struct Game<'a> {
    pub state: State,
    pub score: usize,
    pub level: u16,
    pub tick_count: usize,
    pub player: &'a mut Player<'a>,
    pub invaders: Vec<&'a mut Invader<'a>>,
    pub projectiles: Vec<&'a mut Projectile<'a>>,
    pub obstacles: Vec<&'a mut Obstacle<'a>>,
}
