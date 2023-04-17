use std::collections::HashMap;
use std::error::Error;
use reqwest::{
  Url, StatusCode,
  header::{HeaderValue, COOKIE, HeaderMap},
};
use crate::{
  models::{custom_error::CustomError, transaction::TransactionModel},
  enums::{transaction_type::TransactionType, month::MonthEnum},
  types::responses::{
    response::CustomResponse,
    transaction::{TransactionBalances, MonthByYear, GetTotalSaving},
  },
};

use super::get_url;

pub type MonthByYearMap = HashMap<String, Vec<String>>;

pub async fn get_month_by_year(
  client: &reqwest::Client,
  user_token: &String,
) -> Result<Vec<MonthByYear>, Box<dyn Error>> {
  let url: Url = get_url("/transactions/month-by-years");

  let cookie_header = HeaderValue::from_str(&format!("token={}", user_token))?;
  let mut header = HeaderMap::new();
  header.insert(COOKIE, cookie_header);

  let response = client.get(url).headers(header).send().await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to get months")),
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

  Ok(month_by_year)
}

pub async fn get_transactions_by_month_and_type(
  client: &reqwest::Client,
  user_token: &String,
  transaction_type: TransactionType,
  month: MonthEnum,
) -> Result<Vec<TransactionModel>, Box<dyn Error>> {
  let url: Url = get_url("/transactions");

  let query: Vec<(&str, String)> = vec![
    ("type", String::from(transaction_type)),
    ("month", String::from(month)),
  ];

  let cookie_header = HeaderValue::from_str(&format!("token={}", user_token))?;
  let mut header = HeaderMap::new();
  header.insert(COOKIE, cookie_header);

  let response = client.get(url).headers(header).query(&query).send().await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to get transactions")),
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

  let data: CustomResponse<Vec<TransactionModel>> = response
    .json::<CustomResponse<Vec<TransactionModel>>>()
    .await?;

  Ok(data.data.unwrap().into_iter().collect())
}

pub async fn get_transactions_balances(
  client: &reqwest::Client,
  user_token: &String,
  month: MonthEnum,
) -> Result<TransactionBalances, Box<dyn Error>> {
  let url: Url = get_url("/transactions");

  let query: Vec<(&str, String)> = vec![
    ("balance", String::from("true")),
    ("month", String::from(month)),
  ];

  let cookie_header = HeaderValue::from_str(&format!("token={}", user_token))?;
  let mut header = HeaderMap::new();
  header.insert(COOKIE, cookie_header);

  let response = client.get(url).headers(header).query(&query).send().await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to get transactions")),
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

  let data: CustomResponse<TransactionBalances> = response
    .json::<CustomResponse<TransactionBalances>>()
    .await?;

  Ok(data.data.unwrap())
}

pub async fn get_total_saving(
  client: &reqwest::Client,
  user_token: &String,
) -> Result<f64, Box<dyn Error>> {
  let url: Url = get_url("/transactions/total-saving");

  let cookie_header = HeaderValue::from_str(&format!("token={}", user_token))?;
  let mut header = HeaderMap::new();
  header.insert(COOKIE, cookie_header);

  let response = client.get(url).headers(header).send().await?;

  if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
    return Err(Box::new(CustomError::new(
      Some(""),
      Some(String::from("Error to get transactions")),
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

  let data: CustomResponse<GetTotalSaving> =
    response.json::<CustomResponse<GetTotalSaving>>().await?;

  Ok(data.data.unwrap().total_savings.round())
}
