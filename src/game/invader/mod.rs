#[cfg(test)]
mod tests;

use super::shield::*;
use super::{DifficultyLevel, Owner, Position, Screen, SetSprite, Sprite, NO_SPRITE};

#[derive(Debug, PartialEq)]
pub enum InvaderType {
    None,
    Alpha,
    Beta,
    Delta,
    Gamma,
}

#[derive(Debug, PartialEq)]
pub struct Invader {
    pub position: Position,
    pub r#type: InvaderType,
    pub shield: Shield,
    pub health: u8,
    pub alive: bool,
    sprite: Sprite,
}

impl SetSprite for Invader {
    fn set_sprite(&mut self) {
        self.sprite = match self.r#type {
            InvaderType::None => NO_SPRITE,
            InvaderType::Alpha => ALPHA,
            InvaderType::Beta => BETA,
            InvaderType::Delta => DELTA,
            InvaderType::Gamma => GAMMA,
        };
    }
}

impl Invader {
    pub fn new(
        mut position: Position,
        shield_type: ShieldType,
        r#type: InvaderType,
        mut health: u8,
    ) -> Invader {
        if position.1 < 2 {
            position.1 += 2;
        }
        let shield = Shield::new((position.0, position.1 - 2), shield_type, Owner::Invader);
        health = match r#type {
            InvaderType::None => 0,
            InvaderType::Alpha => health,
            InvaderType::Beta => health + 1,
            InvaderType::Delta => health + 2,
            InvaderType::Gamma => health + 3,
        };
        let mut invader = Invader {
            position,
            r#type,
            shield,
            health,
            alive: true,
            sprite: NO_SPRITE,
        };
        invader.set_sprite();
        invader
    }
}

pub const ALPHA: Sprite = [
    ['\u{259F}', '\u{2586}', '\u{2599}'], // ▟▆▙
    ['\u{2588}', '\u{2588}', '\u{2588}'], // ███
];

pub const BETA: Sprite = [
    ['\u{25B2}', '\u{2582}', '\u{25B2}'], // ▲▂▲
    ['\u{25BC}', '\u{2588}', '\u{25BC}'], // ▼▀▼
];

pub const DELTA: Sprite = [
    ['\u{259B}', '\u{2588}', '\u{259C}'], // ▛█▜
    ['\u{259C}', '\u{2580}', '\u{259B}'], // ▜▀▛
];

pub const GAMMA: Sprite = [
    ['\u{259F}', '\u{2580}', '\u{2599}'], // ▟▀▙
    ['\u{259C}', '\u{2580}', '\u{259B}'], // ▜▀▛
];

pub fn spawn<'a>(difficulty: DifficultyLevel, level: u16, screen: &Screen) -> Vec<&'a mut Invader> {
    let mut invaders = vec![];
    let mut rows: usize = 4;
    let cols: usize = screen.width / 6;
    let mut shield_type = ShieldType::None;
    let mut health_sf: f32 = 1.0;
    match difficulty {
        DifficultyLevel::Easy => {
            rows = 4;
            if level > 15 {
                rows = 4;
                shield_type = ShieldType::Single;
            }
            if level > 30 {
                rows = 5;
                shield_type = ShieldType::Double;
            }
            if level > 45 {
                rows = 6;
                shield_type = ShieldType::Triple;
            }
            if level > 60 {
                rows = 7;
                shield_type = ShieldType::Quad;
            }
        }
        DifficultyLevel::Normal => {
            if level > 10 {
                shield_type = ShieldType::Single;
            }
            if level > 15 {
                rows = 5;
            }
            if level > 20 {
                shield_type = ShieldType::Double;
            }
            if level > 30 {
                rows = 6;
                shield_type = ShieldType::Triple;
            }
            if level > 40 {
                shield_type = ShieldType::Quad;
            }
            if level > 45 {
                rows = 7;
            }
        }
        DifficultyLevel::Hard => {
            health_sf = 1.25;
            shield_type = ShieldType::Single;
            if level > 10 {
                shield_type = ShieldType::Double;
            }
            if level > 20 {
                shield_type = ShieldType::Triple;
            }
            if level > 30 {
                shield_type = ShieldType::Quad;
            }
        }
        DifficultyLevel::Nightmare => {
            health_sf = 2.0;
            shield_type = ShieldType::Double;
            if level > 10 {
                shield_type = ShieldType::Triple;
            }
            if level > 20 {
                shield_type = ShieldType::Quad;
            }
        }
    }
    let health = ((level as f32) / 10.0 * health_sf + 1.0) as u8;
    for row in 0..rows {
        for col in 0..cols {
            let r#type = match row % 4 {
                0 => InvaderType::Alpha,
                1 => InvaderType::Beta,
                2 => InvaderType::Delta,
                _ => InvaderType::Gamma,
            };
            let position = (col * 6 + 2, row * 3 + 4);
            let invader = Invader::new(position, shield_type.clone(), r#type, health);
            invaders.push(Box::leak(Box::new(invader)));
        }
    }
    invaders
}
