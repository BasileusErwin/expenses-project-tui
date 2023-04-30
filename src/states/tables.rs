use tui::widgets::TableState;

use crate::{enums::selected_block::SelectedBlock, models::transaction::TransactionModel};

#[derive(Debug)]
pub enum SortMode {
  ASC,
  DESC,
}

#[derive(Debug)]
pub struct CustomTableState {
  pub state: TableState,
  pub items: Vec<Vec<String>>,
}

pub struct TransactionsTableState {
  pub incomes: CustomTableState,
  pub expenses: CustomTableState,
  pub savings: CustomTableState,
  pub row_expenses: Vec<TransactionModel>,
  pub row_incomes: Vec<TransactionModel>,
  pub row_savings: Vec<TransactionModel>,
  pub transactions_details: Option<TransactionModel>,
}

impl TransactionsTableState {
  pub fn new(
    incomes: CustomTableState,
    expenses: CustomTableState,
    savings: CustomTableState,
    row_expenses: Vec<TransactionModel>,
    row_incomes: Vec<TransactionModel>,
    row_savings: Vec<TransactionModel>,
  ) -> Self {
    Self {
      incomes,
      expenses,
      savings,
      row_expenses,
      row_incomes,
      row_savings,
      transactions_details: None,
    }
  }

  pub fn next(&mut self, selected_block: &SelectedBlock) {
    match selected_block {
      SelectedBlock::Incomes => self
        .incomes
        .state
        .select(match self.incomes.state.selected() {
          Some(i) => {
            if i >= self.incomes.items.len() - 1 {
              Some(0)
            } else {
              Some(i + 1)
            }
          }
          None => Some(0),
        }),
      SelectedBlock::Savings => self
        .savings
        .state
        .select(match self.savings.state.selected() {
          Some(i) => {
            if i >= self.savings.items.len() - 1 {
              Some(0)
            } else {
              Some(i + 1)
            }
          }
          None => Some(0),
        }),
      SelectedBlock::Expenses => self
        .expenses
        .state
        .select(match self.expenses.state.selected() {
          Some(i) => {
            if i >= self.expenses.items.len() - 1 {
              Some(0)
            } else {
              Some(i + 1)
            }
          }
          None => Some(0),
        }),
      _ => {}
    };
  }

  pub fn previous(&mut self, selected_block: &SelectedBlock) {
    match selected_block {
      SelectedBlock::Incomes => self
        .incomes
        .state
        .select(match self.incomes.state.selected() {
          Some(i) => {
            if i == 0 {
              Some(self.incomes.items.len() - 1)
            } else {
              Some(i - 1)
            }
          }
          None => Some(0),
        }),
      SelectedBlock::Savings => self
        .savings
        .state
        .select(match self.savings.state.selected() {
          Some(i) => {
            if i == 0 {
              Some(self.savings.items.len() - 1)
            } else {
              Some(i - 1)
            }
          }
          None => Some(0),
        }),
      SelectedBlock::Expenses => self
        .expenses
        .state
        .select(match self.expenses.state.selected() {
          Some(i) => {
            if i == 0 {
              Some(self.expenses.items.len() - 1)
            } else {
              Some(i - 1)
            }
          }
          None => Some(0),
        }),
      _ => {}
    };
  }
}
