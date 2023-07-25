pub mod block;
pub mod blocks;
pub mod local;
pub mod next;
pub mod timer;

use self::{
    block::{BLOCK_INSET, BLOCK_SIZE},
    blocks::Blocks,
    local::{GarbageWarningBar, LocalField, LocalFieldBundle},
    next::QUEUE_SIZE,
};
use crate::{mino::Angle, net::PlayerId, pos, position::Position};
use bevy::{prelude::*, sprite::Anchor};

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

pub const FIELD_PIXEL_WIDTH: f32 = BLOCK_SIZE * FIELD_WIDTH as f32;
pub const FIELD_PIXEL_HEIGHT: f32 = BLOCK_SIZE * FIELD_HEIGHT as f32;

pub const NEXT_HOLD_BLOCK_SIZE: f32 = BLOCK_SIZE * 0.6;
pub const NEXT_HOLD_BLOCK_INSET: f32 = BLOCK_INSET * 0.6;
pub const NEXT_HOLD_BG_PADDING: f32 = NEXT_HOLD_BLOCK_SIZE * 0.5;
pub const NEXT_HOLD_BG_WIDTH: f32 = NEXT_HOLD_BLOCK_SIZE * 4.0 + NEXT_HOLD_BG_PADDING * 2.0;
pub const NEXT_HOLD_BG_HEIGHT: f32 = NEXT_HOLD_BLOCK_SIZE * 2.0 + NEXT_HOLD_BG_PADDING * 2.0;
pub const NEXT_START_X: f32 =
    FIELD_PIXEL_WIDTH / 2.0 + NEXT_HOLD_BG_PADDING + NEXT_HOLD_BG_WIDTH / 2.0;
pub const HOLD_START_X: f32 = -NEXT_START_X;
pub const NEXT_HOLD_BG_START_Y: f32 = FIELD_PIXEL_HEIGHT / 2.0 - NEXT_HOLD_BG_HEIGHT / 2.0;

pub const FIELD_BACKGROUND_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);

#[derive(Component)]
pub struct Field {
    pub player_id: PlayerId,
    pub blocks: Blocks,
}

#[derive(Component)]
pub struct NextHoldBlock;

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
                    spawn_next_hold_background(parent);
                    GarbageWarningBar::spawn(parent);
                })
                .id()
        } else {
            field_commands.with_children(spawn_background).id()
        }
    }
}

pub fn next_hold_block_system(
    mut commands: Commands,
    block_query: Query<Entity, With<NextHoldBlock>>,
    field_query: Query<(Entity, &LocalField), With<LocalField>>,
) {
    for entity in block_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let Ok((field_entity, field)) = field_query.get_single() else { return; };
    commands.entity(field_entity).with_children(|parent| {
        for (i, shape) in field.next_queue.queue().iter().enumerate() {
            let base = next_pos(i);

            for &pos in shape.blocks(Angle::default()) {
                let translation =
                    base + pos_to_translation(pos, shape.spawn_y_offset(), shape.width());

                let bundle = create_next_hold_block_bundle(translation, shape.color());
                parent.spawn(bundle);
            }
        }

        if let Some(shape) = field.hold {
            let base = Vec3::new(HOLD_START_X, NEXT_HOLD_BG_START_Y, 0.0);

            for &pos in shape.blocks(Angle::default()) {
                let translation =
                    base + pos_to_translation(pos, shape.spawn_y_offset(), shape.width());

                let bundle = create_next_hold_block_bundle(translation, shape.color());
                parent.spawn(bundle);
            }
        }
    });
}

fn create_next_hold_block_bundle(translation: Vec3, color: Color) -> (SpriteBundle, NextHoldBlock) {
    (
        SpriteBundle {
            transform: Transform::from_translation(translation),
            sprite: Sprite {
                color,
                anchor: Anchor::BottomLeft,
                custom_size: Some(Vec2::new(
                    NEXT_HOLD_BLOCK_SIZE - NEXT_HOLD_BLOCK_INSET,
                    NEXT_HOLD_BLOCK_SIZE - NEXT_HOLD_BLOCK_INSET,
                )),
                ..default()
            },
            ..default()
        },
        NextHoldBlock,
    )
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

fn spawn_next_hold_background(parent: &mut ChildBuilder) {
    let next_hold_sprite = Sprite {
        color: FIELD_BACKGROUND_COLOR,
        custom_size: Some(Vec2::new(NEXT_HOLD_BG_WIDTH, NEXT_HOLD_BG_HEIGHT)),
        ..default()
    };

    for i in 0..QUEUE_SIZE {
        let translation = next_pos(i);

        parent.spawn(SpriteBundle {
            transform: Transform::from_translation(translation),
            sprite: next_hold_sprite.clone(),
            ..default()
        });
    }

    parent.spawn(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(HOLD_START_X, NEXT_HOLD_BG_START_Y, 0.0)),
        sprite: next_hold_sprite,
        ..default()
    });
}

#[allow(clippy::cast_precision_loss)]
fn next_pos(i: usize) -> Vec3 {
    Vec3::new(
        NEXT_START_X,
        NEXT_HOLD_BG_START_Y - (NEXT_HOLD_BG_HEIGHT + NEXT_HOLD_BG_PADDING) * i as f32,
        0.0,
    )
}

fn pos_to_translation(pos: Position, offset_y: i8, width: i8) -> Vec3 {
    let x = (pos.x as f32 - width as f32 / 2.0) * NEXT_HOLD_BLOCK_SIZE;
    let y = (pos.y - offset_y - 1) as f32 * NEXT_HOLD_BLOCK_SIZE;

    Vec3::new(x, y, 0.0)
}
