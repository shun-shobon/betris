use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_renet::{
    renet::{DefaultChannel, RenetClient},
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use tetris::{
    block::{transform_system, BLOCK_SIZE},
    field::Field,
    input::{keyboard_input_system, KeyboardRepeatTimer},
    mino::event::{handle_place_mino, handle_spawn_mino, PlaceMinoEvent, SpawnMinoEvent},
    movement::{handle_move_event, MoveEvent},
    network::{renet_client, LocalPlayerId, ServerMessage},
    timer::timer_system,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    MatchMaking,
    Playing,
}

#[derive(Component)]
struct FpsText;

fn main() {
    let (client, transport, local_player_id) = renet_client();

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
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .add_state::<GameState>()
        .add_event::<SpawnMinoEvent>()
        .add_event::<PlaceMinoEvent>()
        .add_event::<MoveEvent>()
        .insert_resource(KeyboardRepeatTimer::default())
        .insert_resource(client)
        .insert_resource(transport)
        .insert_resource(local_player_id)
        .add_systems(Startup, setup)
        .add_systems(Update, fps_system)
        .add_systems(
            Update,
            (wait_for_players).run_if(in_state(GameState::MatchMaking)),
        )
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

fn wait_for_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    local_player_id: Res<LocalPlayerId>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        match bincode::deserialize(&message).unwrap() {
            ServerMessage::PlayerConnected { id } => {
                info!("Player connected: {}", id);

                // TODO: 相手のフィールドを並べて表示する
                if id == local_player_id.0 {
                    let filed = Field::new(id, BLOCK_SIZE);
                    Field::spawn(&mut commands, filed, true, Vec3::new(-500., 0., 0.));
                } else {
                    let filed = Field::new(id, BLOCK_SIZE);
                    Field::spawn(&mut commands, filed, false, Vec3::new(500., 0., 0.));
                }
            }
            ServerMessage::PlayerDisconnected { id: _id } => {
                // TODO: remove field
            }
            ServerMessage::GameStart => {
                info!("Game start");
                game_state.set(GameState::Playing);
            }
        }
    }
}

fn fps_system(diagnostic: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    let Some(fps) = diagnostic.get(FrameTimeDiagnosticsPlugin::FPS) else { return; };
    let Ok(mut fps_text) = query.get_single_mut() else { return; };

    fps_text.sections[1].value = format!("{:.2}", fps.average().unwrap_or_default());
}
