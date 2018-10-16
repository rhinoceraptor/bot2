extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate reqwest;
extern crate toml;

pub mod client;
pub mod structures;
pub mod config;
pub mod bot;

use bot::{Bot};

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

