use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub rooms: Vec<String>,
  pub authentication: Authentication,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Authentication {
  pub user: String,
  pub password: String,
  pub server_url: String,
  pub server_domain: String,
}

pub fn read_config_file (filename: &str) -> Result<String, Box<Error>> {
  let mut buf = String::new();
  let mut f = File::open(filename)?;
  f.read_to_string(&mut buf)?;
  Ok(buf)
}

