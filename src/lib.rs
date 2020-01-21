#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

extern crate base64;
extern crate chrono;
extern crate dirs;
extern crate http;
extern crate openssl;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate time;
extern crate url;

pub mod client;
pub mod config;
mod oauth2;

pub type Result<T> = std::result::Result<T, failure::Error>;
