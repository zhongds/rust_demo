#![cfg(target_os="android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;
extern crate android_logger;

use log::Level;
use log::{info, trace, warn};
use android_logger::{Config as LoggerConfig,FilterBuilder};

mod models;
mod oauth2client;
pub use models::{Options, Credentials, Config};
pub use oauth2client::{Oauth2client};

use std::ffi::{CString, CStr};
use serde_json;
use serde_json::{Map, Value};
use reqwest;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::{JNIEnv};
use jni::errors::Result;

// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString, JObject};

// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::jstring;

#[no_mangle]
pub unsafe extern fn Java_com_xunlei_acc_sdk_MainActivity_init(env: JNIEnv, _: JObject, confStr: JString) {

    android_logger::init_once(
				LoggerConfig::default().with_min_level(Level::Trace));
				
    let confStr: String = env.get_string(confStr).expect("Couldn't get java string!").into();
    let conf: Config = serde_json::from_str(&confStr).expect("parse to Config error");
		Oauth2client::init(conf);
}

// #[no_mangle]
// pub unsafe extern fn Java_com_xunlei_acc_sdk_MainActivity_getAccessToken(env: JNIEnv, _: JClass) -> jstring {
//     android_logger::init_once(
//         LoggerConfig::default().with_min_level(Level::Trace));

// 	let client = Oauth2client::get_instance();
// 	let token = client.get_access_token().expect("get token err");
// 	let output = env.new_string(token).expect("get token error");
// 	output.into_inner()
// }

#[no_mangle]
pub unsafe extern fn Java_com_xunlei_acc_sdk_MainActivity_request(env: JNIEnv, _: JClass, url_str: JString) -> jstring {
    let client = Oauth2client::get_instance();
    let url: String = env.get_string(url_str).expect("get url err").into();
    let opts = Options {
        method: reqwest::Method::GET,
        is_with_credentials: true,
        headers: None,
        body: None,
    };
    let res: Map<String, Value> = client.request(url, opts).unwrap();
    
    let res_str = serde_json::to_string(&res).expect("result parse err");
    info!("result: {}", res_str);
	let output = env.new_string(res_str).expect("get token error");
    output.into_inner()
}