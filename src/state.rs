use std::{vec, result};
use crate::{
  enums::selected_block::{SelectedBlock, self},
  requests::transaction::{MonthByYear, get_transactions_balances},
  utils::{*, transaction_utils::sort},
  models::transaction::TransactionModel,
  types::responses::transaction::TransactionBalances,
  ui::get_transactions,
};
use crossterm::event::{KeyCode, ModifierKeyCode};
use tui::widgets::TableState;
use tui_tree_widget::TreeItem;

#[derive(Debug)]
pub enum SortMode {
  ASC,
  DESC,
}

#[derive(Debug)]
pub struct TransactionsTableState {
  pub state: TableState,
  pub items: Vec<Vec<String>>,
}

pub struct App<'a> {
  pub tree: StatefulTree<'a>,
  pub selected_block: SelectedBlock,
  pub user_token: String,
  pub is_navigation: bool,
  pub incomes_table: TransactionsTableState,
  pub expenses_table: TransactionsTableState,
  pub savings_table: TransactionsTableState,
  pub transactions_header: Vec<&'static str>,
  pub summary: TransactionBalances,
  pub months_by_year: Vec<MonthByYear>,
  pub client: reqwest::Client,
}

pub struct DataTable<'a> {
  pub months_by_year: Vec<MonthByYear>,
  pub expenses: Vec<TransactionModel>,
  pub incomes: Vec<TransactionModel>,
  pub savings: Vec<TransactionModel>,
  pub summary: TransactionBalances,
  pub tree: StatefulTree<'a>,
}

fn get_transactions_row(transaction: Vec<TransactionModel>) -> Vec<Vec<String>> {
  transaction
    .iter()
    .map(|expense: &TransactionModel| {
      vec![
        match expense.day {
          Some(day) => day.to_string(),
          None => "0".to_string(),
        },
        format!("${:?} {:?}", expense.amount, expense.currency),
        match &expense.note {
          Some(note) => note.to_string(),
          None => "".to_string(),
        },
        // match &expense.category {
        //   Some(category) => {
        //     if category.name.len() > 15 {
        //       wrap_text(&category.name, 15)
        //     } else {
        //       category.name.to_string()
        //     }
        //   }
        //   None => "".to_string(),
        // },
      ]
    })
    .collect()
}

