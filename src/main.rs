use std::io;
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

use crate::game::Game;

mod card;
mod deck;
mod game;
mod hand;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut game = Game::new();

  loop {
    terminal.draw(|frame| ui::render_game(frame, &game))?;

    if event::poll(Duration::from_millis(16))? {
      if let Event::Key(key) = event::read()? {
        match key.code {
          KeyCode::Char('n') => {
            game = Game::new();
          },
          KeyCode::Char('q') => { break; },
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
