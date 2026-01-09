use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  text::{Line, Span},
  widgets::{Block, Paragraph},
  style::{Style, Color},
};

use crate::{card::Card, deck::Deck, game::{Game, GameStatus}, hand::Hand};

pub fn build_card(card: &Card, hidden: bool) -> Paragraph<'static> {
  let card_block = Block::bordered().style(Style::default().bg(Color::DarkGray));
  let mut content = vec![];

  if !hidden {
    content.push(Line::from(Span::styled(card.rank_str(), Style::default())));
    content.push(Line::from(Span::styled(card.suit_simbol(), Style::default())));
  }

  let context_widget = Paragraph::new(content).block(card_block);

  return context_widget;
}

pub fn render_deck(frame: &mut Frame, deck: &Deck) {
  let card_width = (f32::from(frame.area().width) * 0.1) as u16;
  let card_height = (f32::from(frame.area().height) * 0.3) as u16 / 2;
  let cards = deck.cards.iter().enumerate();
  let cards_block_start = frame.area().width - (cards.len() as u16) - card_width;

  for (index, card) in cards {
    frame.render_widget(
      build_card(card, true),
      Rect {
        x: cards_block_start + (index as u16),
        y: frame.area().height / 2 - card_height / 2,
        width: card_width,
        height: card_height,
      },
    );
  }
}

pub fn render_hand(frame: &mut Frame, hand: &Hand, aligment: VerticalAlignment) {
  let card_width = (f32::from(frame.area().width) * 0.1) as u16;
  let card_height = (f32::from(frame.area().height) * 0.3) as u16 / 2;

  let cards = hand.cards.iter().enumerate();
  let cards_block_start = (frame.area().width / 2) - (cards.len() as u16 * card_width / 2);

  let y: u16 = match aligment {
    VerticalAlignment::Top => 0,
    VerticalAlignment::Bottom => frame.area().height - card_height,
    _ => 0,
  };

  for (index, card) in cards {
    let is_hidden = aligment == VerticalAlignment::Top && index != 0;

    frame.render_widget(
      build_card(card, is_hidden),
      Rect {
        x: cards_block_start + (index as u16 * 10),
        y,
        width: card_width,
        height: card_height,
      },
    );
  }
}

pub fn render_game(frame: &mut Frame, game: &Game) {

  render_hand(frame, &game.dealer_hand, VerticalAlignment::Top);
  render_hand(frame, &game.player_hand, VerticalAlignment::Bottom);
  render_deck(frame, &game.deck);

  let mut content = String::new();

  match game.status {
    GameStatus::PlayerTurn => {
      content.push_str("Your turn. h - hit, s - stand");
    },
    GameStatus::PlayerWon => {
      content.push_str("You won! n - new game");
    },
    GameStatus::DealerWon => {
      content.push_str("Dealer won! n - new game");
    },
    GameStatus::DealerTurn => {
      content.push_str("Dealer turn");
    }
    _ => { },
  }

  frame.render_widget(
    Block::bordered()
      .title_top(Line::from("Rustjack").alignment(HorizontalAlignment::Left))
      .title_bottom(Line::from(content).alignment(HorizontalAlignment::Right)),
    frame.area(),
  );
}
