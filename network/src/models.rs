use reqwest;
use chrono::prelude::{Utc};
use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub api_origin: String,
  pub client_id: String,
}

#[derive(Clone, Debug)]
pub struct Options {
  pub method: reqwest::Method,
  pub is_with_credentials: bool,
  pub headers: Option<reqwest::header::HeaderMap>,
  pub body: Option<Map<String, Value>>,
}

#[derive(Debug)]
pub struct RequestEnv {
  url: String,
  opts: Options,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
  pub access_token: String,
  pub refresh_token: String,
  pub token_type: String,
  pub expires_in: i64,
  pub expires_at: Option<i64>,
  pub sub: String,
}

impl Credentials {
  pub fn init_expires_at(&mut self) {
    match self.expires_at {
      None => {
        self.expires_at = Some(Utc::now().timestamp() + self.expires_in - 30);
      },
      Some(_) => {},
    }
  }
  pub fn is_expired(&self) -> bool {
    if self.access_token == "" {
      return true;
    }
    match self.expires_at {
      None => true,
      Some(v) => {
        Utc::now().timestamp() > v
      },
    }
  }
}
