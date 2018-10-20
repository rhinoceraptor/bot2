#![recursion_limit = "1024"]

extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate toml;
extern crate percent_encoding;
#[macro_use]
extern crate error_chain;
extern crate ctrlc;

pub mod client;
pub mod config;
pub mod bot;
pub mod room;

use bot::{Bot};

mod errors {
  error_chain! {}
}

fn main() {
  let file = config::read_config_file("/home/jack/git/bot/config.toml")
    .expect("Unable to read file!");
  let config: config::Config = toml::from_str(&file)
    .expect("Unable to parse config!");

  let mut bot = Bot::new(config)
    .expect("Bot failed to initialize");
  bot.init()
    .expect("Failed to set up bot");
}

