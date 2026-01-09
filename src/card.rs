#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardSuit {
  Hearts,
  Diamonds,
  Clubs,
  Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardRank {
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}

pub struct Card {
  pub suit: CardSuit,
  pub rank: CardRank,
}

impl Card {
  pub fn value(&self) -> u8 {
    match self.rank {
      CardRank::Two => 2,
      CardRank::Three => 3,
      CardRank::Four => 4,
      CardRank::Five => 5,
      CardRank::Six => 6,
      CardRank::Seven => 7,
      CardRank::Eight => 8,
      CardRank::Nine => 9,
      CardRank::Ten | CardRank::Jack | CardRank::Queen | CardRank::King => 10,
      CardRank::Ace => 11,
    }
  }

  pub fn rank_str(&self) -> &'static str {
    match self.rank {
      CardRank::Two => "2",
      CardRank::Three => "3",
      CardRank::Four => "4",
      CardRank::Five => "5",
      CardRank::Six => "6",
      CardRank::Seven => "7",
      CardRank::Eight => "8",
      CardRank::Nine => "9",
      CardRank::Ten => "10",
      CardRank::Jack => "J",
      CardRank::Queen => "Q",
      CardRank::King => "K",
      CardRank::Ace => "A",
    }
  }

  pub fn suit_simbol(&self) -> &'static str {
    match self.suit {
      CardSuit::Hearts => "♥",
      CardSuit::Diamonds => "♦",
      CardSuit::Spades => "♠",
      CardSuit::Clubs => "♣",
    }
  }
}
