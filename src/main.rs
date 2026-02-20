use std::error::Error;

mod storage;
mod tui_app;

mod balance;
mod card;
mod deck;
mod game;
mod hand;

use crate::{game::Game, tui_app::engine::Engine};

fn main() -> Result<(), Box<dyn Error>> {
  let mut engine = Engine::init()?;
  let mut game = Game::new();

  let result = tui_app::app::run(&mut engine.instance, &mut game);

  if let Err(err) = result {
    println!("application error: {err}");
  }

  Ok(())
}
