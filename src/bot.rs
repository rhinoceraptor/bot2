use std::error::Error;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use config::{Authentication, Config};
use client::{MatrixClient};
use room::{Room};

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

  fn encode_room(&self, room: &String) -> String {
    let Authentication { server_domain, .. } = &self.config.authentication;
    let r = utf8_percent_encode(&room, DEFAULT_ENCODE_SET).to_string();
    format!("{}:{}", r, server_domain)
  }

  pub fn init(&mut self) -> Result<(), Box<Error>> {
    // Room::new(room, room_alias, room_id, client)
    self.room_list = self.config.rooms
      .iter()
      .map(|room| {
        let room_alias = self.encode_room(room);
        match self.matrix.join_room(&room_alias) {
          Ok(room_id) => Room::new(room.to_string(), room_alias, room_id, self.matrix.clone()),
          Err(e) => panic!("Unable to join {}: {}", room, e)
        }
      })
      .collect();

    for room in self.room_list.iter() {
      room.send_message("wubba lubba dub dub".to_string());
    }

    Ok(())
  }
}

