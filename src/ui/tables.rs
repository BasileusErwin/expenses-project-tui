use crate::{state::App, enums::selected_block::SelectedBlock, colors::*};

use tui::{
  backend::Backend,
  Frame,
  widgets::{Table, Row, Cell},
  style::{Style, Color},
  layout::{Constraint, Rect},
  text::Span,
};

use super::blocks::create_block;

pub fn create_incomes_table<B: Backend>(app: &mut App, frame: &mut Frame<B>, chunk: Rect) {
  let selected_style = Style::default()
    .bg(GREY)
    .fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(
      Style::default()
        .fg(BACKGROUND)
        .bg(GREEN),
    )
    .height(1)
    .bottom_margin(1);

  let rows = app.incomes_table.items.iter().map(|item| {
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

  let table = Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(&[
      Constraint::Length(20),
      Constraint::Length(20),
      Constraint::Length(20),
    ]);

  frame.render_stateful_widget(table, chunk, &mut app.incomes_table.state);
}

pub fn create_expenses_table<B: Backend>(app: &mut App, frame: &mut Frame<B>, chunk: Rect) {
  let selected_style = Style::default()
    .bg(GREY)
    .fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(
      Style::default()
        .fg(BACKGROUND)
        .bg(RED),
    )
    .height(1)
    .bottom_margin(1);

  let rows = app.expenses_table.items.iter().map(|item| {
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

  let table = Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(&[
      Constraint::Length(20),
      Constraint::Length(20),
      Constraint::Length(20),
    ]);

  frame.render_stateful_widget(table, chunk, &mut app.expenses_table.state);
}

pub fn create_savings_table<B: Backend>(app: &mut App, frame: &mut Frame<B>, chunk: Rect) {
  let selected_style = Style::default()
    .bg(GREY)
    .fg(YELLOW);

  let header_cells = app.transactions_header.iter().map(|h| Cell::from(*h));

  let header = Row::new(header_cells)
    .style(
      Style::default()
        .fg(BACKGROUND)
        .bg(FOREGROUND),
    )
    .height(1)
    .bottom_margin(1);

  let rows = app.savings_table.items.iter().map(|item| {
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

  let table = Table::new(rows)
    .header(header)
    .block(block)
    .highlight_style(selected_style)
    .highlight_symbol("")
    .widths(&[
      Constraint::Length(20),
      Constraint::Length(20),
      Constraint::Length(20),
    ]);

  frame.render_stateful_widget(table, chunk, &mut app.savings_table.state);
}
