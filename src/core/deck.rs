use rand::rng;
use rand::seq::SliceRandom;

use super::card::{Card, CardRank, CardSuit};

pub struct Deck {
  pub cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Self {
    let mut deck = Deck {
      cards: Vec::with_capacity(416),
    };

    deck.add_deck();

    return deck;
  }

  pub fn get_decks_count(&mut self) -> u16 {
    let cards_count = self.cards.len() as u16;
    return cards_count / 52;
  }

  pub fn add_deck(&mut self) {
    let suits = [
      CardSuit::Hearts,
      CardSuit::Diamonds,
      CardSuit::Clubs,
      CardSuit::Spades,
    ];
    let ranks = [
      CardRank::Two,
      CardRank::Three,
      CardRank::Four,
      CardRank::Five,
      CardRank::Six,
      CardRank::Seven,
      CardRank::Eight,
      CardRank::Nine,
      CardRank::Ten,
      CardRank::Jack,
      CardRank::Queen,
      CardRank::King,
      CardRank::Ace,
    ];

    for &suit in &suits {
      for &rank in &ranks {
        self.cards.push(Card { suit, rank });
      }
    }
  }

  pub fn remove_deck(&mut self) {
    let cards_count = self.cards.len();
    let _ = self.cards.split_off(cards_count - 52);
  }

  pub fn shuffle(&mut self) {
    let mut curr_rng = rng();
    self.cards.shuffle(&mut curr_rng);
  }

  pub fn draw(&mut self) -> Option<Card> {
    self.cards.pop()
  }
}
