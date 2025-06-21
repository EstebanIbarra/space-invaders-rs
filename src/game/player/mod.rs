#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

pub struct Player {
    pub position: Position,
    pub lives: u8,
    pub sprite: Sprite,
}
