#[warn(clippy::all, clippy::pedantic)]
#[allow(
    clippy::must_use_candidate,
    clippy::cast_lossless,
    clippy::missing_panics_doc,
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions
)]
pub mod field;
pub mod fps;
pub mod input;
pub mod mino;
pub mod movement;
pub mod net;
pub mod position;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    log::LogPlugin,
    prelude::*,
    render::camera::ScalingMode,
    window::{WindowResized, WindowResolution},
};
use field::{
    block::field_block_system,
    local::{garbage_warning_bar_system, handle_receive_garbage, ReceiveGarbageEvent},
    timer::{drop_timer_system, lock_down_timer_system, target_change_timer_system},
};
use fps::{fps_system, setup_fps};
use input::{keyboard_input_system, KeyboardRepeatTimer};
use mino::event::{
    handle_place_mino, handle_spawn_mino, handle_sync_field_change, PlaceMinoEvent, SpawnMinoEvent,
    SyncFieldChangeEvent,
};
use movement::{handle_move, MoveEvent};
use net::{receive_message_system, setup_matchbox_socket, waiting_for_player_system};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_ASPECT: f32 = WINDOW_WIDTH / WINDOW_HEIGHT;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MatchMaking,
    Playing,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tetris".into(),
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
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
        .add_event::<SyncFieldChangeEvent>()
        .insert_resource(KeyboardRepeatTimer::default())
        .add_systems(Startup, (setup, setup_fps))
        .add_systems(Update, (camera_system, fps_system))
        .add_systems(OnEnter(AppState::MatchMaking), setup_matchbox_socket)
        .add_systems(
            Update,
            waiting_for_player_system.run_if(in_state(AppState::MatchMaking)),
        )
        .add_systems(OnEnter(AppState::Playing), setup_game)
        .add_systems(PreUpdate, (field_block_system, garbage_warning_bar_system))
        .add_systems(
            Update,
            (
                drop_timer_system,
                lock_down_timer_system,
                target_change_timer_system,
                keyboard_input_system,
                receive_message_system,
                handle_move,
                handle_spawn_mino,
                handle_place_mino,
                handle_receive_garbage,
                handle_sync_field_change,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
}

fn camera_system(
    mut resize_events: EventReader<WindowResized>,
    window_query: Query<&Window>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for _ in resize_events.iter() {
        let Ok(window) = window_query.get_single() else { return; };
        let Ok(mut projection) = projection_query.get_single_mut() else { return; };

        let window_aspect = window.width() / window.height();
        if window_aspect > WINDOW_ASPECT {
            projection.scaling_mode = ScalingMode::FixedVertical(WINDOW_HEIGHT);
        } else {
            projection.scaling_mode = ScalingMode::FixedHorizontal(WINDOW_WIDTH);
        }
    }
}

fn setup_game(mut spawn_mino_events: EventWriter<SpawnMinoEvent>) {
    spawn_mino_events.send(SpawnMinoEvent);
}
