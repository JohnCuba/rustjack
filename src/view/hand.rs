use ratatui::{
  Frame,
  layout::{Rect, VerticalAlignment},
  text::Text,
};

use crate::hand::Hand;
use crate::view::card;

pub struct RenderHandOptions<'a> {
  pub hand: &'a Hand,
  pub balance: Option<u32>,
  pub aligment: VerticalAlignment,
  pub show_only_first: bool,
}

pub fn render(frame: &mut Frame, options: RenderHandOptions) {
  let mut card_dim = card::calc_dim(frame);
  // Half of card is hidden by screen
  card_dim.height /= 2;

  let cards = options.hand.cards.iter().enumerate();
  let cards_count = cards.len() as u16;
  let cards_block_start = (frame.area().width / 2) - (cards_count * card_dim.width / 3);

  let cards_y: u16 = match options.aligment {
    VerticalAlignment::Top => 0,
    VerticalAlignment::Bottom => frame.area().height - card_dim.height,
    _ => 0,
  };

  for (index, card) in cards {
    let is_hidden = index != 0 && options.show_only_first;

    frame.render_widget(
      card::build(
        card,
        card::BuildCardOptions {
          hidden: is_hidden,
          aligment: options.aligment,
          dimentions: &card_dim,
          last: (index as u16) == cards_count - 1,
        },
      ),
      Rect {
        x: cards_block_start + ((index as u16 + 1) * card_dim.width / 3),
        y: cards_y,
        width: card_dim.width,
        height: card_dim.height,
      },
    );
  }

  options.balance.map(|balance| {
    let balance_y: u16 = match options.aligment {
      VerticalAlignment::Top => card_dim.height + 1,
      VerticalAlignment::Bottom => frame.area().height - card_dim.height - 2,
      _ => 0,
    };

    let balance_str = format!("{}$", balance);
    let balance_str_len = balance_str.len() as u16;

    frame.render_widget(
      Text::from(balance_str),
      Rect {
        x: frame.area().width / 2 - balance_str_len / 2,
        y: balance_y,
        width: 20,
        height: 1,
      },
    );
  });
}
