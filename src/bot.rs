use std::error::Error;
use config::{Config};
use serde_json::{to_string};
use reqwest;
use config::*;
use structures::*;

pub struct Room {
  pub room: String,
  pub room_id: String,
}

impl Room {
  pub fn new(room: String, room_id: String) -> Self {
    Room { room, room_id }
  }
}

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

    //let rooms: Vec<Room> = self.config.rooms
    //  .iter()
    //  .map(|room| Room::new(
    //    room.to_string(),
    //    self.get_room_id(room.to_string())
    //      .expect(&format!("Failed to get room id for {}", room))
    //  ))
    //  .collect();

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

  fn get_room_id(&self, room: String) -> Result<String, Box<Error>> {
    let Authentication { home_server_domain, .. } = &self.config.authentication;

    let access_token = match self.access_token {
      Some(ref t) => t,
      None => panic!("Bot not authenticated")
    };

    let uri = self.build_url(&format!(
      "/directory/{}:{}?access_token={}",
      room,
      home_server_domain,
      access_token
    ));

    println!("{}", uri);
    let client = reqwest::Client::new();
    let body = client
      .get(&uri)
      .send()?;
    println!("{:#?}", body);

    // let RoomId { room_id, .. } = client
    //   .get(&uri)
    //   .query(&[("access_token", &self.access_token)])
    //   .send()?
    //   .json()?;


    //println!("room: {}, room_id: {}", room, room_id);
    Ok("test".to_string())
  }

  // fn send_message(&self, message, room) -> Result<(), Box<Error>> {
  //   let uri = format!(
  //     "{}/_matrix/client/r0/rooms/{}"
  //   );
  // }
}

