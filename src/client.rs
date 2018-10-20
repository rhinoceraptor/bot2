use reqwest;
use config::{Authentication};

error_chain! {
  foreign_links {
    InvalidHeader(reqwest::header::InvalidHeaderValue);
    Reqwest(reqwest::Error);
  }
}

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

#[derive(Clone, Serialize, Debug)]
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

#[derive(Clone)]
pub struct MatrixClient {
  access_token: Option<String>,
  auth: Authorization,
  server_url: String,
  client: reqwest::Client,
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
      client: reqwest::Client::new(),
    }
  }

  pub fn login(&mut self) -> Result<()> {
    let AccessToken { access_token } = self.client
      .post(&self.build_url("/login"))
      .json(&self.auth)
      .send().chain_err(|| "Unable to POST login JSON")?
      .json().chain_err(|| "Unable to deserialize login JSON")?;

    let mut headers = reqwest::header::HeaderMap::new();
    let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", access_token))?;
    headers.insert(reqwest::header::AUTHORIZATION, header_value);

    self.client = reqwest::Client::builder()
      .default_headers(headers)
      .build().chain_err(|| "Unable to build MatrixClient reqwest client")?;

    Ok(())
  }

  pub fn build_url(&self, path: &str) -> String {
    format!("{}/_matrix/client/r0{}", self.server_url, path)
  }

  // pub fn logout(&self) -> Result<Response> {
  //   self.client
  //     .post(&self.build_url(&format!("/rooms")))
  //     .json(&Message::new(message))
  //     .send()
  // }

  // pub fn sync(&self) -> Result<Response> {
  //   self.client
  //     .post(&self.build_url(&format!("/rooms")))
  //     .json(&Message::new(message))
  //     .send()
  // }

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

  // pub fn leave_room(&self) -> Result<Response> {
  //   self.client
  //     .post(&self.build_url(&format!("/rooms")))
  //     .json(&Message::new(message))
  //     .send()
  // }

  // pub fn search(&self) -> Result<Response> {
  //   self.client
  //     .post(&self.build_url(&format!("/rooms")))
  //     .json(&Message::new(message))
  //     .send()
  // }

  // pub fn get_presence(&self) -> Result<Response> {
  //   self.client
  //     .post(&self.build_url(&format!("/rooms")))
  //     .json(&Message::new(message))
  //     .send()
  // }
}

