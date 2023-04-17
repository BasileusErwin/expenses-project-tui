use serde::{Serialize, Deserialize};
use crate::enums::{month::MonthEnum, currency::CurrencyEnum, transaction_type::TransactionType};
use super::{user::UserModel, category::CategoryModel};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionModel {
  pub transaction_id: String,
  pub r#type: TransactionType,
  pub amount: f64,
  pub currency: CurrencyEnum,
  pub note: Option<String>,
  pub day: Option<u8>,
  pub month: MonthEnum,
  pub year: u16,
  pub exchange_rate: Option<f64>,
  pub user_id: String,
  pub user: Option<UserModel>,
  pub category_id: String,
  pub category: Option<CategoryModel>,
}
