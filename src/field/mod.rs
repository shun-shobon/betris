pub mod block;
pub mod blocks;
pub mod local;
pub mod next;
pub mod timer;

use self::{
    block::{BLOCK_INSET, BLOCK_SIZE},
    blocks::Blocks,
    local::{GarbageWarningBar, LocalFieldBundle},
};
use crate::{net::PlayerId, pos};
use bevy::prelude::*;

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

pub const FIELD_PIXEL_WIDTH: f32 = BLOCK_SIZE * FIELD_WIDTH as f32;
pub const FIELD_PIXEL_HEIGHT: f32 = BLOCK_SIZE * FIELD_HEIGHT as f32;

pub const FIELD_BACKGROUND_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);

#[derive(Component)]
pub struct Field {
    pub player_id: PlayerId,
    pub blocks: Blocks,
}

impl Field {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            blocks: Blocks::default(),
        }
    }

    pub fn spawn(self, commands: &mut Commands, is_local_field: bool, translation: Vec3) -> Entity {
        let mut field_commands = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(translation)),
            self,
        ));

        if is_local_field {
            field_commands
                .insert(LocalFieldBundle::default())
                .with_children(|parent| {
                    spawn_background(parent);
                    GarbageWarningBar::spawn(parent);
                })
                .id()
        } else {
            field_commands.with_children(spawn_background).id()
        }
    }
}

fn spawn_background(parent: &mut ChildBuilder) {
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            parent.spawn(SpriteBundle {
                transform: Transform::from_translation(pos!(x, y).translation()),
                sprite: Sprite {
                    color: FIELD_BACKGROUND_COLOR,
                    custom_size: Some(Vec2::new(
                        BLOCK_SIZE - BLOCK_INSET,
                        BLOCK_SIZE - BLOCK_INSET,
                    )),
                    ..default()
                },
                ..default()
            });
        }
    }
}
