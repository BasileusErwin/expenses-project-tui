use tui::{
  widgets::Tabs,
  text::Spans,
  style::{Style, Modifier, Color},
};

use crate::states::App;

use super::{
  blocks::create_block,
  colors::{BLUE, CYAN},
};

pub fn create_tabs<'a>(titles: Vec<Spans<'a>>, app: &App<'a>) -> Tabs<'a> {
  Tabs::new(titles)
    .block(create_block(
      String::from("Tabs"),
      &app.selected_block,
      crate::enums::selected_block::SelectedBlock::Tabs,
      BLUE,
    ))
    .select(app.tabs.index)
    .style(Style::default().fg(CYAN))
    .highlight_style(
      Style::default()
        .add_modifier(Modifier::BOLD)
        .bg(Color::Black),
    )
}
