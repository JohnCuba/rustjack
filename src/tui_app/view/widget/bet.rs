use crate::{
  core::game::{Game, GameStatus},
  tui_app::view::constants::Dimension,
};
use ratatui::{
  Frame,
  layout::{Constraint, Layout},
  text::{Line, Text},
  widgets::Paragraph,
};

pub fn render(frame: &mut Frame, game: &Game) {
  let mut content = vec![Line::from(format!("bet: {}$", game.balance.bet))];

  if let GameStatus::Betting = &game.status {
    content.push(Line::from("⬆/⬇ bet on 5$"));
  }

  let dim = Dimension {
    height: content.len() as u16,
    width: content.last().unwrap().width() as u16,
  };

  let vertical = Layout::vertical([
    Constraint::Length((frame.area().height.saturating_sub(dim.height)) / 2),
    Constraint::Length(dim.height),
    Constraint::Min(0),
  ])
  .split(frame.area());

  let area = Layout::horizontal([
    Constraint::Length((frame.area().width.saturating_sub(dim.width)) / 2),
    Constraint::Length(dim.width),
    Constraint::Min(0),
  ])
  .split(vertical[1])[1];

  frame.render_widget(Paragraph::new(Text::from(content)).centered(), area);
}
