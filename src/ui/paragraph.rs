use tui::{
  text::{Spans, Span},
  widgets::Paragraph,
  layout::Alignment,
  style::{Style, Color},
};

use crate::{
  enums::{selected_block::SelectedBlock, currency::CurrencyEnum},
  models::transaction::TransactionModel,
};

use super::{blocks::create_block, colors::FOREGROUND};

pub fn get_paragraph_to_details_transaction_details<'a>(
  selected_block: &'a SelectedBlock,
  transaction: &'a TransactionModel,
  color: Color,
) -> Paragraph<'a> {
  let details = create_block(
    String::from("Details"),
    selected_block,
    SelectedBlock::Details,
    color,
  );

  let mut spans = vec![
    Spans::from(""),
    Spans::from(Span::styled(
      format!("Id: {}", &transaction.transaction_id),
      Style::default().fg(FOREGROUND),
    )),
    Spans::from(""),
    Spans::from(Span::styled(
      format!("Type: {:?}", &transaction.r#type),
      Style::default().fg(FOREGROUND),
    )),
    Spans::from(""),
    Spans::from(Span::styled(
      format!(
        "Amount: {} {:?}",
        &transaction.amount, &transaction.currency
      ),
      Style::default().fg(FOREGROUND),
    )),
    match &transaction.currency {
      CurrencyEnum::USD | CurrencyEnum::EUR => Spans::from(Span::styled(
        format!(
          "Exchange Rate: {}\n",
          match &transaction.exchange_rate {
            Some(rate) => rate,
            None => &0_f64,
          }
        ),
        Style::default().fg(FOREGROUND),
      )),
      _ => Spans::from(""),
    },
    Spans::from(Span::styled(
      format!(
        "Note: {}",
        match &transaction.note {
          Some(note) => note,
          None => "",
        }
      ),
      Style::default().fg(FOREGROUND),
    )),
    Spans::from(""),
    Spans::from(Span::styled(
      format!(
        "Date: {}/{:?}{}",
        &transaction.year,
        &transaction.month,
        match &transaction.day {
          Some(day) => format!("/{}", day),
          None => "".to_string(),
        },
      ),
      Style::default().fg(FOREGROUND),
    )),
  ];

  match &transaction.category {
    Some(category) => {
      spans.push(Spans::from(""));
      spans.push(Spans::from(Span::styled(
        "Category: ",
        Style::default().fg(FOREGROUND),
      )));
      spans.push(Spans::from(""));
      spans.push(Spans::from(Span::styled(
        format!("Id: {}", category.category_id),
        Style::default().fg(FOREGROUND),
      )));
      spans.push(Spans::from(""));
      spans.push(Spans::from(Span::styled(
        format!("Name: {}", category.name),
        Style::default().fg(FOREGROUND),
      )));
    }
    None => (),
  }

  Paragraph::new(spans)
    .block(details)
    .alignment(Alignment::Left)
}
