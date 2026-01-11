use std::io;
use std::error::Error;
use std::time::Duration;

use crossterm::{
  event::{self, Event},
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
  Terminal,
  backend::CrosstermBackend,
};

mod storage;

mod card;
mod deck;
mod hand;
mod game;
mod balance;
mod view;

use crate::{game::Game};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut game = Game::new();

  loop {
    terminal.draw(|frame| view::render_game(frame, &game))?;

    if event::poll(Duration::from_millis(16))? {
      if let Event::Key(key) = event::read()? {
        let res = view::handle_key_event(key, &mut game);

        match res {
          Ok(()) => continue,
          Err(()) => break,
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
