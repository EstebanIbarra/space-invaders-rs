#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::{Sprite, UnicodeMatrix};

pub struct Shield<'a> {
    pub position: Position,
    pub active: bool,
    pub health: u8,
    pub sprite: &'a mut Sprite,
}

pub const P_S_SHIELD: UnicodeMatrix = [
    [0x20, 0x20, 0x20, 0x20],         //     Filler:
    [0x250C, 0x2500, 0x2510, 0x2500], // ┌─┐ Filler: ─
];

pub const P_D_SHIELD: UnicodeMatrix = [
    [0x20, 0x20, 0x20, 0x20],         //     Filler:
    [0x2554, 0x2550, 0x2557, 0x2550], // ╔═╗ Filler: ═
];

pub const P_T_SHIELD: UnicodeMatrix = [
    [0x250C, 0x2500, 0x2510, 0x2500], // ┌─┐ Filler: ─
    [0x2554, 0x2550, 0x2557, 0x2550], // ╔═╗ Filler: ═
];

pub const P_Q_SHIELD: UnicodeMatrix = [
    [0x2554, 0x2550, 0x2557, 0x2550], // ╔═╗ Filler: ═
    [0x2554, 0x2550, 0x2557, 0x2550], // ╔═╗ Filler: ═
];

pub const I_S_SHIELD: UnicodeMatrix = [
    [0x2514, 0x2500, 0x2518, 0x2500], // └─┘ Filler: ─
    [0x20, 0x20, 0x20, 0x20],         //     Filler:
];

pub const I_D_SHIELD: UnicodeMatrix = [
    [0x255A, 0x2550, 0x255D, 0x2550], // ╚═╝ Filler: ═
    [0x20, 0x20, 0x20, 0x20],         //     Filler:
];

pub const I_T_SHIELD: UnicodeMatrix = [
    [0x255A, 0x2550, 0x255D, 0x2550], // ╚═╝ Filler: ═
    [0x2514, 0x2500, 0x2518, 0x2500], // └─┘ Filler: ─
];

pub const I_Q_SHIELD: UnicodeMatrix = [
    [0x255A, 0x2550, 0x255D, 0x2550], // ╚═╝ Filler: ═
    [0x255A, 0x2550, 0x255D, 0x2550], // ╚═╝ Filler: ═
];
