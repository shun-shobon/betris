use crate::AppState;
use axum::{extract::State, Json};
use tetris_schema::{Room, Rooms};

pub async fn get_all_rooms(State(rooms): State<AppState>) -> Json<Rooms> {
    let rooms = rooms.read().await;

    let res_rooms = rooms.keys().map(|id| Room { id: id.clone() }).collect();
    Json(Rooms(res_rooms))
}
