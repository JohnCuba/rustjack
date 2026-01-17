use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect},
  widgets::{Paragraph, Wrap},
};

use crate::view::constants::{MIN_VIEW};

pub fn check_view_port(frame: &mut Frame) -> bool {
  return frame.area().width < MIN_VIEW.width || frame.area().height < MIN_VIEW.height;
}

pub fn render(frame: &mut Frame) {
  let content = format!(
    "Terminal window must be at least {}x{}, now {}x{}",
    MIN_VIEW.width,
    MIN_VIEW.height,
    frame.area().width,
    frame.area().height,
  );
  let content_len = content.len() as u16;
  frame.render_widget(
    Paragraph::new(content)
      .alignment(HorizontalAlignment::Center)
      .wrap(Wrap { trim: true })
      .scroll((0, 0)),
    Rect {
      x: 0,
      y: (frame.area().height / 2) - content_len / frame.area().width,
      width: frame.area().width,
      height: frame.area().height,
    },
  );
}
