use std::error::Error;
use config::{Config};
use serde_json::{to_string};
use reqwest;
use config::*;
use structures::*;

pub struct Bot {
  config: Config,
  access_token: Option<String>,
  room_list: Vec<Room>,
}

impl Bot {
  pub fn new(config: Config) -> Self {
    Bot { config, access_token: None, room_list: Vec::new() }
  }

  pub fn init(&mut self) -> Result<(), Box<Error>> {
    self.access_token = Some(self.authenticate()?);

    println!("Got auth token");

    self.room_list = self.config.rooms
      .iter()
      .map(|room| Room::new(
        room.to_string(),
        self.join_room(room.to_string())
          .expect(&format!("Failed to get room id for {}", room))
      ))
      .collect();

    for room in &self.room_list {
      self.send_message(Message::new("wubba lubba dub dub".to_string()), room.room_id.to_string())?;
    }

    Ok(())
  }

  fn build_url(&self, path: &str) -> String {
    format!(
      "{}/_matrix/client/r0/{}",
      self.config.authentication.home_server_url,
      path
    )
  }

  fn authenticate(&self) -> Result<String, Box<Error>> {
    let Authentication {
      username,
      password,
      ..
    } = &self.config.authentication;

    let client = reqwest::Client::new();
    let AccessToken { access_token } = client
      .post(&self.build_url("login"))
      .json(&Authorization::new(username.to_string(), password.to_string()))
      .send()?
      .json()?;

    Ok(access_token)
  }

  fn join_room(&self, room: String) -> Result<String, Box<Error>> {
    let Authentication { home_server_domain, .. } = &self.config.authentication;

    let uri = self.build_url(&format!(
      "join/{}:{}?access_token={}",
      room,
      home_server_domain,
      self.access_token.as_ref().unwrap()
    ));

    let client = reqwest::Client::new();
    let RoomId { room_id } = client
      .post(&uri)
      .send()?
      .json()?;

    Ok(room_id)
  }

  fn send_message(&self, message: Message, room: String) -> Result<(), Box<Error>> {
    let Authentication { home_server_domain, .. } = &self.config.authentication;

    let uri = self.build_url(&format!(
      "rooms/{}/send/m.room.message?access_token={}",
      room,
      self.access_token.as_ref().unwrap()
    ));

    let client = reqwest::Client::new();
    let body = client
      .post(&uri)
      .json(&message)
      .send()?
      .text()?;

    Ok(())
  }
}

