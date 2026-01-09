use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::error::Error;
use std::time::Duration;

use crossterm::{
  event::{self, Event, KeyCode},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
  Terminal,
  backend::CrosstermBackend,
};

mod card;
mod deck;
mod hand;
mod game;
mod balance;
mod view;

use crate::{balance::Balance, game::Game};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let balance = Rc::new(RefCell::new(Balance::new()));
  let mut game = Game::new(balance.clone());

  loop {
    terminal.draw(|frame| view::render_game(frame, &game))?;

    if event::poll(Duration::from_millis(16))? {
      if let Event::Key(key) = event::read()? {
        match key.code {
          KeyCode::Char('n') => {
            balance.borrow_mut().divide_bet();
            game = Game::new(balance.clone());
          },
          KeyCode::Char('q') => { break; },
          KeyCode::Char('b') => {
            game.player_increase_bet();
          },
          KeyCode::Char('s') => {
            game.player_stand();
          },
          KeyCode::Char('h') => {
            game.player_hit();
          },
          _ => continue,
        }
      }
    }

    tokio::task::yield_now().await;
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  terminal.show_cursor()?;
  Ok(())
}
