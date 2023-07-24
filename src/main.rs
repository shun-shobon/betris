#[warn(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_lossless)]
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
use field::block::field_block_system;
use fps::{fps_system, setup_fps};
use input::{keyboard_input_system, KeyboardRepeatTimer};
use mino::event::{handle_place_mino, handle_spawn_mino, PlaceMinoEvent, SpawnMinoEvent};
use movement::{handle_move_event, MoveEvent};
use net::{
    handle_local_send_lines_event, handle_local_spawn_mino_event, recieve_message_system,
    setup_matchbox_socket, waiting_for_player_system, LocalPlaceMinoEvent, LocalSendGarbageEvent,
};
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
        .add_event::<LocalPlaceMinoEvent>()
        .add_event::<LocalSendGarbageEvent>()
        .insert_resource(KeyboardRepeatTimer::default())
        .add_systems(Startup, (setup, setup_fps))
        .add_systems(Update, fps_system)
        .add_systems(OnEnter(AppState::MatchMaking), setup_matchbox_socket)
        .add_systems(
            Update,
            waiting_for_player_system.run_if(in_state(AppState::MatchMaking)),
        )
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(PreUpdate, field_block_system)
        .add_systems(
            Update,
            (
                timer_system,
                keyboard_input_system,
                recieve_message_system,
                handle_move_event,
                handle_spawn_mino,
                handle_place_mino,
                handle_local_spawn_mino_event,
                handle_local_send_lines_event,
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
