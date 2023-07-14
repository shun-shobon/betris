use crate::{
    field::{FIELD_HEIGHT, FIELD_WIDTH},
    mino::MinoPosition,
    position::Position,
};
use bevy::prelude::*;
use if_chain::if_chain;

pub const BLOCK_SIZE: f32 = 25.0;
pub const BLOCK_INSET: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Block {
    pub position: Position,
}

macro_rules! spwan_block {
    ($commands:tt, $color:tt, $size:tt, $position:tt) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: $color,
                    custom_size: Some(Vec2::new(
                        $size - BLOCK_INSET as f32,
                        $size - BLOCK_INSET as f32,
                    )),
                    ..default()
                },
                ..default()
            })
            .insert(Block {
                position: $position,
            });
    }};
}

impl Block {
    pub fn spawn(commands: &mut Commands, color: Color, size: f32, position: Position) {
        spwan_block!(commands, color, size, position)
    }

    pub fn spwan_with_parent(
        parent: &mut ChildBuilder,
        color: Color,
        size: f32,
        position: Position,
    ) {
        spwan_block!(parent, color, size, position)
    }

    // Based on https://stackoverflow.com/a/1996601
    pub fn rotate_right(&mut self, size: i32) {
        let Position { x: old_x, y: old_y } = self.position;

        self.position.x = 1 - (old_y - (size - 1) - 2);
        self.position.y = old_x;
    }
    pub fn rotate_left(&mut self, size: i32) {
        let Position { x: old_x, y: old_y } = self.position;

        self.position.x = old_y;
        self.position.y = 1 - (old_x - (size - 1) - 2);
    }
}

pub fn block_transform_system(
    mut query: Query<(&Block, &mut Transform, Option<&Parent>)>,
    mino_pos_query: Query<&MinoPosition>,
) {
    for (block, mut transform, parent) in query.iter_mut() {
        let mut pos = block.position;
        if_chain! {
            if let Some(parent) = parent;
            if let Ok(MinoPosition(mino_pos)) = mino_pos_query.get(parent.get());
            then {
                pos += *mino_pos;
            }
        }

        transform.translation = Vec3::new(
            (pos.x - FIELD_WIDTH) as f32 * BLOCK_SIZE,
            -(pos.y - FIELD_HEIGHT) as f32 * BLOCK_SIZE,
            0.0,
        );
    }
}
