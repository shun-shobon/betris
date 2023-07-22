use crate::{error::AppError, model::Room, AppState};
use axum::{
    extract::{ws::WebSocket, Path, Query, State, WebSocketUpgrade},
    response::Response,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_all_rooms(State(rooms): State<AppState>) -> Json<tetris_schema::Rooms> {
    let rooms = rooms.read().await;

    let res_rooms = rooms
        .keys()
        .map(|id| tetris_schema::Room { id: id.clone() })
        .collect();
    Json(tetris_schema::Rooms(res_rooms))
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomPathParam {
    room_id: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomQueryParam {
    user_id: String,
}

pub async fn join_room(
    Path(path_param): Path<JoinRoomPathParam>,
    Query(query_param): Query<JoinRoomQueryParam>,
    ws: WebSocketUpgrade,
    State(rooms): State<AppState>,
) -> Result<Response, AppError> {
    let rooms = rooms.read().await;

    let Some(room) = rooms.get(&path_param.room_id) else {
        return Err(AppError::RoomNotFound);
    };

    let room = Arc::clone(room);

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, room, query_param.user_id)))
}

async fn handle_socket(mut socket: WebSocket, room: Arc<Mutex<Room>>, user_id: String) {
    while let Some(msg) = socket.recv().await {
        let msg = msg.unwrap();
        println!("Received a message from {}: {:?}", user_id, msg);
    }
}
