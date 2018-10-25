
#[derive(Deserialize, Debug, Clone)]
pub struct EventContent {
  pub body: Option<String>,
  pub msgtype: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawEvent {
  #[serde(rename = "type")]
  pub type_: String,
  pub content: Option<EventContent>,
  pub sender: Option<String>,
  pub event_id: Option<String>,
  pub origin_server_ts: Option<i64>,
}

#[derive(Debug)]
pub enum RoomMessage {
  Audio(RawEvent),
  Emote(RawEvent),
  File(RawEvent),
  Image(RawEvent),
  Location(RawEvent),
  Notice(RawEvent),
  Text(RawEvent),
  Video(RawEvent),
  Feedback(RawEvent),
}

#[derive(Debug)]
pub enum Event {
  CallAnswer(RawEvent),
  CallCandidates(RawEvent),
  CallHangup(RawEvent),
  CallInvite(RawEvent),
  Direct(RawEvent),
  ForwardedRoomKey(RawEvent),
  FullyRead(RawEvent),
  IgnoredUserList(RawEvent),
  Presence(RawEvent),
  Receipt(RawEvent),
  RoomAliases(RawEvent),
  RoomAvatar(RawEvent),
  RoomCanonicalAlias(RawEvent),
  RoomCreate(RawEvent),
  RoomEncrypted(RawEvent),
  RoomEncryption(RawEvent),
  RoomGuestAccess(RawEvent),
  RoomHistoryAvailability(RawEvent),
  RoomJoinRules(RawEvent),
  RoomMember(RawEvent),
  RoomMessage(RoomMessage),
  RoomMessageFeedback(RawEvent),
  RoomName(RawEvent),
  PinnedEvents(RawEvent),
  RoomPowerLevels(RawEvent),
  RoomRedaction(RawEvent),
  RoomServerAcl(RawEvent),
  RoomThirdPartyInvite(RawEvent),
  RoomTopic(RawEvent),
  RoomKey(RawEvent),
  RoomKeyRequest(RawEvent),
  Sticker(RawEvent),
  Tag(RawEvent),
  Typing(RawEvent),
  NotHandled,
}

pub fn parse_room_message (event: RawEvent) -> Event {
  match event.content.clone() {
    Some(content) => match content.msgtype {
      Some(msgtype) => match msgtype.as_str() {
        "m.audio" => Event::RoomMessage(RoomMessage::Audio(event)),
        "m.emote" => Event::RoomMessage(RoomMessage::Emote(event)),
        "m.file" => Event::RoomMessage(RoomMessage::File(event)),
        "m.image" => Event::RoomMessage(RoomMessage::Image(event)),
        "m.location" => Event::RoomMessage(RoomMessage::Location(event)),
        "m.notice" => Event::RoomMessage(RoomMessage::Notice(event)),
        "m.text" => Event::RoomMessage(RoomMessage::Text(event)),
        "m.video" => Event::RoomMessage(RoomMessage::Video(event)),
        _ =>  Event::NotHandled
      },
      None => Event::NotHandled
    },
    None => Event::NotHandled
  }
}

pub fn parse_event_type (event: RawEvent) -> Event {
  match event.type_.as_str() {
    "m.call.answer" => Event::CallAnswer(event),
    "m.call.candidates" => Event::CallCandidates(event),
    "m.call.hangup" => Event::CallHangup(event),
    "m.call.invite" => Event::CallInvite(event),
    "m.direct" => Event::Direct(event),
    "m.forwarded_room_key" => Event::ForwardedRoomKey(event),
    "m.fully_read" => Event::FullyRead(event),
    "m.ignored_user_list" => Event::IgnoredUserList(event),
    "m.presence" => Event::Presence(event),
    "m.receipt" => Event::Receipt(event),
    "m.room.aliases" => Event::RoomAliases(event),
    "m.room.avatar" => Event::RoomAvatar(event),
    "m.room.canonical_alias" => Event::RoomCanonicalAlias(event),
    "m.room.create" => Event::RoomCreate(event),
    "m.room.encrypted" => Event::RoomEncrypted(event),
    "m.room.encryption" => Event::RoomEncryption(event),
    "m.room.guest_access" => Event::RoomGuestAccess(event),
    "m.room.history_visibility" => Event::RoomHistoryAvailability(event),
    "m.room.join_rules" => Event::RoomJoinRules(event),
    "m.room.member" => Event::RoomMember(event),
    "m.room.message" => parse_room_message(event),
    "m.room.message.feedback" => Event::RoomMessageFeedback(event),
    "m.room.name" => Event::RoomName(event),
    "m.room.pinned_events" => Event::PinnedEvents(event),
    "m.room.power_levels" => Event::RoomPowerLevels(event),
    "m.room.redaction" => Event::RoomRedaction(event),
    "m.room.server_acl" => Event::RoomServerAcl(event),
    "m.room.third_party_invite" => Event::RoomThirdPartyInvite(event),
    "m.room.topic" => Event::RoomTopic(event),
    "m.room_key" => Event::RoomKey(event),
    "m.room_key_request" => Event::RoomKeyRequest(event),
    "m.sticker" => Event::Sticker(event),
    "m.tag" => Event::Tag(event),
    "m.typing" => Event::Typing(event),
    _ => Event::NotHandled
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json;

  impl PartialEq for EventContent {
    fn eq(&self, other: &EventContent) -> bool {
      self.body == other.body &&
        self.msgtype == other.msgtype
    }
  }

  #[test]
  fn test_msg_text() {
    let text_msg = r#"{
      "type": "m.room.message",
      "content": {
        "body": "Hello world!",
        "msgtype": "m.text"
      },
      "sender": "@bot:my.domain.co",
      "event_id": "$12345:my.domain.co",
      "origin_server_ts": 1540293504937
    }"#;

    let e: RawEvent = serde_json::from_str(text_msg).unwrap();
    assert_eq!(e.type_, "m.room.message");
    assert_eq!(e.content.clone().unwrap(), EventContent {
      body: "Hello world!".to_string(),
      msgtype: "m.text".to_string()
    });
    assert_eq!(e.sender.unwrap(), "@bot:my.domain.co");
    assert_eq!(e.event_id.unwrap(), "$12345:my.domain.co");
    assert_eq!(e.origin_server_ts.unwrap(), 1540293504937);
  }
}

