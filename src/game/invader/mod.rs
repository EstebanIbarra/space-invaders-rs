#[cfg(test)]
mod tests;

use super::shield::Shield;
use super::Position;
use crate::sprite::{Sprite, UnicodeMatrix};

// u8 here represents max health
pub enum EnemyType {
    Alpha(u8),
    Beta(u8),
    Delta(u8),
    Gamma(u8),
}

pub struct Invader<'a> {
    pub position: Position,
    pub r#type: EnemyType,
    pub shield: &'a mut Shield<'a>,
    pub health: u8,
    pub alive: bool,
    pub sprite: &'a mut Sprite,
}

pub const I_ALPHA: UnicodeMatrix = [
    [0x259F, 0x2586, 0x2599, 0x2586], // ▟▆▙ Filler: ▆
    [0x2588, 0x2588, 0x2588, 0x2588], // ███ Filler: █
];
