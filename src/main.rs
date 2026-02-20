use std::{error::Error, time::Duration};
use crossterm::{
  event::{self, Event},
};

mod tui_app;
mod storage;

mod balance;
mod card;
mod deck;
mod game;
mod hand;
mod view;

use crate::{game::Game, tui_app::engine::Engine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut engine = Engine::init()?;
  let mut game = Game::new();

  loop {
    engine.instance.draw(|frame| view::render_game(frame, &game))?;

    if event::poll(Duration::from_millis(16))? {
      if let Event::Key(key) = event::read()? {
        match view::handle_key_event(key, &mut game) {
          Ok(()) => continue,
          Err(()) => break,
        }
      }
    }

    tokio::task::yield_now().await;
  }

  Ok(())
}