impl<'a> App<'a> {
  pub fn new(data_table: DataTable<'a>, user_token: String, client: reqwest::Client) -> App<'a> {
    let expenses_table = TransactionsTableState {
      state: TableState::default(),
      items: get_transactions_row(data_table.expenses),
    };

    let incomes_table = TransactionsTableState {
      state: TableState::default(),
      items: get_transactions_row(data_table.incomes),
    };

    let savings_table = TransactionsTableState {
      state: TableState::default(),
      items: get_transactions_row(data_table.savings),
    };

    Self {
      client,
      months_by_year: data_table.months_by_year,
      tree: data_table.tree,
      selected_block: SelectedBlock::Months,
      user_token,
      is_navigation: true,
      incomes_table,
      expenses_table,
      savings_table,
      transactions_header: vec!["Day", "Amount", "Note"],
      summary: data_table.summary,
    }
  }

  pub fn next_table_item(&mut self) {
    match self.selected_block {
      SelectedBlock::Incomes => {
        self
          .incomes_table
          .state
          .select(match self.incomes_table.state.selected() {
            Some(i) => {
              if i >= self.incomes_table.items.len() - 1 {
                Some(0)
              } else {
                Some(i + 1)
              }
            }
            None => Some(0),
          })
      }
      SelectedBlock::Savings => {
        self
          .savings_table
          .state
          .select(match self.savings_table.state.selected() {
            Some(i) => {
              if i >= self.savings_table.items.len() - 1 {
                Some(0)
              } else {
                Some(i + 1)
              }
            }
            None => Some(0),
          })
      }
      SelectedBlock::Expenses => {
        self
          .expenses_table
          .state
          .select(match self.expenses_table.state.selected() {
            Some(i) => {
              if i >= self.expenses_table.items.len() - 1 {
                Some(0)
              } else {
                Some(i + 1)
              }
            }
            None => Some(0),
          })
      }
      _ => {}
    };
  }

  pub fn previous_table_item(&mut self) {
    match self.selected_block {
      SelectedBlock::Incomes => {
        self
          .incomes_table
          .state
          .select(match self.incomes_table.state.selected() {
            Some(i) => {
              if i == 0 {
                Some(self.incomes_table.items.len() - 1)
              } else {
                Some(i - 1)
              }
            }
            None => Some(0),
          })
      }
      SelectedBlock::Savings => {
        self
          .savings_table
          .state
          .select(match self.savings_table.state.selected() {
            Some(i) => {
              if i == 0 {
                Some(self.savings_table.items.len() - 1)
              } else {
                Some(i - 1)
              }
            }
            None => Some(0),
          })
      }
      SelectedBlock::Expenses => {
        self
          .expenses_table
          .state
          .select(match self.expenses_table.state.selected() {
            Some(i) => {
              if i == 0 {
                Some(self.expenses_table.items.len() - 1)
              } else {
                Some(i - 1)
              }
            }
            None => Some(0),
          })
      }
      _ => {}
    };
  }

  fn select_next_block(&mut self) {
    self.selected_block = match self.selected_block {
      SelectedBlock::Months => SelectedBlock::Incomes,
      SelectedBlock::Incomes => SelectedBlock::Expenses,
      SelectedBlock::Expenses => SelectedBlock::Savings,
      SelectedBlock::Savings => SelectedBlock::Months,
    };
  }

  fn select_previous_block(&mut self) {
    self.selected_block = match self.selected_block {
      SelectedBlock::Months => SelectedBlock::Savings,
      SelectedBlock::Incomes => SelectedBlock::Months,
      SelectedBlock::Expenses => SelectedBlock::Incomes,
      SelectedBlock::Savings => SelectedBlock::Expenses,
    };
  }

  pub async fn process_key_event(&mut self, key_code: KeyCode) {
    match key_code {
      KeyCode::Char('j') => {
        if self.is_navigation {
          self.select_next_block()
        }
      }
      KeyCode::Char('k') => {
        if self.is_navigation {
          self.select_previous_block()
        }
      }
      KeyCode::Enter => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.toggle(&self.months_by_year);

          let transactions = get_transactions(&self.client, &self.user_token, &self.tree).await;

          match transactions {
            Ok(mut result) => {
              transaction_utils::sort(&mut result.0);
              transaction_utils::sort(&mut result.1);
              transaction_utils::sort(&mut result.2);

              self.expenses_table = TransactionsTableState {
                state: self.expenses_table.state.clone(),
                items: get_transactions_row(result.0),
              };
              self.incomes_table = TransactionsTableState {
                state: self.incomes_table.state.clone(),
                items: get_transactions_row(result.1),
              };
              self.savings_table = TransactionsTableState {
                state: self.incomes_table.state.clone(),
                items: get_transactions_row(result.2),
              };

              match get_transactions_balances(
                &self.client,
                &self.user_token,
                self.tree.current_month.clone(),
              )
              .await
              {
                Ok(data) => self.summary = data,
                Err(_) => println!("errr"),
              };
            }
            Err(_) => println!("error"),
          }
        }
      }
      KeyCode::Left => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.left()
        }
      }
      KeyCode::Right => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.right()
        }
      }
      KeyCode::Down => match self.selected_block {
        SelectedBlock::Months => self.tree.down(),
        SelectedBlock::Incomes | SelectedBlock::Expenses | SelectedBlock::Savings => {
          self.next_table_item()
        }
      },
      KeyCode::Up => match self.selected_block {
        SelectedBlock::Months => self.tree.up(),
        SelectedBlock::Incomes | SelectedBlock::Expenses | SelectedBlock::Savings => {
          self.previous_table_item()
        }
      },
      KeyCode::Char('m') => self.selected_block = SelectedBlock::Months,
      KeyCode::Char('e') => self.selected_block = SelectedBlock::Expenses,
      KeyCode::Char('i') => self.selected_block = SelectedBlock::Incomes,
      KeyCode::Char('s') => self.selected_block = SelectedBlock::Savings,
      _ => {}
    }
  }
}
