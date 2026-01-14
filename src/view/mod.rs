use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use ratatui::{
  Frame,
  layout::{HorizontalAlignment, VerticalAlignment},
  text::Line,
  widgets::Block,
};

use crate::game::{Game, GameStatus};

mod bet;
mod card;
mod constants;
mod deck;
mod fallback;
mod hand;
mod status;

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
  bet::render(frame, &game);
  status::render(frame, &game);

  let mut game_frame = Block::bordered()
    .title_top(Line::from(" RustJack ").alignment(HorizontalAlignment::Left))
    .title_bottom(Line::from("[^q] exit, [^r] reset").alignment(HorizontalAlignment::Left));

  match game.status {
    GameStatus::Betting => {}
    _ => {
      game_frame = game_frame.title_bottom(
        Line::from(game.player_hand.score().to_string()).alignment(HorizontalAlignment::Center),
      );  
    }
  }

  match game.status {
    GameStatus::DealerWon | GameStatus::PlayerWon | GameStatus::Draw => {
      game_frame = game_frame.title_top(
        Line::from(game.dealer_hand.score().to_string()).alignment(HorizontalAlignment::Center),
      );
    }
    _ => {}
  }

  frame.render_widget(game_frame, frame.area());
}

pub fn handle_key_event<'a>(key: KeyEvent, game: &mut Game) -> Result<(), ()> {
  match (key.modifiers, key.code) {
    (KeyModifiers::CONTROL, KeyCode::Char('q')) => Err(()),
    (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
      game.reset_balance();
      Ok(())
    }
    (_, KeyCode::Char('n')) => {
      game.reset();
      Ok(())
    }
    (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
      game.player_remove_deck();
      Ok(())
    }
    (_, KeyCode::Char('d')) => {
      game.player_add_deck();
      Ok(())
    }
    (KeyModifiers::CONTROL, KeyCode::Char('b')) => {
      game.player_decrease_bet();
      Ok(())
    }
    (_, KeyCode::Char('b')) => {
      game.player_increase_bet();
      Ok(())
    }
    (_, KeyCode::Char('s')) => {
      match &game.status {
        GameStatus::Betting => game.start(),
        GameStatus::PlayerTurn => game.player_stand(),
        _ => {}
      };
      Ok(())
    }
    (_, KeyCode::Char('h')) => {
      game.player_hit();
      Ok(())
    }
    _ => Ok(()),
  }
}
