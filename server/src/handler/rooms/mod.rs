use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoomSummary {
    pub id: String,
}

pub async fn get_all_rooms(State(rooms): State<AppState>) -> Json<Vec<RoomSummary>> {
    let rooms = rooms.read().await;

    let json = rooms
        .keys()
        .map(|id| RoomSummary { id: id.clone() })
        .collect();
    Json(json)
}
