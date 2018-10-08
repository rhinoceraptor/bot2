use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub channels: Vec<String>,
  pub authentication: Authentication,
}

#[derive(Deserialize, Debug)]
pub struct Authentication {
  pub username: String,
  pub password: String,
  pub home_server_url: String,
  pub identity_server_url: String,
}

pub fn read_config_file (filename: &str) -> Result<String, Box<Error>> {
  let mut buf = String::new();
  let mut f = File::open(filename)?;
  f.read_to_string(&mut buf)?;
  Ok(buf)
}

