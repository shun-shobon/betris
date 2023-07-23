#[warn(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_lossless)]
pub mod block;
pub mod field;
pub mod input;
pub mod mino;
pub mod movement;
pub mod net;
pub mod position;
pub mod random;
pub mod timer;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    render::camera::ScalingMode,
};
use block::transform_system;
use input::{keyboard_input_system, KeyboardRepeatTimer};
use mino::event::{
    handle_clear_line, handle_place_mino, handle_spawn_mino, ClearLineEvent, PlaceMinoEvent,
    SpawnMinoEvent,
};
use movement::{handle_move_event, MoveEvent};
use net::{
    handle_local_spawn_mino_event, recieve_message_system, setup_matchbox_socket,
    waiting_for_player_system, LocalPlaceMinoEvent,
};
use timer::timer_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MatchMaking,
    Playing,
}

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tetris".into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_state::<AppState>()
        .add_event::<SpawnMinoEvent>()
        .add_event::<PlaceMinoEvent>()
        .add_event::<ClearLineEvent>()
        .add_event::<MoveEvent>()
        .add_event::<LocalPlaceMinoEvent>()
        .insert_resource(KeyboardRepeatTimer::default())
        .add_systems(Startup, setup)
        .add_systems(Update, fps_system)
        .add_systems(OnEnter(AppState::MatchMaking), setup_matchbox_socket)
        .add_systems(
            Update,
            waiting_for_player_system.run_if(in_state(AppState::MatchMaking)),
        )
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(
            Update,
            (
                timer_system,
                keyboard_input_system,
                recieve_message_system,
                handle_move_event,
                handle_spawn_mino,
                handle_place_mino,
                handle_clear_line.before(handle_place_mino),
                handle_local_spawn_mino_event,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(PostUpdate, transform_system)
        .run();
}

fn setup(mut commands: Commands) {
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
}

fn setup_game(mut spawn_mino_events: EventWriter<SpawnMinoEvent>) {
    spawn_mino_events.send(SpawnMinoEvent);
}

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
