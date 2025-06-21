#[cfg(test)]
mod tests;

use super::shield::Shield;
use super::Position;
use crate::sprite::{Sprite, UnicodeMatrix};

pub struct Player<'a> {
    pub position: Position,
    pub lives: u8,
    pub shield: &'a mut Shield<'a>,
    pub sprite: &'a mut Sprite,
}

pub const PLAYER: UnicodeMatrix = [
    [0x20, 0x2584, 0x20, 0x20],       //  ▄  Filler:
    [0x259F, 0x2580, 0x2599, 0x2580], // ▟▀▙ Filler: ▀
];
