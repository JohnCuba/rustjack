use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  style::{Color, Style},
  text::{Line, Span, Text},
  widgets::{Block, Paragraph, Wrap},
};

use crate::game::{Game, GameStatus};

mod card;
mod deck;
mod hand;

pub fn render_game(frame: &mut Frame, game: &Game) {
  if frame.area().width < 50 || frame.area().height < 19 {
    let content = format!(
      "Terminal window must be at least 50x19, now {}x{}",
      frame.area().width,
      frame.area().height,
    );
    let content_len = content.len() as u16;
    frame.render_widget(
      Paragraph::new(content)
        .alignment(HorizontalAlignment::Center)
        .wrap(Wrap { trim: true })
        .scroll((0, 0)),
      Rect {
        x: 0,
        y: (frame.area().height / 2) - content_len / frame.area().width,
        width: frame.area().width,
        height: frame.area().height,
      },
    );
    return;
  }

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
  deck::render(frame);

  let bet_text = format!("bet: {}$", game.balance.bet);
  let bet_text_len = bet_text.len() as u16;

  frame.render_widget(
    Text::from(format!("bet: {}$", game.balance.bet)),
    Rect {
      x: frame.area().width / 2 - bet_text_len / 2,
      y: frame.area().height / 2,
      width: bet_text_len,
      height: 1,
    },
  );

  let mut content = vec![
    Line::from(format!("{} $", game.balance.player)),
    Line::from(""),
  ];

  match game.status {
    GameStatus::Betting => {
      content.push(Line::from("Betting"));
      content.push(Line::from(""));
      content.push(Line::from("B - increase bet on 5$"));
      content.push(Line::from("H - hit"));
      content.push(Line::from("S - stand"));
    }
    GameStatus::PlayerTurn => {
      content.push(Line::from("Your turn"));
      content.push(Line::from(""));
      content.push(Line::from("H - hit"));
      content.push(Line::from("S - stand"));
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
      content.push(Line::from("N - new game"));
      content.push(Line::from("R - reset balance"));
    }
    GameStatus::DealerWon => {
      content.push(Line::from(Span::styled(
        "Dealer won!",
        Style::default().fg(Color::Red),
      )));
      content.push(Line::from(""));
      content.push(Line::from("N - new game"));
      content.push(Line::from("R - reset balance"));
    }
    GameStatus::Draw => {
      content.push(Line::from(Span::styled(
        "Draw!",
        Style::default().fg(Color::Yellow),
      )));
      content.push(Line::from(""));
      content.push(Line::from("N - new game"));
      content.push(Line::from("R - reset balance"));
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
    Block::bordered().title_top(Line::from(" RustJack ").alignment(HorizontalAlignment::Left)),
    frame.area(),
  );
}

pub fn handle_key_event<'a>(
  key: KeyEvent,
  game: &mut Game,
) -> Result<(), ()> {
  match key.code {
    KeyCode::Char('n') => {
      game.reset(false);
      Ok(())
    }
    KeyCode::Char('q') => Err(()),
    KeyCode::Char('r') => {
      game.reset(true);
      Ok(())
    }
    KeyCode::Char('b') => {
      game.player_increase_bet();
      Ok(())
    }
    KeyCode::Char('s') => {
      game.player_stand();
      Ok(())
    }
    KeyCode::Char('h') => {
      game.player_hit();
      Ok(())
    }
    _ => Ok(()),
  }
}
