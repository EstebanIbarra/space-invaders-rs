#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::{Sprite, UnicodeMatrix};

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

pub const P_PROJECTILE: UnicodeMatrix = [
    [0x20, 0x257F, 0x20, 0x20], // " ╿ " Filler: ' '
    [0x20, 0x7C, 0x20, 0x20],   // " | " Filler: ' '
];

pub const I_PROJECTILE: UnicodeMatrix = [
    [0x20, 0x7C, 0x20, 0x20],   // " | " Filler: ' '
    [0x20, 0x257D, 0x20, 0x20], // " ╽ " Filler: ' '
];
