use serde::{Deserialize, Serialize};

pub mod request;
pub mod responses;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub session_id: Option<String>,
  pub url: String,
}
