use crate::{block::BLOCK_SIZE, field::Field, AppState};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;

pub const NUM_PLAYERS: usize = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct PlayerId(PeerId);

pub fn setup_matchbox_socket(mut commands: Commands) {
    let room_id = "tetris";

    let room_url = format!("ws://127.0.0.1:3536/{}", room_id);
    info!("Connecting to matchbox server: {}", room_url);

    let builer = WebRtcSocketBuilder::new(room_url).add_channel(ChannelConfig::reliable());
    commands.insert_resource(MatchboxSocket::from(builer));
}

pub fn waiting_for_player_system(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
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
    if socket.connected_peers().count() < NUM_PLAYERS - 1 {
        return;
    }

    info!("All player has joined, starting game!");

    let my_player_id = PlayerId(socket.id().unwrap());
    Field::new(my_player_id, BLOCK_SIZE).spawn(&mut commands, true, Vec3::new(-500., 0., 0.));

    for peer in socket.connected_peers() {
        let player_id = PlayerId(peer);
        // TODO: 大人数でも正しく並べる
        Field::new(player_id, BLOCK_SIZE).spawn(&mut commands, false, Vec3::new(500., 0., 0.));
    }

    app_state.set(AppState::Playing);
}
