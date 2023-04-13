use std::{io, vec, time::Duration};
use colors::*;
use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  execute,
  event::{EnableMouseCapture, KeyCode, self, Event, DisableMouseCapture, poll},
};
use enums::{selected_block::SelectedBlock, transaction_type::TransactionType, month::MonthEnum};
use state::{App, DataTable};
use tui::{
  Terminal,
  backend::{Backend, CrosstermBackend},
  Frame,
  widgets::{Block, Borders, BorderType},
  style::{Style, Color},
  layout::{Layout, Direction, Constraint},
};
use tui_tree_widget::Tree;
use ui::{
  tables::{create_incomes_table, create_expenses_table, create_savings_table},
  blocks::create_block,
};

use crate::requests::{health, transaction};

pub mod colors;
pub mod enums;
pub mod models;
pub mod requests;
pub mod state;
pub mod types;
pub mod ui;
pub mod utils;

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
    .constraints([Constraint::Percentage(17), Constraint::Percentage(83)].as_ref())
    .split(size);

  let months_block = Tree::new(app.tree.items.clone())
    .block(create_block(
      String::from("Months"),
      &app.selected_block,
      SelectedBlock::Months,
      ORANGE,
    ))
    .highlight_style(Style::default().fg(FOREGROUND))
    .highlight_symbol("");

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
    .split(transactions_layout[1]);

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

  create_incomes_table(app, frame, transactions_icomes_layout[0]);

  create_savings_table(app, frame, transactions_icomes_layout[1]);

  create_expenses_table(app, frame, transactions_layout[0]);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  let ping = health::request(&client).await;
  if ping.is_err() {
    println!("{}", String::from("Error connect to server"));
    std::process::exit(1)
  }

  let token = "1016d36c36191596367650ae5c67302cd7769ad5c46eeb38ca1eb0a0495b535d45a09550fd3df951ec72e410dde2cd5bd831".to_string();

  let data: DataTable = DataTable { 
    months_by_year: transaction::get_month_by_year(&client, &token).await?,
    expenses: transaction::get_transactions_by_month_and_type(&client, &token, TransactionType::EXPENSE, MonthEnum::APRIL).await?, 
    incomes: transaction::get_transactions_by_month_and_type(&client, &token, TransactionType::INCOME, MonthEnum::APRIL).await?,
    savings: transaction::get_transactions_by_month_and_type(&client, &token, TransactionType::SAVING, MonthEnum::APRIL).await?,
  };

  terminal::enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app: App = App::new(data, token);

  let res = run_app(&mut terminal, &mut app);

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
