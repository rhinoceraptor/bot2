use std::{thread, time};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use ctrlc::set_handler as set_ctrlc_handler;
use config::{BotConfig};
use std::error::Error;
use client::Client;

type BotResult<T> = Result<T, Box<Error>>;

#[derive(Debug)]
pub struct Bot {
  config: BotConfig,
  room_list: Vec<String>,
  client: Client,
}

impl Bot {
  pub fn new(config: BotConfig, client: impl Client) -> BotResult<Bot> {
    Ok(Bot { config, room_list: Vec::new(), client })
  }

  fn encode_room(&self, room: &String) -> String {
    let BotConfig { server_domain, .. } = &self.config;
    let r = utf8_percent_encode(&room, DEFAULT_ENCODE_SET).to_string();
    format!("{}:{}", r, server_domain)
  }

  pub fn run(mut self) -> BotResult<()> {
    for room in self.room_list {
      match self.client.join_room(room) {
        Ok(()) => println!("Joined {}", room),
        Err(e) => panic!("Unable to join room {}", room)
      }
    }

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
      let timeout = time::Duration::from_millis(1000);
      thread::sleep(timeout);
    }

    self.destroy()?;

    Ok(())
  }

  pub fn poll(&mut self) -> BotResult<()> {
    self.client.sync()?;
    Ok(())
  }

  pub fn destroy(&mut self) -> BotResult<()> {
    for room in self.room_list.iter() {
      self.client.leave_room(room);
    }

    self.client.logout()?;

    Ok(())
  }
}

