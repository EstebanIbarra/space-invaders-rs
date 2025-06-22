#[cfg(test)]
mod tests;

use super::{Owner, Position, SetSprite, Sprite, NO_SPRITE};

#[derive(Clone, Debug, PartialEq)]
pub enum ShieldType {
    None,
    Single,
    Double,
    Triple,
    Quad,
}

#[derive(Debug, PartialEq)]
pub struct Shield {
    pub position: Position,
    pub r#type: ShieldType,
    owner: Owner,
    sprite: Sprite,
}

impl SetSprite for Shield {
    fn set_sprite(&mut self) {
        self.sprite = match self.owner {
            Owner::Player => match self.r#type {
                ShieldType::None => NO_SPRITE,
                ShieldType::Single => P_S_SHIELD,
                ShieldType::Double => P_D_SHIELD,
                ShieldType::Triple => P_T_SHIELD,
                ShieldType::Quad => P_Q_SHIELD,
            },
            Owner::Invader => match self.r#type {
                ShieldType::None => NO_SPRITE,
                ShieldType::Single => I_S_SHIELD,
                ShieldType::Double => I_D_SHIELD,
                ShieldType::Triple => I_T_SHIELD,
                ShieldType::Quad => I_Q_SHIELD,
            },
        };
    }
}

impl Shield {
    pub fn new(position: Position, r#type: ShieldType, owner: Owner) -> Shield {
        let mut shield = Shield {
            position,
            r#type,
            owner,
            sprite: NO_SPRITE,
        };
        shield.set_sprite();
        shield
    }

    pub fn upgrade(&mut self) {
        if self.r#type == ShieldType::Quad {
            return;
        }
        self.r#type = match self.r#type {
            ShieldType::None => ShieldType::Single,
            ShieldType::Single => ShieldType::Double,
            ShieldType::Double => ShieldType::Triple,
            ShieldType::Triple => ShieldType::Quad,
            _ => panic!("Out of bounds"),
        };
        self.set_sprite();
    }

    pub fn downgrade(&mut self) {
        if self.r#type == ShieldType::None {
            return;
        }
        self.r#type = match self.r#type {
            ShieldType::Single => ShieldType::None,
            ShieldType::Double => ShieldType::Single,
            ShieldType::Triple => ShieldType::Double,
            ShieldType::Quad => ShieldType::Triple,
            _ => panic!("Out of bounds"),
        };
        self.set_sprite();
    }
}

pub const P_S_SHIELD: Sprite = [
    ['\u{0020}', '\u{0020}', '\u{0020}'], //
    ['\u{250C}', '\u{2500}', '\u{2510}'], // ┌─┐
];

pub const P_D_SHIELD: Sprite = [
    ['\u{0020}', '\u{0020}', '\u{0020}'], //
    ['\u{2554}', '\u{2550}', '\u{2557}'], // ╔═╗
];

pub const P_T_SHIELD: Sprite = [
    ['\u{250C}', '\u{2500}', '\u{2510}'], // ┌─┐
    ['\u{2554}', '\u{2550}', '\u{2557}'], // ╔═╗
];

pub const P_Q_SHIELD: Sprite = [
    ['\u{2554}', '\u{2550}', '\u{2557}'], // ╔═╗
    ['\u{2554}', '\u{2550}', '\u{2557}'], // ╔═╗
];

pub const I_S_SHIELD: Sprite = [
    ['\u{2514}', '\u{2500}', '\u{2518}'], // └─┘
    ['\u{0020}', '\u{0020}', '\u{0020}'], //
];

pub const I_D_SHIELD: Sprite = [
    ['\u{255A}', '\u{2550}', '\u{255D}'], // ╚═╝
    ['\u{0020}', '\u{0020}', '\u{0020}'], //
];

pub const I_T_SHIELD: Sprite = [
    ['\u{255A}', '\u{2550}', '\u{255D}'], // ╚═╝
    ['\u{2514}', '\u{2500}', '\u{2518}'], // └─┘
];

pub const I_Q_SHIELD: Sprite = [
    ['\u{255A}', '\u{2550}', '\u{255D}'], // ╚═╝
    ['\u{255A}', '\u{2550}', '\u{255D}'], // ╚═╝
];
