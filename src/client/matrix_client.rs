use reqwest::{
  header,
  Client
};
use client::matrix_types::*;
use config::{Authentication};
use failure::Error;

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

  pub fn login(&mut self) -> Result<(), Error> {
    let AccessToken { access_token } = self.client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send()?
      .json()?;

    let mut headers = header::HeaderMap::new();
    let header_value = header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    headers.insert(header::AUTHORIZATION, header_value);

    self.client = Client::builder()
      .default_headers(headers)
      .build()?;

    Ok(())
  }

  pub fn build_url(&self, path: &str) -> String {
    format!("{}/_matrix/client/r0{}", self.server_url, path)
  }

  pub fn logout(&self) -> Result<(), Error> {
    self.client
      .post(&self.build_url(&format!("/logout")))
      .send()?;

    Ok(())
  }

  pub fn join_room(&self, room_alias: &String) -> Result<String, Error> {
    let RoomId { room_id } = self.client
      .post(&self.build_url(&format!("/join/{}", room_alias)))
      .send()?
      .json()?;

    Ok(room_id)
  }

  pub fn send_message(&self, room_id: &String, message: String) -> Result<(), Error> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/send/m.room.message", room_id)))
      .json(&Message::new(message))
      .send()?;

    Ok(())
  }

  pub fn leave_room(&self, room_id: &String) -> Result<(), Error> {
    self.client
      .post(&self.build_url(&format!("/rooms/{}/leave", room_id)))
      .send()?;

    Ok(())
  }

  pub fn sync(&self) -> Result<(), Error> {
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

  pub fn get_presence(&self) -> Result<(), Error> {
    Ok(())
  }
}


