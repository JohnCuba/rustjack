use rand::seq::SliceRandom;
use rand::rng;

use crate::card::Card;
use crate::card::CardSuit;
use crate::card::CardRank;

pub struct Deck {
  pub cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Self {
    let mut cards = Vec::with_capacity(52);
    let suits = [CardSuit::Hearts, CardSuit::Diamonds, CardSuit::Clubs, CardSuit::Spades];
    let ranks = [
      CardRank::Two, CardRank::Three, CardRank::Four, CardRank::Five, CardRank::Six, CardRank::Seven,
      CardRank::Eight, CardRank::Nine, CardRank::Ten, CardRank::Jack, CardRank::Queen, CardRank::King, CardRank::Ace,
    ];

    for &suit in &suits {
      for &rank in &ranks {
        cards.push(Card { suit, rank });
      }
    }

    Deck { cards }
  }

  pub fn shuffle(&mut self) {
    let mut curr_rng = rng();
    self.cards.shuffle(&mut curr_rng);
  }

  pub fn draw(&mut self) -> Option<Card> {
    self.cards.pop()
  }
}
