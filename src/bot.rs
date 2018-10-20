use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
// use ctrlc::set_handler as set_ctrlc_handler;
use config::{Authentication, Config};
use client::{MatrixClient};
use room::{Room};

error_chain! {}

pub struct Bot<'a> {
  config: Config,
  room_list: Vec<Room<'a>>,
  matrix_client: MatrixClient,
}

impl<'a> Bot<'a> {
  pub fn new(config: Config) -> Result<Bot<'a>> {
    let auth = config.authentication.clone();
    let matrix_client = MatrixClient::new(auth);

    matrix_client
      .login()
      .chain_err(|| "Failed to login bot")?;

    Ok(Bot { config, room_list: Vec::new(), matrix_client })
  }

  fn encode_room(&self, room: &String) -> String {
    let Authentication { server_domain, .. } = &self.config.authentication;
    let r = utf8_percent_encode(&room, DEFAULT_ENCODE_SET).to_string();
    format!("{}:{}", r, server_domain)
  }

  pub fn init(&mut self) -> Result<()> {
    self.room_list = self.config.rooms
      .iter()
      .map(|room| match self.matrix_client.join_room(&self.encode_room(room)) {
        Ok(room_id) => Room::new(room.to_string(), room_id, &self.matrix_client),
        Err(e) => panic!("Unable to join {}: {}", room, e)
      }).collect();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    //set_ctrlc_handler(|| {
    //  self.destroy().chain_err(|| "Failed to destroy bot");
    //  r.store(false, Ordering::SeqCst);
    //}).expect("Failed to set ctrl-c handler");

    while running.load(Ordering::SeqCst) {
      self.poll();
      let timeout = time::Duration::from_millis(100);
      thread::sleep(timeout);
    }

    Ok(())
  }

  pub fn poll(&self) -> Result<()> {
    self.matrix_client.sync().chain_err(|| "Failed to sync bot")?;
    Ok(())
  }

  pub fn destroy(&mut self) -> Result<()> {
    for room in self.room_list.iter_mut() {
      room.destroy().chain_err(|| "Failed to destroy room")?;
    }

    self.matrix_client.logout().chain_err(|| "Failed to logout bot")?;

    Ok(())
  }
}

