use ratatui::{
  Frame,
  layout::{Rect, VerticalAlignment},
  text::Line,
  widgets::{Paragraph, Wrap},
};

use crate::card::{Card, CardRank, CardSuit};
use crate::{
  game::{Game, GameStatus},
  view::card,
};

pub fn render(frame: &mut Frame, game: &Game) {
  let card_dim = card::calc_dim(frame);
  let card = Card {
    suit: CardSuit::Spades,
    rank: CardRank::Ace,
  };

  let x = frame.area().width - 1 - card_dim.width;
  let y = frame.area().height / 2 - card_dim.height / 2;

  frame.render_widget(
    card::build(
      &card,
      card::BuildCardOptions {
        hidden: true,
        aligment: VerticalAlignment::Center,
        dimension: &card_dim,
        last: true,
      },
    ),
    Rect {
      x,
      y,
      width: card_dim.width,
      height: card_dim.height,
    },
  );

  render_help(frame, game, x, y);
}

fn render_help(frame: &mut Frame, game: &Game, x: u16, y: u16) {
  let decks_count = &game.deck.cards.len() / 52;

  let mut content = vec![];

  match &game.status {
    GameStatus::Betting => {
      content.extend(vec![
        Line::from(format!(
          "{} decks ({} cards)",
          decks_count,
          game.deck.cards.len()
        )),
        Line::from("[d] add / [^d] rem"),
      ]);
    }
    _ => {}
  }

  let content_len = content.len() as u16;

  frame.render_widget(
    Paragraph::new(content)
      .wrap(Wrap { trim: true })
      .scroll((0, 0)),
    Rect {
      x,
      y: y - content_len,
      width: frame.area().width - x,
      height: content_len,
    },
  );
}
