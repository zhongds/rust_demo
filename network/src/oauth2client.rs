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
  inner: Arc<Inner>,
}

struct Inner {
  config: Config,
  is_refresh_token: Mutex<bool>,

  // storage credentials key
  credentials_key: Mutex<String>,
  // 内存的credentials
  credentials: Mutex<Option<Credentials>>,
}

pub static mut GLOBAL_CLIENT: Option<Oauth2client> = None;
static START: Once = Once::new();

impl Inner {

  fn refresh_token(&self, refresh_token: String) {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let mut map = Map::new();
    map.insert("client_id".to_string(), Value::String(self.config.client_id.to_string()));
    map.insert("grant_type".to_string(), Value::String("refresh_token".to_string()));
    map.insert("refresh_token".to_string(), Value::String(refresh_token));
    let opts = Options {
      method: reqwest::Method::POST,
      headers: Some(headers),
      body: Some(map),
      is_with_credentials: false,
    };


    let credentials: Credentials = self.requestWithNoToken("/v1/auth/token".to_string(), opts).unwrap();

    println!("======: {:#?}", credentials);
  }

  // 
  #[tokio::main]
  async fn requestWithNoToken<T: DeserializeOwned>(&self, url: String, opts: Options) -> reqwest::Result<T> {
    let api_origin = self.config.api_origin.clone();
    let newUrl = api_origin + &url;
    // 获取token
    let mut headers = opts.headers.unwrap_or(reqwest::header::HeaderMap::new());

    if !headers.contains_key("Content-Type") && !headers.contains_key("content-type") {
      headers.insert("Content-Type", "application/json".parse().unwrap());
    }

    let body = opts.body.unwrap_or(Map::new());

    let client = reqwest::Client::new();
    let res = client.request(reqwest::Method::GET, &url)
      .headers(headers)
      .json(&body)
      .send()
      .await?
      .json::<T>()
      .await?;
    Ok(res)
  }
}

impl Oauth2client {

  pub fn init(config: Config) {
    START.call_once(|| unsafe {
      match &GLOBAL_CLIENT {
        Some(_) => {},
        None => {
          let mut cred_key = "credentials_".to_owned();
          let client_id = config.client_id.clone();
          cred_key.push_str(&client_id);
          // 初始化credentials
          let mut cred: Option<Credentials> = None;

          GLOBAL_CLIENT = Some(Oauth2client{
            inner: Arc::new(Inner{
              config: config,
              is_refresh_token: Mutex::new(false),
              credentials_key: Mutex::new(cred_key),
              credentials: Mutex::new(cred),
            })
          });
        }
      }
    })
  }

  pub fn get_instance() -> &'static Oauth2client {
    unsafe {
      GLOBAL_CLIENT.as_ref().expect("1111")
    }
  }
  // 同步 -> 异步
  #[tokio::main]
  pub async fn request<T: DeserializeOwned>(&self, url: String, opts: Options) -> reqwest::Result<T> {
    let api_origin = self.inner.config.api_origin.clone();
    let newUrl = api_origin + &url;
    // 获取token
    let mut headers = opts.headers.unwrap_or(reqwest::header::HeaderMap::new());
    if opts.is_with_credentials {
      let local_self = self.inner.clone();

      let handle = thread::spawn(move || {
        local_self.refresh_token(String::from("refresh_token"));
      });

      handle.join().unwrap();

      // let token: String = self.get_access_token().unwrap_or("".to_string());
      let token = String::from("1111");
      if token == "" {
        panic!("token  is not found");
      }
      let mut auth_token = String::from("Bearer ");
      auth_token.push_str(&token);
      headers.insert("Authorization", auth_token.parse().unwrap());
    }

    if !headers.contains_key("Content-Type") && !headers.contains_key("content-type") {
      headers.insert("Content-Type", "application/json".parse().unwrap());
    }

    let body = opts.body.unwrap_or(Map::new());

    let client = reqwest::Client::new();
    let res = client.request(opts.method, &newUrl)
      .headers(headers)
      .json(&body)
      .send()
      .await?
      .json::<T>()
      .await?;

    Ok(res)
  }
}
