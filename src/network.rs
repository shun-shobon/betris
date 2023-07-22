use bevy::prelude::*;
use bevy_renet::renet::{
    transport::{
        ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication,
        ServerConfig,
    },
    ConnectionConfig, RenetClient, RenetServer,
};
use serde::{Deserialize, Serialize};
use std::{net::UdpSocket, time::SystemTime};

const SERVER_ADDR: &str = "127.0.0.1:5000";
const PROTOCOL_ID: u64 = 0;
pub const NUM_PLAYER: usize = 1;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
    GameStart,
}

#[derive(Debug, Resource)]
pub struct LocalPlayerId(pub u64);

pub fn renet_client() -> (RenetClient, NetcodeClientTransport, LocalPlayerId) {
    let client = RenetClient::new(ConnectionConfig::default());

    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport, LocalPlayerId(client_id))
}

pub fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());

    let public_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    (server, transport)
}
