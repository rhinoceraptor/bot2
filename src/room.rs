use client::matrix_client::{MatrixClient};
use failure::Error;

pub struct Room<'a> {
  pub room: String,
  pub room_id: String,
  pub client: &'a MatrixClient,
}

impl<'a> Room<'a> {
  pub fn new(room: String, room_id: String, client: &'a MatrixClient) -> Room<'a> {
    Room { room, room_id, client }
  }

  pub fn send_message(&self, message: String) -> Result<(), Error> {
    self.client.send_message(&self.room_id, message)?;
    Ok(())
  }

  pub fn destroy(&mut self) -> Result<(), Error> {
    self.client.leave_room(&self.room_id)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_room() {
    assert_eq!(1, 2);
  }
}

