pub mod block;
pub mod board;
pub mod mino;
pub mod position;

use bevy::{prelude::*, render::camera::ScalingMode};
use board::Board;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_game)
        .add_systems(Update, despawn_board_blocks.pipe(spawn_board_blocks))
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(25.);
    commands.spawn(camera_bundle);
}

fn setup_game(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-10.0, -10.0, 0.0)),
            ..default()
        })
        .insert(Board::default());
}

fn despawn_board_blocks(mut commands: Commands, board: Query<&Children, With<Board>>) {
    let Ok(children) = board.get_single() else { return; };

    for &child in children.iter() {
        commands.entity(child).despawn_recursive();
    }
}

fn spawn_board_blocks(mut commands: Commands, board: Query<(Entity, &Board)>) {
    let Ok((board_entity, board)) = board.get_single() else { return; };

    for (y, row) in board.0.iter().enumerate() {
        for (x, color) in row.iter().enumerate() {
            let Some(color) = color else { continue; };

            let block = commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(x as f32, y as f32, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: color.to_color(),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(board_entity).push_children(&[block]);
        }
    }
}
