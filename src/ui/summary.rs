use tui::{
  style::Style,
  widgets::{Borders, Block, Paragraph},
  Frame,
  layout::{Rect, Alignment},
  backend::Backend,
  text::Spans,
};

use crate::{
  state::App,
  colors::{BACKGROUND, YELLOW},
};

pub fn create_summeries<B: Backend>(app: &mut App, frame: &mut Frame<B>, layout: Vec<Rect>) {
  let incomes = Block::default()
    .title("Summery Incomes")
    .borders(Borders::ALL)
    .style(Style::default().bg(BACKGROUND).fg(YELLOW));

  let expenses = Block::default()
    .title("Summery Expenses")
    .borders(Borders::ALL)
    .style(Style::default().bg(BACKGROUND).fg(YELLOW));

  let savings = Block::default()
    .title("Summery Savings")
    .borders(Borders::ALL)
    .style(Style::default().bg(BACKGROUND).fg(YELLOW));

  let totals = Block::default()
    .title("Total")
    .borders(Borders::ALL)
    .style(Style::default().bg(BACKGROUND).fg(YELLOW));

  let incomes_summery = &app.summary.incomes;

  let incomes_span = vec![
    Spans::from(format!("Total: ${} UYU", incomes_summery.total)),
    Spans::from(format!("UYU: ${}", incomes_summery.uyu)),
    Spans::from(format!("USD: ${}", incomes_summery.usd)),
    Spans::from(format!("EUR: ${}", incomes_summery.eur)),
  ];

  let expenses_span = vec![
    Spans::from(format!("Total: ${} UYU", &app.summary.expenses.total)),
    Spans::from(format!("UYU: ${}", &app.summary.expenses.uyu)),
    Spans::from(format!("USD: ${}", &app.summary.expenses.usd)),
    Spans::from(format!("EUR: ${}", &app.summary.expenses.eur)),
  ];

  let savings_span = vec![
    Spans::from(format!("Total: ${} UYU", &app.summary.savings.total)),
    Spans::from(format!("UYU: ${}", &app.summary.savings.uyu)),
    Spans::from(format!("USD: ${}", &app.summary.savings.usd)),
    Spans::from(format!("EUR: ${}", &app.summary.savings.eur)),
  ];

  let current_money_span = vec![Spans::from(format!(
    "Total: ${} UYU",
    (app.summary.incomes.total - app.summary.expenses.total).round()
  ))];

  let incomes_paragraph = Paragraph::new(incomes_span)
    .block(incomes)
    .alignment(Alignment::Left);

  frame.render_widget(incomes_paragraph, layout[0]);

  let expenses_paragraph = Paragraph::new(expenses_span)
    .block(expenses)
    .alignment(Alignment::Left);

  frame.render_widget(expenses_paragraph, layout[1]);

  let savings_paragraph = Paragraph::new(savings_span)
    .block(savings)
    .alignment(Alignment::Left);

  frame.render_widget(savings_paragraph, layout[2]);

  let current_money_paragraph = Paragraph::new(current_money_span)
    .block(totals)
    .alignment(Alignment::Left);

  frame.render_widget(current_money_paragraph, layout[3]);
}
