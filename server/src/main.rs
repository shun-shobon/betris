pub mod handler;

use axum::{routing::get, Router, Server};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<Rooms>>;

pub type Rooms = HashMap<String, Room>;

pub struct Room;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut rooms: HashMap<String, Room> = HashMap::new();
    rooms.insert("test".to_string(), Room);
    let rooms = Arc::new(RwLock::new(rooms));

    let app = Router::new()
        .route("/health", get(health))
        .route("/rooms", get(handler::rooms::get_all_rooms))
        .with_state(rooms);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    tracing::info!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "ok"
}
