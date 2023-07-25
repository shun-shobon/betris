#[warn(clippy::all, clippy::pedantic)]
#[allow(
    clippy::must_use_candidate,
    clippy::cast_lossless,
    clippy::missing_panics_doc
)]
pub mod field;
pub mod fps;
pub mod input;
pub mod mino;
pub mod movement;
pub mod net;
pub mod position;
pub mod random;
pub mod timer;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*, render::camera::ScalingMode,
};
use field::{
    block::field_block_system,
    local::{garbage_line_system, handle_receive_garbage, ReceiveGarbageEvent},
};
use fps::{fps_system, setup_fps};
use input::{keyboard_input_system, KeyboardRepeatTimer};
use mino::event::{handle_place_mino, handle_spawn_mino, PlaceMinoEvent, SpawnMinoEvent};
use movement::{handle_move, MoveEvent};
use net::{receive_message_system, setup_matchbox_socket, waiting_for_player_system};
use timer::timer_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MatchMaking,
    Playing,
}

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
        .add_event::<MoveEvent>()
        .add_event::<ReceiveGarbageEvent>()
        .insert_resource(KeyboardRepeatTimer::default())
        .add_systems(Startup, (setup, setup_fps))
        .add_systems(Update, fps_system)
        .add_systems(OnEnter(AppState::MatchMaking), setup_matchbox_socket)
        .add_systems(
            Update,
            waiting_for_player_system.run_if(in_state(AppState::MatchMaking)),
        )
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(PreUpdate, (field_block_system, garbage_line_system))
        .add_systems(
            Update,
            (
                timer_system,
                keyboard_input_system,
                receive_message_system,
                handle_move,
                handle_spawn_mino,
                handle_place_mino,
                handle_receive_garbage,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(1000.);
    commands.spawn(camera_bundle);
}

fn setup_game(mut spawn_mino_events: EventWriter<SpawnMinoEvent>) {
    spawn_mino_events.send(SpawnMinoEvent);
}
