use chrono::{Local, Datelike};
use tui_tree_widget::{TreeItem, TreeState};

use crate::{enums::month::MonthEnum, types::responses::transaction::MonthByYear};

pub mod transaction_utils;

pub struct StatefulTree<'a> {
  pub state: TreeState,
  pub items: Vec<TreeItem<'a>>,
  pub current_month: MonthEnum,
  pub current_year: i32,
}

impl<'a> StatefulTree<'a> {
  #[allow(dead_code)]
  pub fn new() -> Self {
    let state: TreeState = TreeState::default();

    let now = Local::now();
    let month = now.month();

    Self {
      state,
      items: Vec::new(),
      current_month: MonthEnum::from_u32(month).unwrap_or(MonthEnum::JANUARY),
      current_year: now.year(),
    }
  }

  pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
    let now = Local::now();

    Self {
      state: TreeState::default(),
      items,
      current_month: MonthEnum::from_u32(now.month()).unwrap_or(MonthEnum::JANUARY),
      current_year: now.year(),
    }
  }

  pub fn first(&mut self) {
    self.state.select_first();
  }

  pub fn last(&mut self) {
    self.state.select_last(&self.items);
  }

  pub fn down(&mut self) {
    self.state.key_down(&self.items);
  }

  pub fn up(&mut self) {
    self.state.key_up(&self.items);
  }

  pub fn left(&mut self) {
    self.state.key_left();
  }

  pub fn right(&mut self) {
    self.state.key_right();
  }

  pub fn toggle(&mut self, months_by_year: &[MonthByYear]) {
    self.state.toggle_selected();

    let selected = self.state.selected();
    if !selected.is_empty() {
      let year_index = selected[0];
      if selected.len() > 1 {
        let month_index = selected[1];

        let month = months_by_year[year_index].months[month_index].clone();
        self.current_month = MonthEnum::from_string(month).unwrap_or(MonthEnum::JANUARY);
      }

      let year = months_by_year[year_index].year.clone();
      self.current_year = year.parse().unwrap_or(2023);
    }
  }
}

impl<'a> Default for StatefulTree<'a> {
  fn default() -> Self {
    Self::new()
  }
}
