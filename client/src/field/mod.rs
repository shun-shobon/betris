pub mod block;

use self::block::{FieldBlock, BLOCK_SIZE};
use crate::{
    net::PlayerId,
    random::RandomBag,
    timer::{DROP_INTERVAL, LOCK_DOWN_INTERVAL},
};
use bevy::prelude::*;

pub const FIELD_WIDTH: i8 = 10;
pub const FIELD_HEIGHT: i8 = 20;
// この値よりもブロックがせり上がった場合はゲームオーバー
pub const FIELD_MAX_HEIGHT: i8 = FIELD_HEIGHT + 20;

const FIELD_GRID_WIDTH: f32 = 1.;

type Lines = [[FieldBlock; FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize];

#[derive(Component)]
pub struct Field {
    pub player_id: PlayerId,
    pub lines: Lines,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Component)]
pub struct LocalField {
    pub random_bag: RandomBag,
    pub drop_timer: Timer,
    pub lock_down_timer: Timer,
}

impl Field {
    #[must_use]
    pub fn new(player_id: PlayerId) -> Self {
        let lines = [[FieldBlock::default(); FIELD_WIDTH as usize]; FIELD_MAX_HEIGHT as usize];

        Self { player_id, lines }
    }

    pub fn spawn(self, commands: &mut Commands, is_local_field: bool, translation: Vec3) -> Entity {
        let mut field_commands = commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(translation)),
            self,
        ));

        if is_local_field {
            field_commands
                .insert(LocalField::default())
                .with_children(spawn_grid)
                .id()
        } else {
            field_commands.with_children(spawn_grid).id()
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
