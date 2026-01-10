use std::cell::RefCell;
use std::rc::Rc;

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  text::{Line, Span, Text},
  widgets::{Block, Paragraph},
  style::{Color, Style},
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
    },
  );
  deck::render(frame);

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

  let mut content = vec![
    Line::from(format!("{} $", game.borrow().balance.borrow().player)),
    Line::from(""),
  ];

  match game.borrow().status {
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
      content.push(Line::from(Span::styled("You won!", Style::default().fg(Color::Green))));
      content.push(Line::from(""));
      content.push(Line::from("N - new game"));
      content.push(Line::from("R - reset balance"));
    }
    GameStatus::DealerWon => {
      content.push(Line::from(Span::styled("Dealer won!", Style::default().fg(Color::Red))));
      content.push(Line::from(""));
      content.push(Line::from("N - new game"));
      content.push(Line::from("R - reset balance"));
    }
    GameStatus::Draw => {
      content.push(Line::from(Span::styled("Draw!", Style::default().fg(Color::Yellow))));
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
    Block::bordered()
      .title_top(Line::from(" RustJack ").alignment(HorizontalAlignment::Left)),
    frame.area(),
  );
}

pub fn handle_key_event(
  key: KeyEvent,
  game: Rc<RefCell<Game>>,
  balance: Rc<RefCell<Balance>>,
) -> Result<(), ()> {
  match key.code {
    KeyCode::Char('n') => {
      balance.borrow_mut().divide_bet();
      game.replace(Game::new(balance.clone()));
      Ok(())
    }
    KeyCode::Char('q') => Err(()),
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
