pub mod event;
pub mod shape;

use self::shape::MinoShape;
use crate::{
    block::Block,
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    position::Position,
};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MinoPosition(pub Position);

#[derive(Debug, Clone, Copy, Component, Default)]
pub struct Mino {
    pub angle: Angle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Angle {
    #[default]
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Mino {
    pub fn spawn(commands: &mut Commands, shape: MinoShape, block_size: f32) -> Entity {
        commands
            .spawn((
                SpatialBundle::default(),
                Mino::default(),
                MinoPosition(Position::new(
                    (FIELD_WIDTH - shape.size()) / 2,
                    FIELD_HEIGHT - 2, // TODO: 20行目が埋まっている場合は21行目に出現させる
                )),
            ))
            .with_children(|parent| {
                for &block_pos in shape.blocks().iter() {
                    Block::spawn_with_parent(parent, shape.color(), block_size, block_pos);
                }
            })
            .id()
    }
}

impl From<u8> for Angle {
    fn from(angle: u8) -> Self {
        match angle {
            0 => Self::Deg0,
            1 => Self::Deg90,
            2 => Self::Deg180,
            3 => Self::Deg270,
            _ => unreachable!(),
        }
    }
}

impl From<Angle> for u8 {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::Deg0 => 0,
            Angle::Deg90 => 1,
            Angle::Deg180 => 2,
            Angle::Deg270 => 3,
        }
    }
}
