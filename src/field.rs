use crate::{
    random::RandomBag,
    timer::{DROP_INTERVAL, LOCK_DOWN_INTERVAL},
};
use bevy::prelude::*;

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;

const FIELD_GRID_WIDTH: f32 = 1.;

#[derive(Component)]
pub struct Field {
    pub handle: usize,
    pub block_size: f32,
}

#[derive(Component)]
pub struct LocalField {
    pub random_bag: RandomBag,
    pub drop_timer: Timer,
    pub lock_down_timer: Timer,
}

impl Field {
    pub fn new(handle: usize, block_size: f32) -> Self {
        Self { handle, block_size }
    }

    pub fn spawn(
        commands: &mut Commands,
        field: Field,
        is_local_field: bool,
        translation: Vec3,
    ) -> Entity {
        let block_size = field.block_size;

        let mut field_commands = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(translation)),
            field,
        ));

        if is_local_field {
            field_commands
                .insert(LocalField::default())
                .with_children(|parent| spawn_grid(parent, block_size))
                .id()
        } else {
            field_commands
                .with_children(|parent| spawn_grid(parent, block_size))
                .id()
        }
    }
}

impl Default for LocalField {
    fn default() -> Self {
        let mut lock_down_timer = Timer::new(LOCK_DOWN_INTERVAL, TimerMode::Once);
        lock_down_timer.pause();

        Self {
            random_bag: RandomBag::new(),
            drop_timer: Timer::new(DROP_INTERVAL, TimerMode::Repeating),
            lock_down_timer,
        }
    }
}

fn spawn_grid(parent: &mut ChildBuilder, block_size: f32) {
    let width = FIELD_WIDTH as f32 * block_size;
    let height = FIELD_HEIGHT as f32 * block_size;

    for y in 0..=FIELD_HEIGHT {
        parent.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., -(y as f32 - FIELD_HEIGHT as f32 / 2.) * block_size, 0.),
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
                translation: Vec3::new((x as f32 - FIELD_WIDTH as f32 / 2.) * block_size, 0., 0.),
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
