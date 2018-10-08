use std::collections::HashMap;
use std::io::{self, Write};
use reqwest;
use config::{Config};

pub struct Bot {
  config: Config,
  auth_token: Option<String>,
}

pub fn create_bot (config: Config) -> Bot {
  Bot { config, auth_token: None }
}

impl Bot {
  pub fn init(&self) {
    self.authenticate();
  }

  fn authenticate(&self) {
    let uri = format!(
      "{}/_matrix/client/r0/login",
      self.config.authentication.home_server_url
    );

    let body = json!({
      "user": self.config.authentication.username,
      "password": self.config.authentication.password,
      "type": "m.login.password"
    });

    let client = reqwest::Client::new();
    let body = client.post(&uri).json(&body).send();

    println!("{:#?}", body);
  }
}

