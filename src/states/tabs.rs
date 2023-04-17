#[derive(Debug)]
pub struct TabsState<'a> {
  pub titles: Vec<&'a str>,
  pub index: usize,
}

impl<'a> Default for TabsState<'a> {
  fn default() -> Self {
    TabsState {
      titles: vec!["All", "Expenses", "Incomes", "Savings"],
      index: 0,
    }
  }
}

impl<'a> TabsState<'a> {
  pub fn new(titles: Vec<&'a str>, index: usize) -> Self {
    Self { titles, index }
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.titles.len();
  }

  pub fn previouse(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.titles.len() - 1;
    }
  }
}

