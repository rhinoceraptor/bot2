#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate http;

use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Config {
  channels: Vec<String>,
  authentication: Authentication,
}

#[derive(Deserialize, Debug)]
struct Authentication {
  username: String,
  password: String,
  home_server_url: String,
  identity_server_url: String,
}

fn read_config_file (filename: &str) -> Result<String, Box<Error>> {
  let mut buf = String::new();
  let mut f = File::open(filename)?;
  f.read_to_string(&mut buf)?;
  Ok(buf)
}

fn main() {
  let file = read_config_file("/home/jack/git/bot/config.toml")
    .expect("Unable to read file!");
  let config: Config = toml::from_str(&file)
    .expect("Unable to parse config!");

  println!("{:#?}", config);
}

