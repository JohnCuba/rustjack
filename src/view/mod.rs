use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::{Block, Paragraph, Wrap},
};

use crate::game::{Game, GameStatus};

mod constants;
mod fallback;
mod card;
mod deck;
mod hand;

pub fn render_game(frame: &mut Frame, game: &Game) {
  if fallback::check_view_port(frame) {
    fallback::render_fallback(frame);
    return;
  }

  match &game.status {
    GameStatus::Betting => {}
    _ => {
      hand::render(
        frame,
        hand::RenderHandOptions {
          hand: &game.dealer_hand,
          aligment: VerticalAlignment::Top,
          show_only_first: match game.status {
            GameStatus::Draw | GameStatus::PlayerWon | GameStatus::DealerWon => false,
            _ => true,
          },
        },
      );
      hand::render(
        frame,
        hand::RenderHandOptions {
          hand: &game.player_hand,
          aligment: VerticalAlignment::Bottom,
          show_only_first: false,
        },
      );
    }
  }

  deck::render(frame, &game);

  let mut content = vec![
    Line::from(format!("{} $", game.balance.player)),
    Line::from(""),
  ];

  match game.status {
    GameStatus::Betting => {
      content.push(Line::from("Betting"));
      content.push(Line::from(""));
      content.push(Line::from("[s] start game"));
    }
    GameStatus::PlayerTurn => {
      content.push(Line::from("Your turn"));
      content.push(Line::from(""));
      content.push(Line::from("[h] hit"));
      content.push(Line::from("[s] stand"));
    }
    GameStatus::DealerTurn => {
      content.push(Line::from("Dealer turn"));
    }
    GameStatus::PlayerWon => {
      content.push(Line::from(Span::styled(
        "You won!",
        Style::default().fg(Color::Green),
      )));
      content.push(Line::from(""));
      content.push(Line::from("[n] new game"));
    }
    GameStatus::DealerWon => {
      content.push(Line::from(Span::styled(
        "Dealer won!",
        Style::default().fg(Color::Red),
      )));
      content.push(Line::from(""));
      content.push(Line::from("[n] new game"));
    }
    GameStatus::Draw => {
      content.push(Line::from(Span::styled(
        "Draw!",
        Style::default().fg(Color::Yellow),
      )));
      content.push(Line::from(""));
      content.push(Line::from("[n] new game"));
    }
  }

  let content_len = content.len() as u16;

  frame.render_widget(
    Text::from(content),
    Rect {
      x: 2,
      y: (frame.area().height / 2) - (content_len / 2).max(1),
      width: 50,
      height: content_len,
    },
  );

  frame.render_widget(
    Block::bordered()
      .title_top(Line::from(" RustJack ").alignment(HorizontalAlignment::Left))
      .title_bottom(
        Line::from("[^q] exit, [^r] reset")
          .alignment(HorizontalAlignment::Left),
      ),
    frame.area(),
  );
}

pub fn handle_key_event<'a>(key: KeyEvent, game: &mut Game) -> Result<(), ()> {
  match key.code {
    KeyCode::Char('n') => {
      game.reset();
      Ok(())
    }
    KeyCode::Char('q') => Err(()),
    KeyCode::Char('r') => {
      game.reset_balance();
      Ok(())
    }
    KeyCode::Char('b') => {
      match &game.status {
        GameStatus::Betting => game.player_increase_bet(),
        _ => {},
      };
      Ok(())
    }
    KeyCode::Char('s') => {
      match &game.status {
        GameStatus::Betting => game.start(),
        GameStatus::PlayerTurn => game.player_stand(),
        _ => {},
      };
      Ok(())
    }
    KeyCode::Char('h') => {
      match &game.status {
        GameStatus::PlayerTurn => game.player_hit(),
        _ => {},
      }
      Ok(())
    }
    _ => Ok(()),
  }
}
