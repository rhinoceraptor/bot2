use std::error::Error;
use reqwest::{Client};
use config::{Authentication};

#[derive(Serialize, Debug)]
pub struct Message {
  pub msgtype: String,
  pub body: String,
}

impl Message {
  pub fn new(body: String) -> Self {
    Message { msgtype: String::from("m.text"), body }
  }
}

#[derive(Deserialize, Debug)]
pub struct RoomId {
  pub room_id: String,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
  pub access_token: String,
}

#[derive(Serialize, Debug)]
pub struct Authorization {
  pub user: String,
  pub password: String,
  #[serde(rename = "type")]
  pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct JoinedRooms {
  joined_rooms: Vec<String>,
}

impl Authorization {
  pub fn new(user: String, password: String) -> Self {
    Authorization { user, password, type_: String::from("m.login.password") }
  }
}

pub struct MatrixClient {
  access_token: Option<String>,
  auth: Authorization,
  server_url: String
}

impl MatrixClient {
  pub fn new(auth: Authentication) -> MatrixClient {
    let matrix_authorization = Authorization::new(
      auth.user.to_string(),
      auth.password.to_string()
    );

    MatrixClient {
      access_token: None,
      auth: matrix_authorization,
      server_url: auth.server_url
    }
  }

  pub fn login(&mut self) -> Result<(), Box<Error>> {
    let client = Client::new();
    let AccessToken { access_token } = client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send()?
      .json()?;

    println!("Got auth token {}", access_token);
    self.access_token = Some(access_token);
    Ok(())
  }

  fn build_url(&self, path: &str) -> String {
    match &self.access_token {
      Some(token) => format!("{}/_matrix/client/r0{}?access_token={}", self.server_url, path, token),
      None => format!("{}/_matrix/client/r0{}", self.server_url, path)
    }
  }

  pub fn logout(&self) -> Result<(), Box<Error>> {
    Ok(())
  }

  pub fn sync(&self) -> Result<(), Box<Error>> {
    Ok(())
  }

  // Get the list of joined room IDs
  pub fn get_joined_room_ids(&self) -> Result<Vec<String>, Box<Error>> {
    let client = Client::new();
    let JoinedRooms { joined_rooms } = client
      .get(&self.build_url("/joined_rooms"))
      .send()?
      .json()?;

    println!("{:#?}", joined_rooms);
    Ok(joined_rooms)
  }

  pub fn join_room(&self) -> Result<(), Box<Error>> {
    //let Authentication { home_server_domain, .. } = &self.config.authentication;

    //let uri = self.build_url(&format!(
    //  "join/{}:{}?access_token={}",
    //  room,
    //  home_server_domain,
    //  self.access_token.as_ref().unwrap()
    //))

    //let client = reqwest::Client::new();
    //let RoomId { room_id } = client
    //  .post(&uri)
    //  .send()?
    //  .json()?;

    //Ok(room_id)
    Ok(())
  }

  pub fn leave_room(&self) -> Result<(), Box<Error>> {
    Ok(())
  }

  pub fn send_message(&self) -> Result<(), Box<Error>> {
    //let Authentication { home_server_domain, .. } = &self.config.authentication;

    //let uri = self.build_url(&format!(
    //  "rooms/{}/send/m.room.message?access_token={}",
    //  room,
    //  self.access_token.as_ref().unwrap()
    //))

    //let client = reqwest::Client::new();
    //let body = client
    //  .post(&uri)
    //  .json(&message)
    //  .send()?
    //  .text()?;

    Ok(())
  }

  pub fn search(&self) -> Result<(), Box<Error>> {
    Ok(())
  }

  pub fn get_presence(&self) -> Result<(), Box<Error>> {
    Ok(())
  }
}

