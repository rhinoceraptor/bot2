use reqwest::{
  header,
  header::InvalidHeaderValue,
  Client,
  Error as ReqwestError
};
use config::{Authentication};

error_chain! {
  foreign_links {
    InvalidHeader(InvalidHeaderValue);
    Reqwest(ReqwestError);
  }
}

#[derive(Deserialize, Debug)]
pub struct AccountData {
}

#[derive(Deserialize, Debug)]
pub struct DeviceLists {
}

#[derive(Deserialize, Debug)]
pub struct DeviceOneTimeKeysCount {
}

#[derive(Deserialize, Debug)]
pub struct Groups {
}

#[derive(Deserialize, Debug)]
pub struct Presence {
}

// #[derive(Deserialize, Debug)]
// pub struct Room {
// }
//
// #[derive(Deserialize, Debug)]
// pub struct Rooms {
//   join: HashMap<String, Room>,
//   invite: HashMap<String, InviteRoom>,
//   leave: HashMap<String, LeaveRoom>,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct ToDevice {
// }

#[derive(Deserialize, Debug)]
pub struct Sync {
  // pub account_data: AccountData,
  // pub device_lists: DeviceLists,
  // pub device_one_time_keys_count: DeviceOneTimeKeysCount,
  // pub groups: Groups,
  // pub presence: Presence,
  // pub next_batch: String,
  // pub rooms: Rooms,
  // pub to_device: ToDevice,
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
    let sync = self.client
      .get(&url)
      .send().chain_err(|| "Unable to send GET /sync message")?
      .text().chain_err(|| "Unable to deserialize sync message")?;

    println!("{}", sync);

    Ok(())
  }

  pub fn get_presence(&self) -> Result<()> {
    Ok(())
  }
}

