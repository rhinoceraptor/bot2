use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client};
use client::matrix_types::*;
use config::{Authentication};
use std::error::Error;

type MatrixClientResult<T> = Result<T, Box<Error>>;

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

  pub fn login(&mut self) -> MatrixClientResult<()> {
    let AccessToken { access_token } = self.client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send()?
      .json()?;

    let mut headers = HeaderMap::new();
    let header_value = HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    headers.insert(AUTHORIZATION, header_value);

    self.client = Client::builder()
      .default_headers(headers)
      .build()?;

    Ok(())
  }

  pub fn build_url(&self, path: &str) -> String {
    format!("{}/_matrix/client/r0{}", self.server_url, path)
  }

  pub fn logout(&self) -> MatrixClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/logout")))
      .send()?;

    Ok(())
  }

  pub fn join_room(&self, room_alias: &String) -> MatrixClientResult<String> {
    let RoomId { room_id } = self.client
      .post(&self.build_url(&format!("/join/{}", room_alias)))
      .send()?
      .json()?;

    Ok(room_id)
  }

  pub fn send_message(&self, room_id: &String, message: String) -> MatrixClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/send/m.room.message", room_id)))
      .json(&Message::new(message))
      .send()?;

    Ok(())
  }

  pub fn leave_room(&self, room_id: &String) -> MatrixClientResult<()> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/leave", room_id)))
      .send()?;

    Ok(())
  }

  pub fn sync(&self) -> MatrixClientResult<()> {
    let url = match &self.last_updated {
      Some(since) => self.build_url(&format!("/sync?since={}", since)),
      None => self.build_url("/sync")
    };

    let Sync { rooms } = self.client
      .get(&url)
      .send()?
      .json()?;

    println!("{:#?}", rooms);

    Ok(())
  }

  pub fn get_presence(&self) -> MatrixClientResult<()> {
    Ok(())
  }
}


