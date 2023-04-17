pub mod tables;
pub mod tabs;

use std::vec;
use crate::{
  enums::selected_block::SelectedBlock,
  requests::transaction,
  utils::*,
  models::transaction::TransactionModel,
  types::responses::transaction::{TransactionBalances, MonthByYear},
  ui::get_transactions,
  states::{
    tabs::TabsState,
    tables::{TransactionsTableState, CustomTableState},
  },
};
use crossterm::event::KeyCode;
use tui::widgets::TableState;

#[derive(Debug)]
pub enum SortMode {
  ASC,
  DESC,
}

pub struct App<'a> {
  pub tree: StatefulTree<'a>,
  pub selected_block: SelectedBlock,
  pub user_token: String,
  pub is_navigation: bool,
  pub transactions_header: Vec<&'static str>,
  pub table_state: TransactionsTableState,
  pub summary: TransactionBalances,
  pub months_by_year: Vec<MonthByYear>,
  pub client: reqwest::Client,
  pub tabs: TabsState<'a>,
  pub details_shown: bool,
  pub total_saving: f64,
}

pub struct DataTable<'a> {
  pub months_by_year: Vec<MonthByYear>,
  pub expenses: Vec<TransactionModel>,
  pub incomes: Vec<TransactionModel>,
  pub savings: Vec<TransactionModel>,
  pub summary: TransactionBalances,
  pub tree: StatefulTree<'a>,
  pub total_saving: f64,
}

