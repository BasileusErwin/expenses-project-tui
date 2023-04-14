use reqwest::Url;

use crate::load_config;

pub mod health;
pub mod auth;
pub mod transaction;

pub fn get_url(endpount: &str) -> Url {
  let url = load_config().unwrap().url;

  let mut base_url = Url::parse(&url).unwrap();

  base_url.set_path((base_url.path().to_owned() + endpount).as_str());

  base_url
}
