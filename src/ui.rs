use ratatui::{
  Frame,
  layout::{HorizontalAlignment, Rect, VerticalAlignment},
  style::{Color, Style},
  text::{Line, Span},
  widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{
  card::{Card, CardRank, CardSuit},
  game::{Game, GameStatus},
  hand::Hand,
};

pub struct BuildCardOptions<'a> {
  pub hidden: bool,
  pub aligment: VerticalAlignment,
  pub dimentions: &'a CardDim,
  pub last: bool,
}

pub fn build_card(card: &Card, options: BuildCardOptions) -> Paragraph<'static> {
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

pub struct CardDim {
  width: u16,
  height: u16,
}

pub fn calc_card_dim(frame: &mut Frame) -> CardDim {
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

pub fn render_deck(frame: &mut Frame) {
  let card_dim = calc_card_dim(frame);
  let card = Card {
    suit: CardSuit::Spades,
    rank: CardRank::Ace,
  };

  frame.render_widget(
    build_card(
      &card,
      BuildCardOptions {
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

pub fn render_hand(frame: &mut Frame, hand: &Hand, aligment: VerticalAlignment) {
  let mut card_dim = calc_card_dim(frame);
  // Half of card is hidden by screen
  card_dim.height /= 2;

  let cards = hand.cards.iter().enumerate();
  let cards_count = cards.len() as u16;
  let cards_block_start = (frame.area().width / 2) - (cards_count * card_dim.width / 2);

  let y: u16 = match aligment {
    VerticalAlignment::Top => 0,
    VerticalAlignment::Bottom => frame.area().height - card_dim.height,
    _ => 0,
  };

  for (index, card) in cards {
    let is_hidden = aligment == VerticalAlignment::Top && index != 0;

    frame.render_widget(
      build_card(
        card,
        BuildCardOptions {
          hidden: is_hidden,
          aligment,
          dimentions: &card_dim,
          last: (index as u16) == cards_count - 1,
        },
      ),
      Rect {
        x: cards_block_start + (index as u16 * 10),
        y,
        width: card_dim.width,
        height: card_dim.height,
      },
    );
  }
}

pub fn render_game(frame: &mut Frame, game: &Game) {
  render_hand(frame, &game.dealer_hand, VerticalAlignment::Top);
  render_hand(frame, &game.player_hand, VerticalAlignment::Bottom);
  render_deck(frame);

  let mut content = String::new();

  match game.status {
    GameStatus::PlayerTurn => {
      content.push_str("Your turn. h - hit, s - stand");
    }
    GameStatus::PlayerWon => {
      content.push_str("You won! n - new game");
    }
    GameStatus::DealerWon => {
      content.push_str("Dealer won! n - new game");
    }
    GameStatus::DealerTurn => {
      content.push_str("Dealer turn");
    }
    _ => {}
  }

  frame.render_widget(
    Block::bordered()
      .title_top(Line::from("Rustjack").alignment(HorizontalAlignment::Left))
      .title_bottom(Line::from(content).alignment(HorizontalAlignment::Right)),
    frame.area(),
  );
}
