pub mod block;
pub mod field;
pub mod mino;
pub mod position;

use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(25.);
    commands.spawn(camera_bundle);
}
