#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::{Sprite, UnicodeMatrix};

pub struct Obstacle<'a> {
    pub position: Position,
    pub max_health: u8,
    pub health: u8,
    pub sprite: &'a mut Sprite,
}

pub const OBSTACLE: UnicodeMatrix = [
    [0x2597, 0x2586, 0x2596, 0x2586], // ▗▆▖ Filler: ▆
    [0x259F, 0x2580, 0x2599, 0x2580], // █▀█ Filler: ▀
];
