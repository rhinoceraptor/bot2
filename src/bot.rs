use std::error::Error;
use config::{Config};
use reqwest;

#[derive(Deserialize, Debug)]
struct AccessToken {
  access_token: String,
}

pub struct Bot {
  config: Config,
  auth_token: Option<String>,
}

impl Bot {
  pub fn new(config: Config) -> Self {
    Bot { config, auth_token: None }
  }

  pub fn init(&mut self) -> Result<(), Box<Error>> {
    self.auth_token = Some(self.authenticate()?);
    Ok(())
  }

  fn authenticate(&self) -> Result<String, Box<Error>> {
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
    let AccessToken { access_token } = client
      .post(&uri)
      .json(&body)
      .send()?
      .json()?;

    Ok(access_token)
  }
}

