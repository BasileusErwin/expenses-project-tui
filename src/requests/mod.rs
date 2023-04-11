use reqwest::Url;

pub mod health;
pub mod auth;
pub mod user;
pub mod transaction;

pub fn get_url(endpount: &str) -> Url {
  let mut base_url = Url::parse("http://localhost:3000/api").unwrap();

  base_url.set_path((base_url.path().to_owned() + endpount).as_str());

  base_url
}
