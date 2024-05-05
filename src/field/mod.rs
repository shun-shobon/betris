pub mod block;
pub mod blocks;
pub mod local;
pub mod next;
pub mod timer;

use self::{
    block::{BLOCK_INSET, BLOCK_SIZE},
    blocks::Blocks,
    local::{spawn_next_hold_background, GarbageWarningBar, LocalFieldBundle},
};
use crate::{
    net::{Player, PlayerState},
    pos,
};
use bevy::{prelude::*, sprite::Anchor};

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

pub const FIELD_PIXEL_WIDTH: f32 = BLOCK_SIZE * FIELD_WIDTH as f32;
pub const FIELD_PIXEL_HEIGHT: f32 = BLOCK_SIZE * FIELD_HEIGHT as f32;

pub const FIELD_BACKGROUND_COLOR: Color = Color::rgb(0.85, 0.85, 0.85);

pub const RESULT_TEXT_SIZE: f32 = 70.0;
pub const RESULT_LOSE_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
pub const RESULT_WIN_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

#[derive(Component)]
pub struct Field {
    pub player: Player,
    pub blocks: Blocks,
}

#[derive(Component)]
pub struct ResultText;

impl Field {
    pub fn new(player: Player) -> Self {
        Self {
            player,
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
                    spawn_result_text(parent);

                    spawn_next_hold_background(parent);
                    GarbageWarningBar::spawn(parent);
                })
                .id()
        } else {
            field_commands
                .with_children(|parent| {
                    spawn_background(parent);
                    spawn_result_text(parent);
                })
                .id()
        }
    }
}

pub fn result_text_system(
    field_query: Query<&Field>,
    mut result_text_query: Query<(&mut Text, &Parent), With<ResultText>>,
) {
    for (mut text, parent) in &mut result_text_query {
        let Ok(field) = field_query.get(parent.get()) else {
            continue;
        };

        match field.player.state {
            PlayerState::Playing => {
                text.sections[0].value.replace_range(.., "");
            }
            PlayerState::GameOver => {
                text.sections[0].value.replace_range(.., "Lose...");
                text.sections[0].style.color = RESULT_LOSE_COLOR;
            }
            PlayerState::Win => {
                text.sections[0].value.replace_range(.., "Win!");
                text.sections[0].style.color = RESULT_WIN_COLOR;
            }
        }
    }
}

fn spawn_result_text(parent: &mut ChildBuilder) {
    parent.spawn((
        ResultText,
        Text2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: RESULT_TEXT_SIZE,
                    ..default()
                },
            ),
            ..default()
        },
    ));
}

fn spawn_background(parent: &mut ChildBuilder) {
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            parent.spawn(SpriteBundle {
                transform: Transform::from_translation(pos!(x, y).translation()),
                sprite: Sprite {
                    anchor: Anchor::BottomLeft,
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
