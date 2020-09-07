use reqwest;
use std::string::String;
use serde::de::DeserializeOwned;
use serde_json::{Value, Map};
use serde_json;
use serde;
use std::sync::{Once, Arc, Mutex};
use std::thread;
use std::sync::atomic::AtomicBool;

use crate::models::{Config, Credentials, Options};

pub struct Oauth2client {
  pub name: String,
}

impl Oauth2client {
  #[tokio::main]
  pub async fn request<T: DeserializeOwned>(&self, url: String) -> reqwest::Result<T> {
    let client = reqwest::Client::new();
    let res = client.request(reqwest::Method::GET, &url)
      .send()
      .await?
      .json::<T>()
      .await?;

    Ok(res)
  }
}
