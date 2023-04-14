use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionTotal {
  pub total: f64,
  pub uyu: f64,
  pub usd: f64,
  pub eur: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionBalances {
  pub incomes: TransactionTotal,
  pub expenses: TransactionTotal,
  pub savings: TransactionTotal,
}
