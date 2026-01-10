use ratatui::{
  Frame,
  layout::{Rect, VerticalAlignment},
};

use crate::hand::Hand;
use crate::view::card;

pub struct RenderHandOptions<'a> {
  pub hand: &'a Hand,
  pub aligment: VerticalAlignment,
  pub show_only_first: bool,
}

pub fn render(frame: &mut Frame, options: RenderHandOptions) {
  let mut card_dim = card::calc_dim(frame);
  // Half of card is hidden by screen
  card_dim.height = (card_dim.height / 2).max(5);

  let cards = options.hand.cards.iter().enumerate();
  let cards_count = cards.len() as u16;
  let card_visible_path = card_dim.width / 3;
  // (screen center) - (hand cards visible paths) - (full last visible path)
  let cards_block_start = (frame.area().width / 2)
    - ((cards_count - 1) * card_visible_path)
    - (card_dim.width - card_visible_path);

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
}
