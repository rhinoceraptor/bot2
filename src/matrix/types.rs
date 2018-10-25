use std::collections::HashMap;
use matrix::event::RawEvent;

#[derive(Deserialize, Debug)]
pub struct Timeline {
  pub events: Vec<RawEvent>,
}

#[derive(Deserialize, Debug)]
pub struct Room {
  pub timeline: Timeline,
}

#[derive(Deserialize, Debug)]
pub struct Rooms {
  pub join: HashMap<String, Room>,
}

#[derive(Deserialize, Debug)]
pub struct Sync {
  pub next_batch: String,
  pub rooms: Rooms,
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

impl Authorization {
  pub fn new(user: String, password: String) -> Self {
    Authorization { user, password, type_: String::from("m.login.password") }
  }
}

