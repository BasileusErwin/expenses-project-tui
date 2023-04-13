use std::vec;
use crate::{
  enums::{selected_block::SelectedBlock, currency::CurrencyEnum},
  requests::transaction::MonthByYear,
  utils::*,
  models::transaction::{TransactionModel, self},
};
use crossterm::event::KeyCode;
use tui::widgets::TableState;
use tui_tree_widget::TreeItem;

#[derive(Debug)]
pub struct TransactionsTableState {
  pub state: TableState,
  pub items: Vec<Vec<String>>,
}

pub struct TransactionTotal {
  pub total: f64,
  pub uyu: f64,
  pub usd: f64,
  pub eur: f64,
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
  pub incomes_total: TransactionTotal,
  pub expenses_total: TransactionTotal,
  pub savings_total: TransactionTotal,
}

pub struct DataTable {
  pub months_by_year: Vec<MonthByYear>,
  pub expenses: Vec<TransactionModel>,
  pub incomes: Vec<TransactionModel>,
  pub savings: Vec<TransactionModel>,
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
      ]
    })
    .collect()
}

impl<'a> App<'a> {
  pub fn new(data_table: DataTable, user_token: String) -> App<'a> {
    let mut tree_items: Vec<TreeItem> = Vec::new();

    for item in data_table.months_by_year {
      let mut months: Vec<TreeItem> = Vec::new();

      for month in item.months {
        months.push(TreeItem::new_leaf(month));
      }

      tree_items.push(TreeItem::new(item.year, months));
    }

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

    let mut savings_total: TransactionTotal = TransactionTotal {
      total: 0.0,
      uyu: 0.0,
      usd: 0.0,
      eur: 0.0,
    };

    let mut incomes_total: TransactionTotal = TransactionTotal {
      total: 0.0,
      uyu: 0.0,
      usd: 0.0,
      eur: 0.0,
    };

    let mut expenses_total: TransactionTotal = TransactionTotal {
      total: 0.0,
      uyu: 0.0,
      usd: 0.0,
      eur: 0.0,
    };

    // for data in data_table.savings {
    //   match data.currency {
    //     CurrencyEnum::UYU => savings_total.uyu += data.amount,
    //     CurrencyEnum::USD => {
    //       savings_total.usd += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           savings_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //     CurrencyEnum::EUR => {
    //       savings_total.eur += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           savings_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //   }
    //
    //   savings_total.total += savings_total.total;
    // }
    //
    // data_table.incomes.iter().for_each(|data| {
    //   match data.currency {
    //     CurrencyEnum::UYU => incomes_total.uyu += data.amount,
    //     CurrencyEnum::USD => {
    //       savings_total.usd += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           incomes_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //     CurrencyEnum::EUR => {
    //       savings_total.eur += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           incomes_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //   }
    //
    //   incomes_total.total += incomes_total.total;
    // });
    //
    // for data in data_table.expenses.iter() {
    //   match data.currency {
    //     CurrencyEnum::UYU => expenses_total.uyu += data.amount,
    //     CurrencyEnum::USD => {
    //       savings_total.usd += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           expenses_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //     CurrencyEnum::EUR => {
    //       savings_total.eur += data.amount;
    //       match data.exchange_rate {
    //         Some(exchange_rate) => {
    //           expenses_total.total += data.amount * exchange_rate.parse::<f64>().unwrap_or(1.0)
    //         }
    //         None => (),
    //       }
    //     }
    //   }
    //
    //   incomes_total.total += incomes_total.total;
    // }

    Self {
      tree: StatefulTree::with_items(tree_items.clone()),
      selected_block: SelectedBlock::Months,
      user_token,
      is_navigation: true,
      incomes_table,
      expenses_table,
      savings_table,
      transactions_header: vec!["Day", "Amount", "Category"],
      savings_total,
      incomes_total,
      expenses_total,
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
      SelectedBlock::Months => SelectedBlock::Summary,
      SelectedBlock::Summary => SelectedBlock::Incomes,
      SelectedBlock::Incomes => SelectedBlock::Expenses,
      SelectedBlock::Expenses => SelectedBlock::Savings,
      SelectedBlock::Savings => SelectedBlock::Months,
    };
  }

  fn select_previous_block(&mut self) {
    self.selected_block = match self.selected_block {
      SelectedBlock::Months => SelectedBlock::Savings,
      SelectedBlock::Summary => SelectedBlock::Months,
      SelectedBlock::Incomes => SelectedBlock::Summary,
      SelectedBlock::Expenses => SelectedBlock::Incomes,
      SelectedBlock::Savings => SelectedBlock::Expenses,
    };
  }

  pub fn process_key_event(&mut self, key_code: KeyCode) {
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
      KeyCode::Char('\n' | ' ') => self.tree.toggle(),
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
        _ => {}
      },
      KeyCode::Up => match self.selected_block {
        SelectedBlock::Months => self.tree.up(),
        SelectedBlock::Incomes | SelectedBlock::Expenses | SelectedBlock::Savings => {
          self.previous_table_item()
        }
        _ => {}
      },
      _ => {}
    }
  }
}
