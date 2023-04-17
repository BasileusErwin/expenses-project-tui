use std::cmp::Ordering;

use crate::models::transaction::TransactionModel;

pub fn sort(transactions: &mut [TransactionModel]) {
  transactions.sort_by(|a, b| match (a.day, b.day) {
    (Some(a_day), Some(b_day)) => a_day.cmp(&b_day),
    (Some(_), None) => Ordering::Less,
    (None, Some(_)) => Ordering::Greater,
    (None, None) => Ordering::Equal,
  })
}