fn get_transactions_row(transactions: &[TransactionModel]) -> Vec<Vec<String>> {
  transactions
    .iter()
    .map(|transaction: &TransactionModel| {
      vec![
        match transaction.day {
          Some(day) => day.to_string(),
          None => "".to_string(),
        },
        format!("${:?} {:?}", transaction.amount, transaction.currency),
        match &transaction.note {
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
    let expenses_table = CustomTableState {
      state: TableState::default(),
      items: get_transactions_row(&data_table.expenses),
    };

    let incomes_table = CustomTableState {
      state: TableState::default(),
      items: get_transactions_row(&data_table.incomes),
    };

    let savings_table = CustomTableState {
      state: TableState::default(),
      items: get_transactions_row(&data_table.savings),
    };

    Self {
      client,
      months_by_year: data_table.months_by_year,
      tree: data_table.tree,
      selected_block: SelectedBlock::Months,
      user_token,
      is_navigation: true,
      table_state: TransactionsTableState::new(
        incomes_table,
        expenses_table,
        savings_table,
        data_table.expenses,
        data_table.incomes,
        data_table.savings,
      ),
      transactions_header: vec!["Day", "Amount", "Note"],
      summary: data_table.summary,
      tabs: TabsState::default(),
      details_shown: false,
      total_saving: data_table.total_saving,
    }
  }

  fn select_next_block(&mut self) {
    match self.tabs.index {
      0 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Incomes,
          SelectedBlock::Incomes => self.selected_block = SelectedBlock::Expenses,
          SelectedBlock::Expenses => self.selected_block = SelectedBlock::Savings,
          SelectedBlock::Savings => self.selected_block = SelectedBlock::Months,
          _ => (),
        };
      }
      1 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Expenses,
          SelectedBlock::Expenses => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Incomes => (),
          SelectedBlock::Savings => (),
          _ => (),
        };
      }
      2 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Incomes,
          SelectedBlock::Incomes => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Expenses => (),
          SelectedBlock::Savings => (),
          _ => (),
        };
      }
      3 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Savings,
          SelectedBlock::Savings => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Expenses => (),
          SelectedBlock::Incomes => (),
          _ => (),
        };
      }
      _ => (),
    }
  }

  fn select_previous_block(&mut self) {
    match self.tabs.index {
      0 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Savings,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Incomes => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Expenses => self.selected_block = SelectedBlock::Incomes,
          SelectedBlock::Savings => self.selected_block = SelectedBlock::Expenses,
          _ => (),
        };
      }
      1 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Expenses,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Expenses => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Incomes => (),
          SelectedBlock::Savings => (),
          _ => (),
        };
      }
      2 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Incomes,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Incomes => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Expenses => (),
          SelectedBlock::Savings => (),
          _ => (),
        };
      }
      3 => {
        match self.selected_block {
          SelectedBlock::Months => self.selected_block = SelectedBlock::Savings,
          SelectedBlock::Tabs => self.selected_block = SelectedBlock::Months,
          SelectedBlock::Savings => self.selected_block = SelectedBlock::Tabs,
          SelectedBlock::Expenses => (),
          SelectedBlock::Incomes => (),
          _ => (),
        };
      }
      _ => (),
    }
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
      KeyCode::Enter => match self.selected_block {
        SelectedBlock::Savings => {
          if let Some(i) = self.table_state.savings.state.selected() {
            self.details_shown = true;
            self.table_state.transactions_details = Some(self.table_state.row_savings[i].clone());
          }
        }
        SelectedBlock::Incomes => {
          if let Some(i) = self.table_state.incomes.state.selected() {
            self.details_shown = true;
            self.table_state.transactions_details = Some(self.table_state.row_incomes[i].clone());
          }
        }
        SelectedBlock::Expenses => {
          if let Some(i) = self.table_state.expenses.state.selected() {
            self.details_shown = true;
            self.table_state.transactions_details = Some(self.table_state.row_expenses[i].clone());
          }
        }
        SelectedBlock::Months => {
          self.tree.toggle(&self.months_by_year);

          let transactions = get_transactions(&self.client, &self.user_token, &self.tree).await;

          match transactions {
            Ok(mut result) => {
              transaction_utils::sort(&mut result.0);
              transaction_utils::sort(&mut result.1);
              transaction_utils::sort(&mut result.2);

              self.table_state.expenses = CustomTableState {
                state: self.table_state.expenses.state.clone(),
                items: get_transactions_row(&result.0),
              };
              self.table_state.incomes = CustomTableState {
                state: self.table_state.incomes.state.clone(),
                items: get_transactions_row(&result.1),
              };
              self.table_state.savings = CustomTableState {
                state: self.table_state.savings.state.clone(),
                items: get_transactions_row(&result.2),
              };

              match transaction::get_transactions_balances(
                &self.client,
                &self.user_token,
                self.tree.current_month.clone(),
              )
              .await
              {
                Ok(data) => self.summary = data,
                Err(err) => println!("{:?}", err),
              };
            }
            Err(err) => println!("{:?}", err),
          }
        }
        _ => (),
      },
      KeyCode::Left => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.left()
        }

        if self.selected_block == SelectedBlock::Tabs {
          self.details_shown = false;
          self.table_state.transactions_details = None;
          self.tabs.previouse()
        }
      }
      KeyCode::Right => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.right()
        }

        if self.selected_block == SelectedBlock::Tabs {
          self.details_shown = false;
          self.table_state.transactions_details = None;
          self.tabs.next()
        }
      }
      KeyCode::Down => match self.selected_block {
        SelectedBlock::Months => self.tree.down(),
        SelectedBlock::Incomes | SelectedBlock::Expenses | SelectedBlock::Savings => {
          self.table_state.next(&self.selected_block);
        }
        _ => (),
      },
      KeyCode::Up => match self.selected_block {
        SelectedBlock::Months => self.tree.up(),
        SelectedBlock::Incomes | SelectedBlock::Expenses | SelectedBlock::Savings => {
          self.table_state.previous(&self.selected_block);
        }
        _ => (),
      },
      KeyCode::Char('r') => {
        let transactions = get_transactions(&self.client, &self.user_token, &self.tree).await;
        let total_saving = transaction::get_total_saving(&self.client, &self.user_token).await;

        match total_saving {
          Ok(data) => {
            self.total_saving = data;
          }
          Err(err) => println!("{:?}", err),
        }

        match transactions {
          Ok(mut result) => {
            transaction_utils::sort(&mut result.0);
            transaction_utils::sort(&mut result.1);
            transaction_utils::sort(&mut result.2);

            self.table_state.expenses = CustomTableState {
              state: self.table_state.expenses.state.clone(),
              items: get_transactions_row(&result.0),
            };
            self.table_state.incomes = CustomTableState {
              state: self.table_state.incomes.state.clone(),
              items: get_transactions_row(&result.1),
            };
            self.table_state.savings = CustomTableState {
              state: self.table_state.savings.state.clone(),
              items: get_transactions_row(&result.2),
            };

            match transaction::get_transactions_balances(
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
      _ => {}
    }
  }
}
