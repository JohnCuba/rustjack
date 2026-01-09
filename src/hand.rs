use crate::card::{Card, CardRank};

pub struct Hand {
  pub cards: Vec<Card>,
}

impl Hand {
  pub fn new() -> Self {
    Self { cards: Vec::new() }
  }

  pub fn push(&mut self, card: Card) {
    self.cards.push(card);
  }

  pub fn score(&self) -> u8 {
    let mut total = 0;
    let mut aces = 0;

    for card in &self.cards {
      total += card.value();
      if card.rank == CardRank::Ace {
        aces += 1;
      }
    }

    while total > 21 && aces > 0 {
      total -= 10;
      aces -= 1;
    }

    return total;
  }
}
