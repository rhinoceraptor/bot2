use reqwest::{
  header,
  header::InvalidHeaderValue,
  Client,
  Error as ReqwestError
};
use matrix::types::*;
use matrix::event::*;
use config::{Authentication};

error_chain! {
  foreign_links {
    InvalidHeader(InvalidHeaderValue);
    Reqwest(ReqwestError);
  }
}


#[derive(Clone)]
pub struct MatrixClient {
  access_token: Option<String>,
  auth: Authorization,
  server_url: String,
  client: Client,
  last_updated: Option<String>,
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
      server_url: auth.server_url,
      client: Client::new(),
      last_updated: None,
    }
  }

  pub fn login(&mut self) -> Result<()> {
    let AccessToken { access_token } = self.client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send().chain_err(|| "Unable to POST login JSON")?
      .json().chain_err(|| "Unable to deserialize login JSON")?;

    let mut headers = header::HeaderMap::new();
    let header_value = header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    headers.insert(header::AUTHORIZATION, header_value);

    self.client = Client::builder()
      .default_headers(headers)
      .build().chain_err(|| "Unable to build MatrixClient reqwest client")?;

    Ok(())
  }

  pub fn build_url(&self, path: &str) -> String {
    format!("{}/_matrix/client/r0{}", self.server_url, path)
  }

  pub fn logout(&self) -> Result<()> {
    self.client
      .post(&self.build_url(&format!("/logout")))
      .send().chain_err(|| "Unable to logout")?;

    Ok(())
  }

  pub fn join_room(&self, room_alias: &String) -> Result<String> {
    let RoomId { room_id } = self.client
      .post(&self.build_url(&format!("/join/{}", room_alias)))
      .send().chain_err(|| "Unable to POST join room")?
      .json().chain_err(|| "Unable to deserialize join room JSON")?;

    Ok(room_id)
  }

  pub fn send_message(&self, room_id: &String, message: String) -> Result<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/send/m.room.message", room_id)))
      .json(&Message::new(message))
      .send().chain_err(|| "Unable to send message")?;

    Ok(())
  }

  pub fn leave_room(&self, room_id: &String) -> Result<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/leave", room_id)))
      .send().chain_err(|| "Unable to POST leave room message")?;

    Ok(())
  }

  pub fn sync(&self) -> Result<()> {
    let url = match &self.last_updated {
      Some(since) => self.build_url(&format!("/sync?since={}", since)),
      None => self.build_url("/sync")
    };
    let sync: Sync = self.client
      .get(&url)
      .send().chain_err(|| "Unable to send GET /sync message")?
      .json().chain_err(|| "Unable to deserialize sync message")?;

    let events = self.parse_sync_events(sync);
    println!("{:#?}", events);

    Ok(())
  }

  pub fn parse_sync_events(&self, sync: Sync) -> Vec<Event> {
    let mut events: Vec<Event> = Vec::new();

    for (room_id, raw_event) in sync.rooms.join.iter() {
      println!("{:#?}", room_id);
      println!("{:#?}", raw_event);
    }

    events
  }

  pub fn get_presence(&self) -> Result<()> {
    Ok(())
  }
}


