#[cfg(test)]
mod tests;

use super::{Position, Sprite};

pub struct Obstacle {
    pub position: Position,
    pub max_health: u8,
    pub health: u8,
    pub sprite: Sprite,
}

pub const OBSTACLE: Sprite = [
    ['\u{2597}', '\u{2586}', '\u{2596}'], // ▗▆▖
    ['\u{259F}', '\u{2580}', '\u{2599}'], // █▀█
];
