pub mod block;
pub mod field;
pub mod input;
pub mod mino;
pub mod movement;
pub mod position;
pub mod timer;

use crate::field::Field;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::ScalingMode,
};
use block::{block_transform_system, BLOCK_SIZE};
use input::keyboard_input_system;
use mino::event::{handle_place_mino, handle_spwan_mino, PlaceMinoEvent, SpwanMinoEvent};
use movement::{handle_move_event, MoveEvent};
use timer::timer_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
enum GameState {
    #[default]
    MatchMaking,
    Playing,
}

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_event::<SpwanMinoEvent>()
        .add_event::<PlaceMinoEvent>()
        .add_event::<MoveEvent>()
        .insert_resource(input::KeyboardRepeatTimer::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_move_event, handle_spwan_mino, handle_place_mino),
        )
        .add_systems(Update, timer_system)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, fps_system)
        .add_systems(PostUpdate, block_transform_system)
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .run();
}

fn setup(mut commands: Commands, mut mino_events: EventWriter<SpwanMinoEvent>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.);
    commands.spawn(camera_bundle);

    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font_size: 20.,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 20.,
                    color: Color::WHITE,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.),
                left: Val::Px(5.),
                ..default()
            }),
        )
        .insert(FpsText);

    let field_entity = Field::spawn(&mut commands, BLOCK_SIZE, Vec3::new(-500., 0., 0.));
    mino_events.send(SpwanMinoEvent(field_entity));
}

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
