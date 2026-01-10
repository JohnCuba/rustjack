use std::cell::RefCell;
use std::rc::Rc;

use crossterm::{
  event::{KeyCode, KeyEvent},
};

use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  text::{Line, Text},
  widgets::Block,
};

use crate::balance::Balance;
use crate::game::{Game, GameStatus};

mod card;
mod deck;
mod hand;

pub fn render_game(frame: &mut Frame, game: Rc<RefCell<Game>>) {
  hand::render(
    frame,
    hand::RenderHandOptions {
      hand: &game.borrow().dealer_hand,
      aligment: VerticalAlignment::Top,
      balance: None,
      show_only_first: match game.borrow().status {
        GameStatus::Draw | GameStatus::PlayerWon | GameStatus::DealerWon => false,
        _ => true,
      },
    },
  );
  hand::render(
    frame,
    hand::RenderHandOptions {
      hand: &game.borrow().player_hand,
      aligment: VerticalAlignment::Bottom,
      show_only_first: false,
      balance: Some(game.borrow().balance.borrow().player),
    }
  );
  deck::render(frame);

  let mut content = String::new();

  match game.borrow().status {
    GameStatus::Betting => {
      content.push_str(" Betting. B - increase bet on 5$, H - hit, S - stand ");
    }
    GameStatus::PlayerTurn => {
      content.push_str(" Your turn. H - hit, S - stand ");
    }
    GameStatus::PlayerWon => {
      content.push_str(" You won! N - new game ");
    }
    GameStatus::DealerWon => {
      content.push_str(" Dealer won! N - new game ");
    }
    GameStatus::Draw => {
      content.push_str(" Draw! N - new game ");
    }
    GameStatus::DealerTurn => {
      content.push_str(" Dealer turn ");
    }
  }

  let bet_text = format!("bet: {}$", game.borrow().balance.borrow().bet);
  let bet_text_len = bet_text.len() as u16;

  frame.render_widget(
    Text::from(format!("bet: {}$", game.borrow().balance.borrow().bet)),
    Rect {
      x: frame.area().width / 2 - bet_text_len / 2,
      y: frame.area().height / 2,
      width: bet_text_len,
      height: 1,
    },
  );

  frame.render_widget(
    Block::bordered()
      .title_top(Line::from(" RustJack ").alignment(HorizontalAlignment::Left))
      .title_bottom(Line::from(" R - reset balance ").alignment(HorizontalAlignment::Left))
      .title_bottom(Line::from(content).alignment(HorizontalAlignment::Right)),
    frame.area(),
  );
}

pub fn handle_key_event(key: KeyEvent, game: Rc<RefCell<Game>>, balance: Rc<RefCell<Balance>>) -> Result<(), ()> {
  match key.code {
    KeyCode::Char('n') => {
      balance.borrow_mut().divide_bet();
      game.replace(Game::new(balance.clone()));
      Ok(())
    }
    KeyCode::Char('q') => {
      Err(())
    }
    KeyCode::Char('r') => {
      balance.borrow_mut().reset();
      game.replace(Game::new(balance.clone()));
      Ok(())
    }
    KeyCode::Char('b') => {
      game.borrow_mut().player_increase_bet();
      Ok(())
    }
    KeyCode::Char('s') => {
      game.borrow_mut().player_stand();
      Ok(())
    }
    KeyCode::Char('h') => {
      game.borrow_mut().player_hit();
      Ok(())
    }
    _ => Ok(()),
  }
}
