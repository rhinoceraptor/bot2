use std::error::Error;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use config::{Authentication, Config};
use client::{MatrixClient};
use room::{Room};

pub struct Bot {
  config: Config,
  room_list: Vec<Room>,
  matrix_client: MatrixClient,
}

impl Bot {
  pub fn new(config: Config) -> Result<Bot, Box<Error>> {
    let auth = config.authentication.clone();
    let mut matrix_client = MatrixClient::new(auth);
    matrix_client.login()?;
    Ok(Bot { config, room_list: Vec::new(), matrix_client })
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
        match self.matrix_client.join_room(&room_alias) {
          Ok(room_id) => Room::new(room.to_string(), room_alias, room_id, self.matrix_client.clone()),
          Err(e) => panic!("Unable to join {}: {}", room, e)
        }
      })
      .collect();

    self.matrix_client.sync()?;

    // for room in self.room_list.iter() {
    //   room.send_message("wubba lubba dub dub".to_string());
    // }

    for room in self.room_list.iter_mut() {
      room.destroy()?;
    }

    self.room_list = Vec::new();
    self.matrix_client.logout()?;

    Ok(())
  }
}

