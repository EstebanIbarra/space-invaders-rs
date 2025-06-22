#[cfg(test)]
mod tests;

use super::{Owner, Position, Sprite};

pub struct Projectile {
    pub position: Position,
    pub owner: Owner,
    pub velocity: i16,
    pub sprite: Sprite,
}

pub const P_PROJECTILE: Sprite = [
    ['\u{0020}', '\u{257F}', '\u{0020}'], //  ╿
    ['\u{0020}', '\u{007C}', '\u{0020}'], //  |
];

pub const I_PROJECTILE: Sprite = [
    ['\u{0020}', '\u{007C}', '\u{0020}'], //  |
    ['\u{0020}', '\u{257D}', '\u{0020}'], //  ╽
];
