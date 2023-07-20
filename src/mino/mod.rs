pub mod event;
pub mod shape;
pub mod timer;

use self::{
    shape::MinoShape,
    timer::{DropTimer, LockDownTimer},
};
use crate::{block::Block, field::FIELD_WIDTH, position::Position};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MinoPosition(pub Position);

type MinoBlocks = [Position; 4];

#[derive(Debug, Clone, Copy, Component)]
pub struct Mino {
    blocks: MinoBlocks,
}

impl Mino {
    pub fn spawn(commands: &mut Commands, mino_type: MinoShape, block_size: f32) -> Entity {
        let mino = Mino {
            blocks: mino_type.blocks(),
        };

        commands
            .spawn(SpatialBundle::default())
            .insert((
                mino,
                MinoPosition(Position::new((FIELD_WIDTH - mino_type.size()) / 2, 0)),
                DropTimer::default(),
                LockDownTimer::default(),
            ))
            .with_children(|parent| {
                for &block_pos in mino.blocks.iter() {
                    Block::spwan_with_parent(parent, mino_type.color(), block_size, block_pos);
                }
            })
            .id()
    }
}
