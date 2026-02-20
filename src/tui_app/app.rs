use std::{
  io::{Result, Stdout},
  time::Duration,
};

use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::CrosstermBackend};

use crate::game::Game;
use super::view;

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, game: &mut Game) -> Result<()> {
  loop {
    terminal.draw(|frame| view::render_game(frame, game))?;

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
