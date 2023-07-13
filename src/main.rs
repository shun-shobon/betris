pub mod board;
pub mod mino;

use bevy::prelude::*;
use board::Board;
use mino::MINO_SIZE;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_game)
        .add_systems(Update, despawn_board_blocks.pipe(spawn_board_blocks))
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-200.0, 300.0, 0.0)),
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
                        translation: Vec3::new((x as f32) * MINO_SIZE, (y as f32) * MINO_SIZE, 0.0),
                        scale: Vec3::new(MINO_SIZE, MINO_SIZE, 1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: color.to_color(),
                        ..default()
                    },
                    ..default()
                })
                .id();

            commands.entity(board_entity).push_children(&[block]);
        }
    }
}
