use std::convert::From;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CurrencyEnum {
  USD,
  UYU,
  EUR,
}

impl From<CurrencyEnum> for String {
  fn from(currency: CurrencyEnum) -> Self {
    match currency {
      CurrencyEnum::USD => "USD".to_string(),
      CurrencyEnum::UYU => "UYU".to_string(),
      CurrencyEnum::EUR => "EUR".to_string(),
    }
  }
}
