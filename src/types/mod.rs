use serde::{Deserialize, Serialize};

pub mod responses;
pub mod request;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub user_token: Option<String>,
  pub url: String
}
