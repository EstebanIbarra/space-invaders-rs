#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

pub struct Obstacle {
    pub position: Position,
    pub max_health: u8,
    pub health: u8,
    pub sprite: Sprite,
}
