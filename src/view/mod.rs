use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  text::{Line, Text},
  widgets::Block,
};

use crate::game::{Game, GameStatus};

mod card;
mod deck;
mod hand;

pub fn render_game(frame: &mut Frame, game: &Game) {
  hand::render(
    frame,
    &game.dealer_hand,
    game.balance.borrow().dealer,
    VerticalAlignment::Top,
    match game.status {
      GameStatus::Draw | GameStatus::PlayerWon | GameStatus::DealerWon => false,
      _ => true,
    }
  );
  hand::render(
    frame,
    &game.player_hand,
    game.balance.borrow().player,
    VerticalAlignment::Bottom,
    false,
  );
  deck::render(frame);

  let mut content = String::new();

  match game.status {
    GameStatus::Betting => {
      content.push_str(" Betting. b - increase bet on 5$, h - hit, s - stand ");
    },
    GameStatus::PlayerTurn => {
      content.push_str(" Your turn. h - hit, s - stand ");
    }
    GameStatus::PlayerWon => {
      content.push_str(" You won! n - new game ");
    }
    GameStatus::DealerWon => {
      content.push_str(" Dealer won! n - new game ");
    }
    GameStatus::Draw => {
      content.push_str(" Draw! n - new game ");
    }
    GameStatus::DealerTurn => {
      content.push_str(" Dealer turn ");
    }
  }

  let bet_text = format!("bet: {}$", game.balance.borrow().bet);
  let bet_text_len = bet_text.len() as u16;

  frame.render_widget(
    Text::from(format!("bet: {}$", game.balance.borrow().bet)),
    Rect {
      x: frame.area().width / 2 - bet_text_len / 2,
      y: frame.area().height / 2,
      width: bet_text_len,
      height: 1,
    },
  );

  frame.render_widget(
    Block::bordered()
      .title_top(Line::from(" Rustjack ").alignment(HorizontalAlignment::Left))
      .title_bottom(Line::from(content).alignment(HorizontalAlignment::Right)),
    frame.area(),
  );
}
