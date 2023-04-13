use tui::{
  widgets::{Block, Borders, BorderType},
  style::{Style, Color},
};

use crate::{enums::selected_block::SelectedBlock, colors::*};

pub fn create_block<'a>(
  title: String,
  selected_block_state: &SelectedBlock,
  selected_block: SelectedBlock,
  foreground: Color,
) -> Block<'a> {
  Block::default()
    .title(title)
    .borders(Borders::ALL)
    .border_type(if selected_block_state == &selected_block {
      BorderType::Thick
    } else {
      BorderType::Plain
    })
    .style(Style::default().bg(BACKGROUND).fg(foreground))
}
