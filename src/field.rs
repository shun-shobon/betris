use bevy::prelude::*;

use crate::{
    random::RandomBag,
    timer::{DropTimer, LockDownTimer},
};

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;

const FIELD_GRID_WIDTH: f32 = 1.;

#[derive(Component)]
pub struct Field {
    pub id: u32,
    pub block_size: f32,
    pub random_bag: RandomBag,
}

impl Field {
    pub fn spawn(commands: &mut Commands, block_size: f32, translation: Vec3) -> Entity {
        let field = Field {
            id: 0,
            block_size,
            random_bag: RandomBag::default(),
        };

        commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(translation)),
                field,
                DropTimer::default(),
                LockDownTimer::default(),
            ))
            .with_children(|parent| {
                let width = FIELD_WIDTH as f32 * block_size;
                let height = FIELD_HEIGHT as f32 * block_size;

                for y in 0..=FIELD_HEIGHT {
                    parent.spawn(SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(
                                0.,
                                -(y as f32 - FIELD_HEIGHT as f32 / 2.) * block_size,
                                0.,
                            ),
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
                            translation: Vec3::new(
                                (x as f32 - FIELD_WIDTH as f32 / 2.) * block_size,
                                0.,
                                0.,
                            ),
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
            })
            .id()
    }
}
