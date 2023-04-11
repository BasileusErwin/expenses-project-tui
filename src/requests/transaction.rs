use std::collections::HashMap;
use std::error::Error;
use reqwest::{
  Url, StatusCode,
  header::{HeaderValue, COOKIE, HeaderMap},
};
use serde::{Serialize, Deserialize};

use crate::{models::custom_error::CustomError, types::responses::response::CustomResponse};

use super::get_url;

pub type MonthByYearMap = HashMap<String, Vec<String>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthByYear {
  pub year: String,
  pub months: Vec<String>,
}

pub async fn get_month_by_year(
  client: &reqwest::Client,
  user_token: String,
) -> Result<Vec<MonthByYear>, Box<dyn Error>> {
  let url: Url = get_url("/transactions/month-by-years");

  let cookie_header = HeaderValue::from_str(&format!("token={}", user_token))?;
  let mut header = HeaderMap::new();
  header.insert(COOKIE, cookie_header);

  let response = client.get(url).headers(header).send().await?;

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

  let data: CustomResponse<MonthByYearMap> =
    response.json::<CustomResponse<MonthByYearMap>>().await?;

  let month_by_year: Vec<MonthByYear> = data
    .data
    .unwrap()
    .into_iter()
    .map(|(year, months)| MonthByYear { year, months })
    .collect();

  return Ok(month_by_year);
}
