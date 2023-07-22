use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub type Rooms = HashMap<String, Arc<Mutex<Room>>>;

pub struct Room;
