use crate::{states::App, enums::selected_block::SelectedBlock};
use tui::{
  widgets::{Table, Row, Cell},
  style::Style,
  layout::Constraint,
  text::Span,
};

use super::colors::*;
use super::blocks::create_block;

pub fn create_incomes_table<'a>(app: &mut App<'a>) -> Table<'a> {
  let selected_style = Style::default().bg(GREY).fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(Style::default().fg(BACKGROUND).bg(GREEN))
    .height(1)
    .bottom_margin(1);

  let rows = app.table_state.incomes.items.iter().map(|item| {
    let height = item
      .iter()
      .map(|content| content.chars().filter(|c| *c == '\n').count())
      .max()
      .unwrap_or(0)
      + 1;

    let cells = item.iter().map(|c| Cell::from(Span::from(c.clone())));
    Row::new(cells).height(height as u16).bottom_margin(1)
  });

  let block = create_block(
    String::from("Incomes"),
    &app.selected_block,
    SelectedBlock::Incomes,
    GREEN,
  );

  Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(
      [
        Constraint::Length(4),
        Constraint::Length(15),
        Constraint::Length(30),
      ]
      .as_ref(),
    )
}

pub fn create_expenses_table<'a>(app: &mut App<'a>) -> Table<'a> {
  let selected_style = Style::default().bg(GREY).fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(Style::default().fg(BACKGROUND).bg(RED))
    .height(1)
    .bottom_margin(1);

  let rows = app.table_state.expenses.items.iter().map(|item| {
    let height = item
      .iter()
      .map(|content| content.chars().filter(|c| *c == '\n').count())
      .max()
      .unwrap_or(0)
      + 1;

    let cells = item.iter().map(|c| Cell::from(Span::from(c.clone())));
    Row::new(cells).height(height as u16).bottom_margin(1)
  });

  let block = create_block(
    String::from("Expenses"),
    &app.selected_block,
    SelectedBlock::Expenses,
    RED,
  );

  Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(
      [
        Constraint::Length(4),
        Constraint::Length(15),
        Constraint::Length(30),
      ]
      .as_ref(),
    )
}

pub fn create_savings_table<'a>(app: &mut App<'a>) -> Table<'a> {
  let selected_style = Style::default().bg(GREY).fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(Style::default().fg(BACKGROUND).bg(FOREGROUND))
    .height(1)
    .bottom_margin(1);

  let rows = app.table_state.savings.items.iter().map(|item| {
    let height = item
      .iter()
      .map(|content| content.chars().filter(|c| *c == '\n').count())
      .max()
      .unwrap_or(0)
      + 1;

    let cells = item.iter().map(|c| Cell::from(Span::from(c.clone())));
    Row::new(cells).height(height as u16).bottom_margin(1)
  });

  let block = create_block(
    String::from("Savings"),
    &app.selected_block,
    SelectedBlock::Savings,
    FOREGROUND,
  );

  Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(
      [
        Constraint::Length(4),
        Constraint::Length(15),
        Constraint::Length(30),
      ]
      .as_ref(),
    )
}
