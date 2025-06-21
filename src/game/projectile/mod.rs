#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

pub enum Owner {
    Player,
    Invader,
}

pub struct Projectile {
    pub position: Position,
    pub owner: Owner,
    pub velocity: i16,
    pub sprite: Sprite,
}
