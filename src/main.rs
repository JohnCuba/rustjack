use std::error::Error;

mod core;
mod tui_app;

use core::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
  let mut game = Game::new();

  let result = tui_app::app::run(&mut game);

  if let Err(err) = result {
    println!("application error: {err}");
  }

  Ok(())
}
