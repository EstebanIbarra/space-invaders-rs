#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

pub struct Obstacle<'a> {
    pub position: Position,
    pub max_health: u8,
    pub health: u8,
    pub sprite: &'a mut Sprite,
}
