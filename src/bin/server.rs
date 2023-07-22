use bevy::{log::LogPlugin, prelude::*};
use bevy_renet::{renet::ServerEvent, transport::NetcodeServerPlugin, RenetServerPlugin};
use tetris::network::new_renet_server;

fn main() {
    let (server, transport) = new_renet_server();

    App::new()
        .add_plugins((LogPlugin::default(), MinimalPlugins))
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Update, handle_server_event)
        .run();
}

fn handle_server_event(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client connected: {}", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client disconnected: {} ({})", client_id, reason);
            }
        }
    }
}
