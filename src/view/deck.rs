use ratatui::{
  Frame,
  layout::{Rect, VerticalAlignment},
};

use crate::card::{Card, CardRank, CardSuit};
use crate::view::card;

pub fn render(frame: &mut Frame) {
  let card_dim = card::calc_dim(frame);
  let card = Card {
    suit: CardSuit::Spades,
    rank: CardRank::Ace,
  };

  frame.render_widget(
    card::build(
      &card,
      card::BuildCardOptions {
        hidden: true,
        aligment: VerticalAlignment::Center,
        dimentions: &card_dim,
        last: true,
      },
    ),
    Rect {
      x: frame.area().width - 1 - card_dim.width,
      y: frame.area().height / 2 - card_dim.height / 2,
      width: card_dim.width,
      height: card_dim.height,
    },
  );
}
