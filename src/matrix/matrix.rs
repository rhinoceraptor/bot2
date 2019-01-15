use reqwest::{
  header::HeaderMap,
  header::HeaderValue,
  header::AUTHORIZATION,
  Client as ReqwestClient
};
use matrix::types::*;
use matrix::event::*;
use config::{Authentication};
use client::{Client, ClientResult};

#[derive(Debug)]
pub struct Matrix {
  access_token: Option<String>,
  auth: Authorization,
  server_url: String,
  client: ReqwestClient,
  last_updated: Option<String>,
}

impl Client for Matrix {
  fn new(auth: Authentication) -> Matrix {
    let matrix_authorization = Authorization::new(
      auth.user.to_string(),
      auth.password.to_string()
    );

    Matrix {
      access_token: None,
      auth: matrix_authorization,
      server_url: auth.server_url,
      client: ReqwestClient::new(),
      last_updated: None,
    }
  }

  fn login(&mut self) -> ClientResult<()> {
    let AccessToken { access_token } = self.client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send()?
      .json()?;

    let mut headers = HeaderMap::new();
    let header_value = HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    headers.insert(AUTHORIZATION, header_value);

    self.client = ReqwestClient::builder()
      .default_headers(headers)
      .build()?;

    Ok(())
  }

  fn logout(&self) -> ClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/logout")))
      .send()?;
    Ok(())
  }

  fn join_room(&self, room_alias: &String) -> ClientResult<String> {
    let RoomId { room_id } = self.client
      .post(&self.build_url(&format!("/join/{}", room_alias)))
      .send()?
      .json()?;

    Ok(room_id)
  }

  fn leave_room(&self, room_id: &String) -> ClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/leave", room_id)))
      .send()?;

    Ok(())
  }

  fn send_message(&self, room_id: &String, message: String) -> ClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/send/m.room.message", room_id)))
      .json(&Message::new(message))
      .send()?;

    Ok(())
  }

  fn get_presence(&self) -> ClientResult<()> {
    Ok(())
  }
}

impl Matrix {
  fn build_url(&self, path: &str) -> String {
    format!("{}/_matrix/client/r0{}", self.server_url, path)
  }

  fn sync(&self) -> ClientResult<()> {
    let url = match &self.last_updated {
      Some(since) => self.build_url(&format!("/sync?since={}", since)),
      None => self.build_url("/sync")
    };

    let sync: Sync = self.client
      .get(&url)
      .send()?
      .json()?;

    Ok(())
  }

  fn parse_sync_events(&self, sync: Sync) -> Vec<Event> {
    let Sync { rooms, next_batch } = sync;

    let events: Vec<Event> = Vec::new();

    for (room_id, raw_event) in rooms.join.iter() {
      println!("{:#?}", room_id);
      println!("{:#?}", raw_event);
    }

    events
  }
}

