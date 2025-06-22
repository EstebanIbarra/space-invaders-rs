#[cfg(test)]
mod tests;

use super::shield::Shield;
use super::{Position, Sprite};

pub struct Player {
    pub position: Position,
    pub lives: u8,
    pub shield: Shield,
    pub sprite: Sprite,
}

pub const SKIN_A: Sprite = [
    ['\u{0020}', '\u{2584}', '\u{0020}'], //  ▄
    ['\u{259F}', '\u{2580}', '\u{2599}'], // ▟▀▙
];
