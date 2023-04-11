use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
  pub user_id: String,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub password: String,
}

