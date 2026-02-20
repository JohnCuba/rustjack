use std::io::Result;

use crossterm::event::{self, Event};

use super::{engine::Engine, view, constants::POOL_TIMEOUT};
use crate::{core::game::Game, tui_app::constants::InputResult};

pub fn run(game: &mut Game) -> Result<()> {
  let mut engine = Engine::init()?;

  loop {
    engine
      .instance
      .draw(|frame| view::render_game(frame, &game))?;

    if event::poll(POOL_TIMEOUT)? {
      if let Event::Key(key) = event::read()? {
        match view::handle_key_event(key, game) {
          InputResult::Continue => continue,
          InputResult::Exit => return Ok(()),
        }
      }
    }
  }
}
