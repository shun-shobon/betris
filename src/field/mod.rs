pub mod block;
pub mod blocks;
pub mod local;
pub mod next;
pub mod timer;

use self::{
    block::BLOCK_SIZE,
    blocks::Blocks,
    local::{GarbageWarningBar, LocalFieldBundle},
};
use crate::net::PlayerId;
use bevy::prelude::*;

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

const FIELD_GRID_WIDTH: f32 = 1.;

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
                    spawn_grid(parent);
                    GarbageWarningBar::spawn(parent);
                })
                .id()
        } else {
            field_commands.with_children(spawn_grid).id()
        }
    }
}

fn spawn_grid(parent: &mut ChildBuilder) {
    let width = FIELD_WIDTH as f32 * BLOCK_SIZE;
    let height = FIELD_HEIGHT as f32 * BLOCK_SIZE;

    for y in 0..=FIELD_HEIGHT {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -(y as f32 - FIELD_HEIGHT as f32 / 2.) * BLOCK_SIZE, 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(width, FIELD_GRID_WIDTH)),
                ..default()
            },
            ..default()
        });
    }

    for x in 0..=FIELD_WIDTH {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new((x as f32 - FIELD_WIDTH as f32 / 2.) * BLOCK_SIZE, 0., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(FIELD_GRID_WIDTH, height)),
                ..default()
            },
            ..default()
        });
    }
}
