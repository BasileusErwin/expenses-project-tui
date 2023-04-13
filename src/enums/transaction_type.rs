use std::convert::From;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
  INCOME,
  EXPENSE,
  SAVING,
  INSTALLMENTS,
}

impl From<TransactionType> for String {
  fn from(transaction_type: TransactionType) -> Self {
    match transaction_type {
      TransactionType::INCOME => "INCOME".to_string(),
      TransactionType::EXPENSE => "EXPENSE".to_string(),
      TransactionType::SAVING => "SAVING".to_string(),
      TransactionType::INSTALLMENTS => "INSTALLMENTS".to_string(),
    }
  }
}
