use std::error::Error;

mod storage;
mod tui_app;

mod balance;
mod card;
mod deck;
mod game;
mod hand;

use crate::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
  let mut game = Game::new();

  let result = tui_app::app::run(&mut game);

  if let Err(err) = result {
    println!("application error: {err}");
  }

  Ok(())
}
