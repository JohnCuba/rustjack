use crate::deck::Deck;
use crate::hand::Hand;

pub enum GameStatus {
  PlayerTurn,
  DealerTurn,
  PlayerWon,
  DealerWon,
  Draw,
}

pub struct Game {
  pub deck: Deck,
  pub player_hand: Hand,
  pub dealer_hand: Hand,
  pub status: GameStatus,
}

impl Game {
  pub fn new() -> Self {
    let mut deck = Deck::new();
    deck.shuffle();

    let mut game = Self {
      deck,
      player_hand: Hand::new(),
      dealer_hand: Hand::new(),
      status: GameStatus::PlayerTurn,
    };

    game.initial_deal();
    return game;
  }

  fn initial_deal(&mut self) {
    for _ in 0..2 {
      if let Some(card) = self.deck.draw() {
        self.player_hand.push(card);
      }
      if let Some(card) = self.deck.draw() {
        self.dealer_hand.push(card);
      }
    }
  }

  fn dealer_play(&mut self) {
    while self.dealer_hand.score() < 17 {
      if let Some(card) = self.deck.draw() {
        self.dealer_hand.push(card);
      }
    }
    self.determine_winner();
  }

  pub fn player_hit(&mut self) {
    if let GameStatus::PlayerTurn = self.status {
      if let Some(card) = self.deck.draw() {
        self.player_hand.push(card);
      }

      if self.player_hand.score() > 21 {
        self.status = GameStatus::DealerWon;
        return;
      }

      self.dealer_play();
    }
  }

  pub fn player_stand(&mut self) {
    self.status = GameStatus::DealerTurn;
    self.dealer_play();
  }

  fn determine_winner(&mut self) {
    let p_score = self.player_hand.score();
    let d_score = self.dealer_hand.score();

    if d_score > 21 || p_score > d_score {
      self.status = GameStatus::PlayerWon;
    } else if d_score > p_score {
      self.status = GameStatus::DealerWon;
    } else {
      self.status = GameStatus::Draw;
    }
  }
}
