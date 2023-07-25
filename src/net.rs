use crate::{
    field::{
        local::{LocalField, ReceiveGarbageEvent},
        Field,
    },
    mino::{
        event::{PlaceMinoEvent, SpawnMinoEvent},
        Mino,
    },
    AppState,
};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

pub const NUM_PLAYERS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(PeerId);

#[derive(Resource)]
pub struct Players(pub Vec<PlayerId>);

#[derive(Event)]
pub struct LocalPlaceMinoEvent;

#[derive(Event)]
pub struct LocalSendGarbageEvent {
    pub player_id: PlayerId,
    pub lines: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Message {
    MinoPlaced { mino: Mino },
    GarbageSent { lines: u8 },
}

impl From<PlaceMinoEvent> for Message {
    fn from(event: PlaceMinoEvent) -> Self {
        Self::MinoPlaced { mino: event.mino }
    }
}

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
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut receive_garbage_events: EventWriter<ReceiveGarbageEvent>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
) {
    for (peer_id, message) in socket.receive() {
        match bincode::deserialize(&message).unwrap() {
            Message::MinoPlaced { mino } => {
                info!("MinoPlaced: {:?}", mino);
                place_mino_event_writer.send(PlaceMinoEvent {
                    player_id: PlayerId(peer_id),
                    mino,
                });
            }
            Message::GarbageSent { lines } => {
                info!("LineSent: {}", lines);
                receive_garbage_events.send(ReceiveGarbageEvent(lines));
            }
        }
    }
}

pub fn handle_local_place_mino(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    field_query: Query<&Field, With<LocalField>>,
    mut mino_query: Query<(Entity, &Mino)>,
    mut local_place_mino_event_reader: EventReader<LocalPlaceMinoEvent>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
    mut spwan_mino_event_writer: EventWriter<SpawnMinoEvent>,
) {
    for _ in local_place_mino_event_reader.iter() {
        let Ok(field) = field_query.get_single() else { return; };
        let (mino_entity, &mino) = mino_query.get_single_mut().unwrap();

        let place_mino_event = PlaceMinoEvent {
            player_id: field.player_id,
            mino,
        };

        place_mino_event_writer.send(place_mino_event);
        commands.entity(mino_entity).despawn_recursive();
        spwan_mino_event_writer.send(SpawnMinoEvent);

        let message = Message::from(place_mino_event);
        for peer_id in socket.connected_peers().collect::<Vec<_>>().iter() {
            let message = bincode::serialize(&message).unwrap().into_boxed_slice();
            socket.send(message, *peer_id);
        }
    }
}

pub fn handle_local_send_garbage(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut local_send_lines_events: EventReader<LocalSendGarbageEvent>,
) {
    for event in local_send_lines_events.iter() {
        let message = Message::GarbageSent { lines: event.lines };
        let message = bincode::serialize(&message).unwrap().into_boxed_slice();

        let Some(peer_id) = socket.connected_peers().find(|&peer_id| peer_id == event.player_id.0) else { continue; };
        socket.send(message, peer_id);
    }
}
