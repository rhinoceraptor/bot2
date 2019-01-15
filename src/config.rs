use std::io::prelude::*;
use std::fs::File;
use failure::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub bot_config: BotConfig,
  pub authentication: Authentication,
}

#[derive(Deserialize, Debug)]
pub struct BotConfig {
  pub rooms: Vec<String>,
  pub server_domain: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Authentication {
  pub user: String,
  pub password: String,
  pub server_url: String,
}

pub fn read_config_file (filename: &str) -> Result<String, Error> {
  let mut buf = String::new();
  let mut f = File::open(filename)?;
  f.read_to_string(&mut buf)?;
  Ok(buf)
}

