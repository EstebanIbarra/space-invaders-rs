#[cfg(test)]
mod tests;

use super::Position;
use crate::sprite::Sprite;

// u8 here represents max health
pub enum EnemyType {
    Alpha(u8),
    Beta(u8),
    Delta(u8),
    Gamma(u8),
}

pub struct Invader {
    pub position: Position,
    pub r#type: EnemyType,
    pub health: u8,
    pub alive: bool,
    pub sprite: Sprite,
}
