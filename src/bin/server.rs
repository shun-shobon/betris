use bevy::{log::LogPlugin, prelude::*, utils::HashMap};
use bevy_renet::{
    renet::{DefaultChannel, RenetServer, ServerEvent},
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use tetris::network::{new_renet_server, ServerMessage, NUM_PLAYER};

#[derive(Resource, Default)]
pub struct Room(pub HashMap<u64, ()>);

fn main() {
    let (server, transport) = new_renet_server();

    App::new()
        .add_plugins((LogPlugin::default(), MinimalPlugins))
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .insert_resource(Room::default())
        .add_systems(Update, handle_server_event)
        .run();
}

fn handle_server_event(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut room: ResMut<Room>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client connected: {}", client_id);

                for &player_id in room.0.keys() {
                    let message =
                        bincode::serialize(&ServerMessage::PlayerConnected { id: player_id })
                            .unwrap();
                    server.send_message(*client_id, DefaultChannel::ReliableOrdered, message)
                }
                room.0.insert(*client_id, ());

                let message =
                    bincode::serialize(&ServerMessage::PlayerConnected { id: *client_id }).unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);

                if room.0.len() == NUM_PLAYER {
                    let message = bincode::serialize(&ServerMessage::GameStart).unwrap();
                    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
                }
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client disconnected: {} ({})", client_id, reason);
                room.0.remove(client_id);

                let message =
                    bincode::serialize(&ServerMessage::PlayerDisconnected { id: *client_id })
                        .unwrap();
                server.broadcast_message(DefaultChannel::ReliableOrdered, message);
            }
        }
    }
}
