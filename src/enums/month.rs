use std::convert::From;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl MonthEnum {
  pub fn from_u32(month: u32) -> Option<Self> {
    match month {
      1 => Some(MonthEnum::JANUARY),
      2 => Some(MonthEnum::FEBRUARY),
      3 => Some(MonthEnum::MARCH),
      4 => Some(MonthEnum::APRIL),
      5 => Some(MonthEnum::MAY),
      6 => Some(MonthEnum::JUNE),
      7 => Some(MonthEnum::JULY),
      8 => Some(MonthEnum::AUGUST),
      9 => Some(MonthEnum::SEPTEMBER),
      10 => Some(MonthEnum::OCTOBER),
      11 => Some(MonthEnum::NOVEMBER),
      12 => Some(MonthEnum::DECEMBER),
      _ => None,
    }
  }

  pub fn from_string(month: String) -> Option<Self> {
    match month.as_str() {
      "January" => Some(MonthEnum::JANUARY),
      "FEBRUARY" => Some(MonthEnum::FEBRUARY),
      "MARCH" => Some(MonthEnum::MARCH),
      "APRIL" => Some(MonthEnum::APRIL),
      "MAY" => Some(MonthEnum::MAY),
      "JUNE" => Some(MonthEnum::JUNE),
      "JULY" => Some(MonthEnum::JULY),
      "AUGUST" => Some(MonthEnum::AUGUST),
      "SEPTEMBER" => Some(MonthEnum::SEPTEMBER),
      "OCTOBER" => Some(MonthEnum::OCTOBER),
      "NOVEMBER" => Some(MonthEnum::NOVEMBER),
      "DECEMBER" => Some(MonthEnum::DECEMBER),
      _ => None,
    }
  }
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
