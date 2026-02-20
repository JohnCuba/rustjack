use std::io::{self, Stdout};

use crossterm::{
  execute,
  terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

pub struct Engine {
  pub instance: Terminal<CrosstermBackend<Stdout>>,
}

impl Engine {
  pub fn init() -> io::Result<Self> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen,).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    return Ok(Engine { instance: terminal });
  }
}

impl Drop for Engine {
  fn drop(&mut self) {
    let _ = disable_raw_mode();
    let _ = execute!(self.instance.backend_mut(), LeaveAlternateScreen);
    let _ = self.instance.show_cursor();
  }
}
