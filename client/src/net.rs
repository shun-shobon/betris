use crate::{
    block::BLOCK_SIZE,
    field::{Field, LocalField},
    mino::{
        event::{PlaceMinoEvent, SpawnMinoEvent},
        shape::MinoShape,
        Angle, Mino, MinoPosition,
    },
    position::Position,
    AppState,
};
use bevy::prelude::*;
use bevy_matchbox::prelude::*;
use serde::{Deserialize, Serialize};

pub const NUM_PLAYERS: usize = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(PeerId);

#[derive(Event)]
pub struct LocalPlaceMinoEvent;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Message {
    MinoPlaced {
        pos: Position,
        angle: Angle,
        shape: MinoShape,
    },
}

impl From<PlaceMinoEvent> for Message {
    fn from(event: PlaceMinoEvent) -> Self {
        Self::MinoPlaced {
            pos: event.pos,
            angle: event.angle,
            shape: event.shape,
        }
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
    Field::new(my_player_id, BLOCK_SIZE).spawn(&mut commands, true, Vec3::new(-500., 0., 0.));

    for peer in socket.connected_peers() {
        let player_id = PlayerId(peer);
        // TODO: 大人数でも正しく並べる
        Field::new(player_id, BLOCK_SIZE).spawn(&mut commands, false, Vec3::new(500., 0., 0.));
    }

    app_state.set(AppState::Playing);
}

pub fn recieve_message_system(
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
) {
    for (peer_id, message) in socket.receive() {
        match bincode::deserialize(&message).unwrap() {
            Message::MinoPlaced { pos, angle, shape } => {
                info!("MinoPlaced: {:?}", (pos, angle, shape));
                place_mino_event_writer.send(PlaceMinoEvent {
                    player_id: PlayerId(peer_id),
                    pos,
                    angle,
                    shape,
                });
            }
        }
    }
}

pub fn handle_local_spawn_mino_event(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    field_query: Query<&Field, With<LocalField>>,
    mut mino_query: Query<(Entity, &Mino, &MinoPosition)>,
    mut local_place_mino_event_reader: EventReader<LocalPlaceMinoEvent>,
    mut place_mino_event_writer: EventWriter<PlaceMinoEvent>,
    mut spwan_mino_event_writer: EventWriter<SpawnMinoEvent>,
) {
    for _ in local_place_mino_event_reader.iter() {
        let Ok(field) = field_query.get_single() else { return; };
        let (mino_entity, mino, mino_position) = mino_query.get_single_mut().unwrap();

        let place_mino_event = PlaceMinoEvent {
            player_id: field.player_id,
            pos: mino_position.0,
            angle: mino.angle,
            shape: mino.shape,
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
