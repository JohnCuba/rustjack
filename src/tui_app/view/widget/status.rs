use ratatui::{
  Frame,
  layout::{Constraint, Layout},
  style::{Color, Style},
  text::{Line, Span, Text},
};

use crate::game::{Game, GameStatus};

pub fn render(frame: &mut Frame, game: &Game) {
  let mut content = vec![
    Line::from(format!("Balance: {}$", game.balance.player)),
    Line::default(),
  ];

  let status_lines: Vec<Line> = match &game.status {
    GameStatus::Betting => vec![
      Line::from("Betting"),
      Line::from("[enter] start game"),
    ],
    GameStatus::PlayerTurn => vec![
      Line::from("Your turn"),
      Line::from("[⮕] hit"),
      Line::from("[⬅] stand"),
    ],
    GameStatus::DealerTurn => vec![
      Line::from("Dealer turn"),
    ],
    GameStatus::PlayerWon => vec![
      Line::from(Span::styled(
        "You won!",
        Style::default().fg(Color::Green).bold(),
      )),
      Line::from("[enter] new game"),
    ],
    GameStatus::DealerWon => vec![
      Line::from(Span::styled(
        "Dealer won!",
        Style::default().fg(Color::Red).bold(),
      )),
      Line::from("[enter] new game"),
    ],
    GameStatus::Draw => vec![
      Line::from(Span::styled(
        "Draw!",
        Style::default().fg(Color::Yellow).bold(),
      )),
      Line::from("[enter] new game"),
    ],
  };
  content.extend(status_lines);

  let content_height = content.len() as u16;

  let vertical = Layout::vertical([
    Constraint::Length((frame.area().height.saturating_sub(content_height)) / 2),
    Constraint::Length(content_height),
    Constraint::Min(0),
  ])
  .split(frame.area());

  let area = Layout::horizontal([
      Constraint::Length(2),
      Constraint::Min(0),
  ])
  .split(vertical[1]);

  frame.render_widget(Text::from(content), area[1]);
}
