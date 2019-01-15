use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use failure::Error;
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use ctrlc::set_handler as set_ctrlc_handler;
use config::{BotConfig};
use client::matrix_client::MatrixClient;
use room::{Room};

pub struct Bot<'a> {
  config: BotConfig,
  room_list: Vec<Room<'a>>,
  matrix_client: &'a MatrixClient,
}

impl<'a> Bot<'a> {
  pub fn new(config: BotConfig, matrix_client: &'a MatrixClient) -> Result<Bot<'a>, Error> {
    Ok(Bot { config, room_list: Vec::new(), matrix_client })
  }

  fn encode_room(&self, room: &String) -> String {
    let BotConfig { server_domain, .. } = &self.config;
    let r = utf8_percent_encode(&room, DEFAULT_ENCODE_SET).to_string();
    format!("{}:{}", r, server_domain)
  }

  pub fn run(mut self) -> Result<(), Error> {
    self.room_list = self.config.rooms
      .iter()
      .map(|room| match self.matrix_client.join_room(&self.encode_room(room)) {
        Ok(room_id) => Room::new(room.to_string(), room_id, &self.matrix_client),
        Err(e) => panic!("Unable to join {}: {}", room, e)
      }).collect();

    for room in self.room_list.iter() {
      room.send_message("wubba lubba dub dub".to_string())?;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_ctrlc_handler(move || {
      println!("\n\nShutting down...");
      r.store(false, Ordering::SeqCst);
    }).expect("Failed to set ctrl-c handler");

    while running.load(Ordering::SeqCst) {
      self.poll()?;
      let timeout = time::Duration::from_millis(100);
      thread::sleep(timeout);
    }

    self.destroy()?;

    Ok(())
  }

  pub fn poll(&self) -> Result<(), Error> {
    self.matrix_client.sync()?;
    Ok(())
  }

  pub fn destroy(&mut self) -> Result<(), Error> {
    for room in self.room_list.iter_mut() {
      room.destroy()?;
    }

    self.matrix_client.logout()?;

    Ok(())
  }
}

