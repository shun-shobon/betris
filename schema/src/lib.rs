use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Room {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Rooms(pub Vec<Room>);
