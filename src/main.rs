mod utils;

use crate::utils::*;
use std::{io, vec, time::Duration};
use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  execute,
  event::{EnableMouseCapture, KeyCode, self, Event, DisableMouseCapture, poll},
};
use requests::{
  user,
  transaction::{self, MonthByYear},
};
use bcrypt::DEFAULT_COST;
use termion::color::Rgb;
use tui::{
  Terminal,
  backend::{Backend, CrosstermBackend},
  Frame,
  widgets::{Block, Borders, BorderType},
  style::{Style, Color, Modifier},
  layout::{Layout, Direction, Constraint, Alignment},
  text::Span,
};
use tui_tree_widget::{TreeItem, Tree};

// $background: #0a0e14; -> rgb(10, 14, 20)
// $foreground: #bfbab0; -> rgb(191, 186, 176)
//
// $grey: #151720; -> rgb(21, 23, 32)
// $yellowLite: #ffd580; -> rgb(255, 213, 128)
// $yellow: #ffb454; -> rgb(255, 180, 84)
// $orange: #ff8f40; -> rgb(255, 143, 64)
// $blue: #73d0ff; -> rgb(115, 208, 255)
// $green: #bae67e; -> rgb(186, 230, 126)
// $red: #ff3333; -> rgb(255, 51, 51)
// $cyan: #95e6cb; -> rgb(149, 230, 203)
//
// $archColor: #1793d1; -> rgb(23, 147, 209)

use crate::requests::health;

pub mod models;
pub mod requests;
pub mod types;

#[derive(Debug, PartialEq)]
enum SelectedBlock {
  Months,
  Summary,
  Incomes,
  Savings,
  Expenses,
}

struct App<'a> {
  tree: StatefulTree<'a>,
  selected_block: SelectedBlock,
  user_token: String,
}

impl<'a> App<'a> {
  pub fn new(months_by_year: Vec<MonthByYear>, user_token: String) -> App<'a> {
    let mut tree_items: Vec<TreeItem> = Vec::new();

    for item in months_by_year {
      let mut months: Vec<TreeItem> = Vec::new();

      for month in item.months {
        months.push(TreeItem::new_leaf(month));
      }

      tree_items.push(TreeItem::new(item.year, months));
    }

    Self {
      tree: StatefulTree::with_items(tree_items.clone()),
      selected_block: SelectedBlock::Months,
      user_token,
    }
  }

  fn select_next_block(&mut self) {
    self.selected_block = match self.selected_block {
      SelectedBlock::Months => SelectedBlock::Summary,
      SelectedBlock::Summary => SelectedBlock::Incomes,
      SelectedBlock::Incomes => SelectedBlock::Savings,
      SelectedBlock::Savings => SelectedBlock::Expenses,
      SelectedBlock::Expenses => SelectedBlock::Months,
    };
  }

  fn select_previous_block(&mut self) {
    self.selected_block = match self.selected_block {
      SelectedBlock::Months => SelectedBlock::Expenses,
      SelectedBlock::Summary => SelectedBlock::Months,
      SelectedBlock::Incomes => SelectedBlock::Summary,
      SelectedBlock::Savings => SelectedBlock::Incomes,
      SelectedBlock::Expenses => SelectedBlock::Savings,
    };
  }

  pub fn process_key_event(&mut self, key_code: KeyCode) {
    match key_code {
      KeyCode::Char('j') => self.select_next_block(),
      KeyCode::Char('k') => self.select_previous_block(),
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
      KeyCode::Down => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.down()
        }
      }
      KeyCode::Up => {
        if self.selected_block == SelectedBlock::Months {
          self.tree.up()
        }
      }
      _ => {}
    }
  }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
  loop {
    terminal.draw(|f: &mut Frame<B>| ui(f, app))?;

    if poll(Duration::from_millis(100))? {
      if let Event::Key(event) = event::read()? {
        if event.code == KeyCode::Char('q') {
          return Ok(());
        }
        app.process_key_event(event.code);
      }
    }
  }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
  let size = frame.size();

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
    .split(size);

  let months_block = Tree::new(app.tree.items.clone())
    .block(
      Block::default()
        .title("Months")
        .borders(Borders::ALL)
        .border_type(if app.selected_block == SelectedBlock::Months {
          BorderType::Thick
        } else {
          BorderType::Plain
        })
        .style(
          Style::default()
            // black
            .bg(Color::Rgb(10, 14, 20))
            // orange
            .fg(Color::Rgb(255, 143, 64)),
        ),
    )
    .highlight_style(Style::default().fg(Color::Rgb(191, 186, 176)));

  frame.render_stateful_widget(months_block, chunks[0], &mut app.tree.state);

  let center_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(chunks[1]);

  let center_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(10), Constraint::Percentage(80)].as_ref())
    .split(center_chunks[0]);

  let summery_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Length(30)].as_ref())
    .split(center_layout[0]);

  let transactions_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(center_layout[1]);

  let transactions_icomes_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
    .split(transactions_layout[0]);

  let block_summery = Block::default()
    .title("Summary")
    .borders(Borders::ALL)
    .border_type(if app.selected_block == SelectedBlock::Summary {
      BorderType::Thick
    } else {
      BorderType::Plain
    })
    .style(
      Style::default()
        .bg(Color::Rgb(10, 14, 20))
        .fg(Color::Rgb(255, 180, 84)),
    );

  frame.render_widget(block_summery, summery_layout[0]);

  let block_incomes = Block::default()
    .title("Incomes")
    .borders(Borders::ALL)
    .border_type(if app.selected_block == SelectedBlock::Incomes {
      BorderType::Thick
    } else {
      BorderType::Plain
    })
    .style(
      Style::default()
        .bg(Color::Rgb(10, 14, 20))
        .fg(Color::Rgb(186, 230, 126)),
    );

  let block_savings = Block::default()
    .title("Savings")
    .borders(Borders::ALL)
    .border_type(if app.selected_block == SelectedBlock::Savings {
      BorderType::Thick
    } else {
      BorderType::Plain
    })
    .style(Style::default().bg(Color::Rgb(10, 14, 20)));

  frame.render_widget(block_incomes, transactions_icomes_layout[0]);
  frame.render_widget(block_savings, transactions_icomes_layout[1]);

  let block_expenses = Block::default()
    .title("Expenses")
    .borders(Borders::ALL)
    .border_type(if app.selected_block == SelectedBlock::Expenses {
      BorderType::Thick
    } else {
      BorderType::Plain
    })
    .style(
      Style::default()
        .bg(Color::Rgb(10, 14, 20))
        .fg(Color::Rgb(255, 51, 51)),
    );

  frame.render_widget(block_expenses, transactions_layout[1]);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  if !health::request(&client).await.unwrap() {
    panic!("{:?}", String::from("Error connect to server"));
  }

  terminal::enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let data = transaction::get_month_by_year(&client, "f1de3ff886dbe934ce7dedd42c51b3da15d1e71ce49a3288284c9c51f37952b8f7f167c1290b83e2c7ea46dc601c68ecc34c".to_string()).await?;
  println!("{:?}", data);

  let mut app: App = App::new(data, "f1de3ff886dbe934ce7dedd42c51b3da15d1e71ce49a3288284c9c51f37952b8f7f167c1290b83e2c7ea46dc601c68ecc34c".to_string());

  let res = run_app(&mut terminal, &mut app);

  terminal::enable_raw_mode()?;

  terminal::disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;

  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{:?}", err)
  }

  Ok(())
}
