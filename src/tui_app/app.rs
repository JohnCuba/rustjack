use std::{io::Result, time::Duration};

use crossterm::event::{self, Event};

use super::{engine::Engine, view};
use crate::core::game::Game;

pub fn run(game: &mut Game) -> Result<()> {
  let mut engine = Engine::init()?;

  loop {
    engine
      .instance
      .draw(|frame| view::render_game(frame, game))?;

    if event::poll(Duration::from_millis(16))? {
      if let Event::Key(key) = event::read()? {
        match view::handle_key_event(key, game) {
          Ok(()) => continue,
          Err(()) => break,
        }
      }
    }
  }

  Ok(())
}
