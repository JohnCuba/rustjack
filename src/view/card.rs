use ratatui::{
  Frame,
  layout::VerticalAlignment,
  style::{Color, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::card::{Card, CardSuit};

pub struct CardDim {
  pub width: u16,
  pub height: u16,
}

pub fn calc_dim(frame: &mut Frame) -> CardDim {
  // 1.4 Real card aspect ratio / 2.0 terminal character aspect ratio
  let terminal_card_aspect_ratio: f32 = 0.7;

  // 15% screen width
  let mut card_width = (frame.area().width as f32 * 0.15) as u16;
  card_width = card_width.max(10);

  let mut card_height = (card_width as f32 * terminal_card_aspect_ratio) as u16;
  card_height = card_height.max(5);

  return CardDim {
    width: card_width,
    height: card_height,
  };
}

pub struct BuildCardOptions<'a> {
  pub hidden: bool,
  pub aligment: VerticalAlignment,
  pub dimentions: &'a CardDim,
  pub last: bool,
}

pub fn build(card: &Card, options: BuildCardOptions) -> Paragraph<'static> {
  let top_padding = match options.aligment {
    VerticalAlignment::Top => {
      if options.hidden {
        0
      } else {
        // card height - 2 (lines rank & suit) - 1 (borders)
        options.dimentions.height - 2 - 1
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

  match options.hidden {
    false => {
      let content_color = match card.suit {
        CardSuit::Hearts | CardSuit::Diamonds => Color::Red,
        CardSuit::Spades | CardSuit::Clubs => Color::Black,
      };

      content.push(Line::from(Span::styled(
        card.rank_str(),
        Style::default().fg(content_color),
      )));

      content.push(Line::from(Span::styled(
        card.suit_simbol(),
        Style::default().fg(content_color),
      )));

      if options.aligment == VerticalAlignment::Top {
        content.reverse();
      }
    }
    true => {
      let pattern = "▒░";
      for index in 0..options.dimentions.height - 1 {
        content.push(Line::from(Span::styled(
          match index & 1 == 1 {
            true => pattern.repeat(usize::from(options.dimentions.width - 2)),
            false => pattern
              .chars()
              .rev()
              .collect::<String>()
              .repeat(usize::from(options.dimentions.width - 2)),
          },
          Style::default(),
        )));
      }
    }
  }

  return Paragraph::new(content).block(card_block);
}
