use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ShowMessage {
  pub en: String,
  pub es: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomResponse<T> {
  pub data: Option<T>,
  pub result: bool,
  pub message: Option<String>,
  pub show_message: Option<ShowMessage>,
}
