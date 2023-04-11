use std::error::Error;
use std::fmt::*;

use crate::types::responses::response::ShowMessage;

#[derive(Debug)]
pub struct CustomError<T: Debug> {
  data: Option<T>,
  message: Option<String>,
  show_message: Option<ShowMessage>,
}

impl<T: Debug> CustomError<T> {
  pub fn new(
    data: Option<T>,
    message: Option<String>,
    show_message: Option<ShowMessage>,
  ) -> CustomError<T> {
    CustomError {
      data,
      message,
      show_message,
    }
  }
}

impl<T: Debug> Display for CustomError<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    match &self.show_message {
      Some(show_message) => write!(f, "{:?}", show_message.en),
      None => match &self.message {
        Some(data) => write!(f, "{:?}", data),
        None => write!(f, "{:?}", "Generic error")
      }
    }
  }
}

impl<T: Debug> Error for CustomError<T> {}
