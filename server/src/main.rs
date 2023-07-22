pub mod error;
pub mod handler;
pub mod model;

use crate::model::Room;
use axum::{routing::get, Router, Server};
use model::Rooms;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::{Mutex, RwLock};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::prelude::*;

pub type AppState = Arc<RwLock<Rooms>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tetris_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let test_room = Room;
    let test_room = Arc::new(Mutex::new(test_room));

    let mut rooms = HashMap::new();
    rooms.insert("test".to_string(), test_room);
    let rooms = Arc::new(RwLock::new(rooms));

    let app = Router::new()
        .route("/health", get(health))
        .route("/rooms", get(handler::rooms::get_all_rooms))
        .route("/rooms/:room_id", get(handler::rooms::join_room))
        .with_state(rooms)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health() -> &'static str {
    "ok"
}
