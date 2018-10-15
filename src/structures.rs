
#[derive(Deserialize, Debug)]
pub struct AccessToken {
  pub access_token: String,
}

#[derive(Serialize, Debug)]
pub struct Authorization {
  pub user: String,
  pub password: String,
  #[serde(rename = "type")]
  pub type_: String
}

impl Authorization {
  pub fn new(user: String, password: String) -> Self {
    Authorization {
      user,
      password,
      type_: String::from("m.login.password")
    }
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

#[derive(Debug)]
pub struct Room {
  pub room: String,
  pub room_id: String,
}

impl Room {
  pub fn new(room: String, room_id: String) -> Self {
    Room { room, room_id }
  }
}
