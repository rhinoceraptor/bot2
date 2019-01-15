use std::error::Error;

pub type ClientResult<T> = Result<T, Box<Error>>;

pub trait Client {
  fn new<T>(auth: T) -> Self;
  fn login(&mut self) -> ClientResult<()>;
  fn logout(&self) -> ClientResult<()>;
  fn join_room(&self, room: &String) -> ClientResult<()>;
  fn leave_room(&self, room: &String) -> ClientResult<()>;
  fn get_presence(&self, user: &String) -> ClientResult<bool>;
  fn send_message(&self, room: &String, message: &String) -> ClientResult<()>;
}

