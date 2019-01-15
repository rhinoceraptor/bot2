use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
pub struct InviteRoom {
}

#[derive(Deserialize, Debug)]
pub struct LeaveRoom {
}


#[derive(Deserialize, Debug)]
pub struct Room {
}

#[derive(Deserialize, Debug)]
pub struct Rooms {
  join: HashMap<String, Room>,
  invite: HashMap<String, InviteRoom>,
  leave: HashMap<String, LeaveRoom>,
}

#[derive(Deserialize, Debug)]
pub struct ToDevice {
}

#[derive(Deserialize, Debug)]
pub struct JoinedRooms {
  joined_rooms: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Sync {
  // pub account_data: AccountData,
  // pub device_lists: DeviceLists,
  // pub device_one_time_keys_count: DeviceOneTimeKeysCount,
  // pub groups: Groups,
  // pub presence: Presence,
  // pub next_batch: String,
  pub rooms: Rooms,
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

impl Authorization {
  pub fn new(user: String, password: String) -> Self {
    Authorization { user, password, type_: String::from("m.login.password") }
  }
}

