use crossterm::event::KeyEvent;

use ratatui::Frame;
use crate::game::Game;

mod constants;
mod widget;
mod screen;

pub fn render_game(frame: &mut Frame, game: &Game) {
	if screen::fallback::check_view_port(frame) {
    screen::fallback::render(frame);
    return;
  }

  screen::game::render(frame, game);
}

pub fn handle_key_event<'a>(key: KeyEvent, game: &mut Game) -> Result<(), ()> {
  screen::game::handle_key_event(key, game)
}
