use crate::{
    field::{
        blocks::{Garbages, Lines},
        local::ReceiveGarbageEvent,
        Field,
    },
    mino::{event::SyncFieldChangeEvent, Mino},
    state::StateChangeEvent,
    AppState,
};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

pub const NUM_PLAYERS: usize = 2;
const SIGNALING_SERVER_URL: &str = "ws://127.0.0.1:3536";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerId(PeerId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PlayerState {
    #[default]
    Playing,
    GameOver,
    Win,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub id: PlayerId,
    pub state: PlayerState,
}

#[derive(Resource)]
pub struct Players(pub Vec<Player>);

impl Player {
    fn new(peer_id: PeerId) -> Self {
        Self {
            id: PlayerId(peer_id),
            state: PlayerState::default(),
        }
    }
}

#[derive(Resource)]
pub struct Socket(MatchboxSocket<SingleChannel>);

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Message {
    FieldChanged {
        mino: Mino,
        clear_lines: Lines,
        garbage_lines: Garbages,
    },
    GarbageSent {
        amount: u8,
    },
    StateChanged {
        state: PlayerState,
    },
}

pub fn setup_matchbox_socket(mut commands: Commands) {
    let room_id = "tetris";

    let room_url = format!("{}/{}?next={}", SIGNALING_SERVER_URL, room_id, NUM_PLAYERS);
    info!("Connecting to matchbox server: {}", room_url);

    let builer = WebRtcSocketBuilder::new(room_url).add_channel(ChannelConfig::reliable());
    let socket = MatchboxSocket::from(builer);
    let socket = Socket(socket);
    commands.insert_resource(socket);
}

pub fn waiting_for_player_system(
    mut commands: Commands,
    mut socket: ResMut<Socket>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let Socket(socket) = &mut *socket;

    if socket.id().is_none() || socket.get_channel(0).is_err() {
        return;
    }

    for (peer, new_state) in socket.update_peers() {
        match new_state {
            PeerState::Connected => info!("Connected to peer: {}", peer),
            PeerState::Disconnected => info!("Disconnected from peer: {}", peer),
        }
    }

    // 自分は数えないので，1つ減らす
    #[allow(clippy::absurd_extreme_comparisons)]
    if socket.connected_peers().count() < NUM_PLAYERS - 1 {
        return;
    }

    info!("All player has joined, starting game!");

    let my_player = Player::new(socket.id().unwrap());
    Field::new(my_player).spawn(&mut commands, true, Vec3::new(-350., 0., 0.));

    let mut players = socket
        .connected_peers()
        .map(Player::new)
        .collect::<Vec<_>>();
    players.sort_by_key(|player| player.id);

    for &player in players.iter() {
        // TODO: 大人数でも正しく並べる
        Field::new(player).spawn(&mut commands, false, Vec3::new(350., 0., 0.));
    }

    let players = Players(players);
    commands.insert_resource(players);
    app_state.set(AppState::Playing);
}

pub fn receive_message_system(
    mut socket: ResMut<Socket>,
    mut receive_garbage_events: EventWriter<ReceiveGarbageEvent>,
    mut sync_field_change_events: EventWriter<SyncFieldChangeEvent>,
    mut state_change_events: EventWriter<StateChangeEvent>,
) {
    let Socket(socket) = &mut *socket;

    for (peer_id, message) in socket.receive() {
        match bincode::deserialize(&message).unwrap() {
            Message::FieldChanged {
                mino,
                clear_lines,
                garbage_lines,
            } => {
                info!("{}: FieldChanged", peer_id);
                sync_field_change_events.send(SyncFieldChangeEvent {
                    player_id: PlayerId(peer_id),
                    mino,
                    clear_lines,
                    garbage_lines,
                });
            }
            Message::GarbageSent { amount } => {
                info!("{}: GarbageSent", peer_id);
                receive_garbage_events.send(ReceiveGarbageEvent(amount));
            }
            Message::StateChanged { state } => {
                info!("{}: StageChanged", peer_id);
                state_change_events.send(StateChangeEvent {
                    player_id: PlayerId(peer_id),
                    state,
                });
            }
        }
    }
}

pub fn send_garbage(Socket(socket): &mut Socket, player_id: PlayerId, amount: u8) {
    let message = Message::GarbageSent { amount };
    let message = bincode::serialize(&message).unwrap().into_boxed_slice();

    socket.send(message, player_id.0);
}

pub fn broadcast_state(Socket(socket): &mut Socket, players: &Players, state: PlayerState) {
    let message = Message::StateChanged { state };
    let message = bincode::serialize(&message).unwrap().into_boxed_slice();

    for player in players.0.iter() {
        socket.send(message.clone(), player.id.0);
    }
}

pub fn sync_local_field_change(
    Socket(socket): &mut Socket,
    players: &Players,
    mino: Mino,
    clear_lines: Lines,
    garbage_lines: Garbages,
) {
    let message = Message::FieldChanged {
        mino,
        clear_lines,
        garbage_lines,
    };
    let message = bincode::serialize(&message).unwrap().into_boxed_slice();

    for player in players.0.iter() {
        socket.send(message.clone(), player.id.0);
    }
}
