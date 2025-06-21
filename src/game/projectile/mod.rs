#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

pub enum Owner {
    Player,
    Invader,
}

pub struct Projectile<'a> {
    pub position: Position,
    pub owner: Owner,
    pub velocity: i16,
    pub sprite: &'a mut Sprite,
}
