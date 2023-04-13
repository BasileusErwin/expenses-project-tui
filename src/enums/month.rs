use std::convert::From;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MonthEnum {
  JANUARY,
  FEBRUARY,
  MARCH,
  APRIL,
  MAY,
  JUNE,
  JULY,
  AUGUST,
  SEPTEMBER,
  OCTOBER,
  NOVEMBER,
  DECEMBER,
}

impl From<MonthEnum> for String {
  fn from(month: MonthEnum) -> Self {
    match month {
      MonthEnum::JANUARY => "JANUARY".to_string(),
      MonthEnum::FEBRUARY => "FEBRUARY".to_string(),
      MonthEnum::MARCH => "MARCH".to_string(),
      MonthEnum::APRIL => "APRIL".to_string(),
      MonthEnum::MAY => "MAY".to_string(),
      MonthEnum::JUNE => "JUNE".to_string(),
      MonthEnum::JULY => "JULY".to_string(),
      MonthEnum::AUGUST => "AUGUST".to_string(),
      MonthEnum::SEPTEMBER => "SEPTEMBER".to_string(),
      MonthEnum::OCTOBER => "OCTOBER".to_string(),
      MonthEnum::NOVEMBER => "NOVEMBER".to_string(),
      MonthEnum::DECEMBER => "DECEMBER".to_string(),
    }
  }
}
