use client::{MatrixClient};

error_chain! {}

pub struct Room {
  pub room: String,
  pub room_alias: String,
  pub room_id: String,
  pub client: MatrixClient,
}

impl Room {
  pub fn new(room: String, room_alias: String, room_id: String, client: MatrixClient) -> Room {
    Room { room, room_alias, room_id, client }
  }

  pub fn send_message(&self, message: String) -> Result<()> {
    self.client
      .send_message(&self.room_id, message)
      .chain_err(|| "Failed to send message through MatrixClient")

  }
}

impl Drop for Room {
  fn drop(&mut self) {
  }
}

