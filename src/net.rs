use crate::{
    field::{local::ReceiveGarbageEvent, Field},
    mino::Mino,
    AppState,
};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

pub const NUM_PLAYERS: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(PeerId);

#[derive(Resource)]
pub struct Players(pub Vec<PlayerId>);

#[derive(Resource)]
pub struct Socket(MatchboxSocket<SingleChannel>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Message {
    MinoPlaced { mino: Mino },
    GarbageSent { lines: u8 },
}

pub fn setup_matchbox_socket(mut commands: Commands) {
    let room_id = "tetris";

    let room_url = format!("ws://127.0.0.1:3536/{}", room_id);
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

    let my_player_id = PlayerId(socket.id().unwrap());
    Field::new(my_player_id).spawn(&mut commands, true, Vec3::new(-500., 0., 0.));

    let player_ids = socket.connected_peers().map(PlayerId).collect::<Vec<_>>();
    for player_id in player_ids.iter() {
        // TODO: 大人数でも正しく並べる
        Field::new(*player_id).spawn(&mut commands, false, Vec3::new(500., 0., 0.));
    }

    let players = Players(player_ids);
    commands.insert_resource(players);
    app_state.set(AppState::Playing);
}

pub fn receive_message_system(
    mut socket: ResMut<Socket>,
    mut receive_garbage_events: EventWriter<ReceiveGarbageEvent>,
) {
    let Socket(socket) = &mut *socket;

    for (_, message) in socket.receive() {
        match bincode::deserialize(&message).unwrap() {
            Message::MinoPlaced { mino } => {
                info!("MinoPlaced: {:?}", mino);
                // TODO: 他プレイヤーのフィールドの状態を更新
            }
            Message::GarbageSent { lines } => {
                info!("LineSent: {}", lines);
                receive_garbage_events.send(ReceiveGarbageEvent(lines));
            }
        }
    }
}

pub fn send_garbage(Socket(socket): &mut Socket, player_id: PlayerId, lines: u8) {
    let message = Message::GarbageSent { lines };
    let message = bincode::serialize(&message).unwrap().into_boxed_slice();

    socket.send(message, player_id.0);
}
