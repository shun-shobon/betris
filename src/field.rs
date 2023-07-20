use bevy::prelude::*;

use crate::mino::{shape::MinoShape, Mino};

pub const FIELD_WIDTH: i32 = 10;
pub const FIELD_HEIGHT: i32 = 20;

const FIELD_GRID_WIDTH: f32 = 1.;

#[derive(Component)]
pub struct Field {
    pub id: u32,
    pub block_size: f32,
}

#[derive(Event)]
pub struct SpwanMinoEvent(pub u32);

impl Field {
    pub fn spawn(commands: &mut Commands, block_size: f32, translation: Vec3) -> Entity {
        commands
            .spawn(SpatialBundle::from_transform(Transform::from_translation(
                translation,
            )))
            .insert(Field { id: 0, block_size })
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

pub fn handle_spwan_mino(
    mut commands: Commands,
    mut spwan_mino_events: EventReader<SpwanMinoEvent>,
    field_query: Query<(Entity, &Field)>,
) {
    for SpwanMinoEvent(id) in spwan_mino_events.iter() {
        let Some((field_entity, field)) = field_query.iter().find(|(_, field)| field.id == *id) else { continue; };

        let mino_type = MinoShape::T;

        let mino_entity = Mino::spawn(&mut commands, mino_type, field.block_size);
        commands.entity(field_entity).push_children(&[mino_entity]);
    }
}
