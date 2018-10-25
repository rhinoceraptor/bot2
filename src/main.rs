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

pub mod matrix;
pub mod config;
pub mod bot;
pub mod room;

use matrix::client::MatrixClient;
use bot::Bot;

mod errors {
  error_chain! {}
}

error_chain! {}

fn main() {
  let file = config::read_config_file("/home/jack/git/bot/config.toml")
    .expect("Unable to read file!");
  let config: config::Config = toml::from_str(&file)
    .expect("Unable to parse config!");

  let auth = config.authentication;
  let mut matrix_client = MatrixClient::new(auth);

  matrix_client
    .login()
    .chain_err(|| "Failed to login bot")
    .expect("Matrix client initialization failed!");

  let bot = Bot::new(config.bot_config, &mut matrix_client)
    .expect("Bot failed to initialize!");
  bot.init()
    .expect("Failed to set up bot!");
}

