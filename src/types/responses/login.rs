use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;
use super::response::CustomResponse;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
  pub user: UserModel,
  pub token: String,
}
