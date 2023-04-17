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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTotalSaving {
  pub total_savings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthByYear {
  pub year: String,
  pub months: Vec<String>,
}

