use std::error::Error;
use reqwest::{Url, StatusCode};

use crate::{
  models::{custom_error::CustomError, user::UserModel},
  types::{
    responses::response::CustomResponse,
    request::user::CreateUserRequest,
  },
};

use super::get_url;

pub async fn create_user(
  client: &reqwest::Client,
  new_user: CreateUserRequest,
) -> Result<UserModel, Box<dyn Error>> {
  let url: Url = get_url("/users");

  let response = client
    .post(url)
    .json::<CreateUserRequest>(&new_user)
    .send()
    .await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to create user")),
      None,
    )));
  }

  if response.status() != StatusCode::OK {
    let body = response.json::<CustomResponse<String>>().await?;

    return Err(Box::new(CustomError::new(
      if body.data.is_some() {
        Some(body.data)
      } else {
        None
      },
      if body.message.is_some() {
        Some(body.message.unwrap())
      } else {
        None
      },
      if body.show_message.is_some() {
        Some(body.show_message.unwrap())
      } else {
        None
      },
    )));
  }

  let data: CustomResponse<UserModel> = response.json::<CustomResponse<UserModel>>().await?;

  Ok(data.data.unwrap())
}
