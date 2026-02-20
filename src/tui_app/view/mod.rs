use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use ratatui::Frame;

use crate::{core::game::Game, tui_app::constants::InputResult};

mod constants;
mod screen;
mod widget;

pub fn render_game(frame: &mut Frame, game: &Game) {
  if screen::fallback::check_view_port(frame) {
    screen::fallback::render(frame);
    return;
  }

  screen::game::render(frame, game);
}

pub fn handle_key_event<'a>(key: KeyEvent, game: &mut Game) -> InputResult {
  match (key.modifiers, key.code) {
    (KeyModifiers::CONTROL, KeyCode::Char('c')) => InputResult::Exit,
    _ => screen::game::handle_key_event(key, game)
  }
}
