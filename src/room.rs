use client::{MatrixClient};

error_chain! {}

pub struct Room {
  destroyed: bool,
  pub room: String,
  pub room_alias: String,
  pub room_id: String,
  pub client: MatrixClient,
}

impl Room {
  pub fn new(room: String, room_alias: String, room_id: String, client: MatrixClient) -> Room {
    Room { destroyed: false, room, room_alias, room_id, client }
  }

  pub fn send_message(&self, message: String) -> Result<()> {
    self.client
      .send_message(&self.room_id, message)
      .chain_err(|| "Failed to send message through MatrixClient")
  }

  pub fn destroy(&mut self) -> Result<()> {
    self.client
      .leave_room(&self.room_id)
      .chain_err(|| format!("Failed to leave room {}", self.room))?;

    self.destroyed = true;
    Ok(())
  }
}

impl Drop for Room {
  fn drop(&mut self) {
    if !self.destroyed {
      println!("Room {} was dropped without being destroyed!", self.room);
    }
  }
}

