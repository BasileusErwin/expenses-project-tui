use std::{time::Duration, error};
use crate::{
  colors::*,
  load_config,
  models::{custom_error::CustomError, transaction::TransactionModel},
  enums::{transaction_type::TransactionType, month::MonthEnum, selected_block::SelectedBlock},
  utils::{transaction_utils, StatefulTree},
  state::{DataTable, App},
};
use crossterm::event::{KeyCode, self, Event, poll};
use tui::{
  Terminal,
  backend::Backend,
  Frame,
  style::Style,
  layout::{Layout, Direction, Constraint},
};
use tui_tree_widget::{Tree, TreeItem};

use crate::requests::transaction;

use self::{
  blocks::create_block,
  tables::{create_expenses_table, create_savings_table, create_incomes_table},
  summary::create_summeries,
};

pub mod blocks;
pub mod summary;
pub mod tables;

type TransactionsTuple = (
  Vec<TransactionModel>,
  Vec<TransactionModel>,
  Vec<TransactionModel>,
);

pub async fn get_transactions<'a>(
  client: &reqwest::Client,
  token: &String,
  tree: &StatefulTree<'a>,
) -> Result<TransactionsTuple, Box<dyn error::Error>> {
  let mut expenses = transaction::get_transactions_by_month_and_type(
    client,
    token,
    TransactionType::EXPENSE,
    tree.current_month.clone(),
  )
  .await?;

  let installments = transaction::get_transactions_by_month_and_type(
    client,
    token,
    TransactionType::INSTALLMENTS,
    tree.current_month.clone(),
  )
  .await?;

  expenses.extend(installments);

  let incomes = transaction::get_transactions_by_month_and_type(
    client,
    token,
    TransactionType::INCOME,
    tree.current_month.clone(),
  )
  .await?;

  let savings = transaction::get_transactions_by_month_and_type(
    client,
    token,
    TransactionType::SAVING,
    tree.current_month.clone(),
  )
  .await?;

  Ok((expenses, incomes, savings))
}

pub async fn run_app<B: Backend>(
  terminal: &mut Terminal<B>,
  client: &reqwest::Client,
) -> Result<(), Box<dyn error::Error>> {
  let config = match load_config() {
    Ok(config) => config,
    Err(_) => {
      return Err(Box::new(CustomError::new(
        Some(""),
        Some(String::from("Error to load config")),
        None,
      )));
    }
  };

  let token = match config.user_token {
    Some(token) => token,
    None => {
      return Err(Box::new(CustomError::new(
        Some(""),
        Some(String::from("Error to load user token")),
        None,
      )))
    }
  };

  let months_by_year = transaction::get_month_by_year(client, &token).await?;

  let mut tree_items: Vec<TreeItem> = Vec::new();

  for item in &months_by_year {
    let mut months: Vec<TreeItem> = Vec::new();

    for month in &item.months {
      months.push(TreeItem::new_leaf(month.clone()));
    }

    tree_items.push(TreeItem::new(item.year.clone(), months));
  }

  let tree = StatefulTree::with_items(tree_items.clone());

  let mut transactions = get_transactions(client, &token, &tree).await?;

  transaction_utils::sort(&mut transactions.0);
  transaction_utils::sort(&mut transactions.1);
  transaction_utils::sort(&mut transactions.2);

  let summary = transaction::get_transactions_balances(client, &token, tree.current_month.clone()).await?;

  let data: DataTable = DataTable {
    months_by_year,
    expenses: transactions.0,
    incomes: transactions.1,
    savings: transactions.2,
    summary,
    tree,
  };

  let mut app: App = App::new(data, token, client.to_owned());
  loop {
    terminal.draw(|f: &mut Frame<B>| ui(f, &mut app))?;

    if poll(Duration::from_millis(100))? {
      if let Event::Key(event) = event::read()? {
        if event.code == KeyCode::Char('q') {
          return Ok(());
        }
        app.process_key_event(event.code).await;
      }
    }
  }
}

pub fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
  let size = frame.size();

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
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

  let summary_layout = Layout::default()
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

  let summaries_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
      ]
      .as_ref(),
    )
    .split(summary_layout[0]);

  create_summeries(app, frame, summaries_layout);

  create_incomes_table(app, frame, transactions_icomes_layout[0]);

  create_savings_table(app, frame, transactions_icomes_layout[1]);

  create_expenses_table(app, frame, transactions_layout[0]);
}
