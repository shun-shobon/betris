pub mod event;
pub mod shape;

use self::shape::MinoShape;
use crate::{block::Block, field::FIELD_WIDTH, position::Position};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct MinoPosition(pub Position);

#[derive(Debug, Clone, Copy, Component)]
pub struct Mino;

impl Mino {
    pub fn spawn(commands: &mut Commands, shape: MinoShape, block_size: f32) -> Entity {
        commands
            .spawn(SpatialBundle::default())
            .insert((
                Mino,
                MinoPosition(Position::new((FIELD_WIDTH - shape.size()) / 2, 0)),
            ))
            .with_children(|parent| {
                for &block_pos in shape.blocks().iter() {
                    Block::spwan_with_parent(parent, shape.color(), block_size, block_pos);
                }
            })
            .id()
    }
}
