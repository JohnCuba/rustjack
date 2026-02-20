use ratatui::{
  Frame,
  layout::VerticalAlignment,
  style::{Color, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{
  core::card::{Card, CardSuit},
  tui_app::view::constants::Dimension,
};

pub fn calc_dim(frame: &mut Frame) -> Dimension {
  // 1.4 Real card aspect ratio / 2.0 terminal character aspect ratio
  let terminal_card_aspect_ratio: f32 = 0.7;

  // 25% screen width
  let card_width = ((frame.area().width as f32 * 0.25) as u16).min(24).max(8);

  let mut card_height = (card_width as f32 * terminal_card_aspect_ratio) as u16;
  card_height = card_height.max(5);

  return Dimension {
    width: card_width,
    height: card_height,
  };
}

pub struct BuildCardOptions<'a> {
  pub hidden: bool,
  pub aligment: VerticalAlignment,
  pub dimension: &'a Dimension,
  pub last: bool,
}

pub fn build(card: &Card, options: BuildCardOptions) -> Paragraph<'static> {
  let top_padding = match options.aligment {
    VerticalAlignment::Top => {
      if options.hidden {
        0
      } else {
        // card height - 2 (lines rank & suit) - 1 (borders)
        options.dimension.height - 2 - 1
      }
    }
    _ => 0,
  };

  let card_block = Block::new()
    .borders(
      match options.aligment {
        VerticalAlignment::Top => Borders::BOTTOM | Borders::LEFT,
        VerticalAlignment::Bottom => Borders::TOP | Borders::LEFT,
        VerticalAlignment::Center => Borders::TOP | Borders::BOTTOM | Borders::LEFT,
      } | if options.last {
        Borders::RIGHT
      } else {
        Borders::NONE
      },
    )
    .border_type(BorderType::Rounded)
    .padding(Padding::new(1, 1, top_padding, 0));

  let mut content = vec![];

  if options.hidden {
    let pattern = "▒░";
    for index in 0..options.dimension.height - 1 {
      content.push(Line::from(Span::styled(
        if index & 1 == 1 {
          pattern.repeat(usize::from(options.dimension.width - 2))
        } else {
          pattern
            .chars()
            .rev()
            .collect::<String>()
            .repeat(usize::from(options.dimension.width - 2))
        },
        Style::default(),
      )));
    }
  } else {
    let content_color = match card.suit {
      CardSuit::Hearts | CardSuit::Diamonds => Color::Red,
      CardSuit::Spades | CardSuit::Clubs => Color::White,
    };

    content.extend(vec![
      Line::from(Span::styled(
        card.rank_str(),
        Style::default().fg(content_color).bold(),
      )),
      Line::from(Span::styled(
        card.suit_simbol(),
        Style::default().fg(content_color).bold(),
      )),
    ]);

    if options.aligment == VerticalAlignment::Top {
      content.reverse();
    }
  }

  return Paragraph::new(content).block(card_block);
}
