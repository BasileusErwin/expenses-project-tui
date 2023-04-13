use std::error::Error;
use reqwest::{Url, StatusCode};

use crate::{
  models::custom_error::CustomError,
  types::responses::{
    login::{LoginRequest, LoginResponse},
    response::CustomResponse,
  },
};

use super::get_url;

pub async fn login(
  client: &reqwest::Client,
  login_body: LoginRequest,
) -> Result<LoginResponse, Box<dyn Error>> {
  let url: Url = get_url("/auth/login");

  let response = client
    .post(url)
    .json::<LoginRequest>(&login_body)
    .send()
    .await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to login")),
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

  let data: CustomResponse<LoginResponse> =
    response.json::<CustomResponse<LoginResponse>>().await?;

  Ok(data.data.unwrap())
}

