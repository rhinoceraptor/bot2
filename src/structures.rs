#[derive(Debug)]
pub struct Room {
  pub room: String,
  pub room_id: String,
}

impl Room {
  pub fn new(room: String, room_id: String) -> Self {
    Room { room, room_id }
  }
}
