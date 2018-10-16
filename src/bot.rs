use std::error::Error;
use config::{Config};
use client::{MatrixClient};
use structures::*;

pub struct Bot {
  config: Config,
  room_list: Vec<Room>,
  matrix: MatrixClient,
}

impl Bot {
  pub fn new(config: Config) -> Result<Bot, Box<Error>> {
    let auth = config.authentication.clone();
    let mut matrix = MatrixClient::new(auth);
    matrix.login()?;
    Ok(Bot { config, room_list: Vec::new(), matrix })
  }

  // Get the list of rooms the bot is already joined to,
  // and merge it with the list of rooms in config
  pub fn init(&mut self) -> Result<(), Box<Error>> {
    let joined_rooms = self.matrix.get_joined_room_ids()?;
    Ok(())
  }
}

