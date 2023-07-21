#[warn(clippy::all, clippy::pedantic)]
#[allow(clippy::cast_lossless)]
pub mod block;
pub mod field;
pub mod input;
pub mod mino;
pub mod movement;
pub mod position;
pub mod random;
pub mod timer;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    render::camera::ScalingMode,
};
use block::BLOCK_SIZE;
use field::Field;
use input::keyboard_input_system;
use mino::event::{handle_place_mino, handle_spawn_mino, PlaceMinoEvent, SpawnMinoEvent};
use movement::{handle_move_event, MoveEvent};
use timer::timer_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MatchMaking,
    Playing,
}

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        .add_state::<GameState>()
        .add_event::<SpawnMinoEvent>()
        .add_event::<PlaceMinoEvent>()
        .add_event::<MoveEvent>()
        .insert_resource(input::KeyboardRepeatTimer::default())
        .add_systems(Startup, setup)
        .add_systems(Update, fps_system)
        .add_systems(OnEnter(GameState::Playing), (setup_game,))
        .add_systems(
            Update,
            (
                timer_system,
                keyboard_input_system,
                handle_move_event,
                handle_spawn_mino,
                handle_place_mino,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(PostUpdate, block::transform_system)
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
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

    game_state.set(GameState::Playing);
}

fn setup_game(mut commands: Commands, mut spawn_mino_events: EventWriter<SpawnMinoEvent>) {
    let field = Field::new(0, BLOCK_SIZE);
    Field::spawn(&mut commands, field, true, Vec3::new(-500., 0., 0.));
    spawn_mino_events.send(SpawnMinoEvent);
}

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
