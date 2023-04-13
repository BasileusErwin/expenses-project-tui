use serde::{Serialize, Deserialize};

use crate::enums::{month::MonthEnum, currency::CurrencyEnum, transaction_type::TransactionType};

use super::user::UserModel;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategoryModel {
  pub category_id: String,
  pub r#type: TransactionType,
  pub name: String,
  pub note: Option<String>,
  pub user_id: String,
  pub user: Option<UserModel>,
}
