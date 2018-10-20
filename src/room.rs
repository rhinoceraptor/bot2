use client::{MatrixClient};

error_chain! {}

pub struct Room<'a> {
  destroyed: bool,
  pub room: String,
  pub room_id: String,
  pub client: &'a MatrixClient,
}

impl<'a> Room<'a> {
  pub fn new(room: String, room_id: String, client: &'a MatrixClient) -> Room<'a> {
    Room { destroyed: false, room, room_id, client }
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

