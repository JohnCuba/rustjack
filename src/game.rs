use crate::balance::Balance;
use crate::deck::Deck;
use crate::hand::Hand;

pub enum GameStatus {
  Betting,
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
  pub balance: Balance,
  max_decks: u16,
}

impl Game {
  pub fn new() -> Self {
    let mut game =  Self {
      deck: Deck::new(),
      player_hand: Hand::new(),
      dealer_hand: Hand::new(),
      status: GameStatus::Betting,
      balance: Balance::new(),
      max_decks: 8,
    };

    game.balance.increase_bet();

    return game;
  }

  pub fn start(&mut self) {
    self.deck.shuffle();

    for _ in 0..2 {
      if let Some(card) = self.deck.draw() {
        self.player_hand.push(card);
      }
      if let Some(card) = self.deck.draw() {
        self.dealer_hand.push(card);
      }
    }

    self.status = GameStatus::PlayerTurn;
  }

  pub fn reset(&mut self) {
    self.balance.divide_bet();
    self.deck = Deck::new();
    self.player_hand = Hand::new();
    self.dealer_hand = Hand::new();
    self.status = GameStatus::Betting;

    self.balance.increase_bet();
  }

  pub fn reset_balance(&mut self) {
    self.balance.reset();
    self.reset();
  }

  pub fn player_increase_bet(&mut self) {
    let GameStatus::Betting = self.status else {return};

    self.balance.increase_bet();
  }

  pub fn player_decrease_bet(&mut self) {
    let GameStatus::Betting = self.status else {return};

    self.balance.decrease_bet();
  }

  pub fn player_add_deck(&mut self) {
    let GameStatus::Betting = self.status else {return};

    if self.deck.get_decks_count() == self.max_decks {return};

    self.deck.add_deck();
  }

  pub fn player_remove_deck(&mut self) {
    let GameStatus::Betting = self.status else {return};

    if self.deck.get_decks_count() == 1 {return};

    self.deck.remove_deck();
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
    let GameStatus::PlayerTurn = self.status else {return};

    if let Some(card) = self.deck.draw() {
      self.player_hand.push(card);
    }

    if self.player_hand.score() > 21 {
      self.status = GameStatus::DealerWon;
      self.balance.dealer_take_bet();
      return;
    }
  }

  pub fn player_stand(&mut self) {
    let GameStatus::PlayerTurn = self.status else {return};

    self.status = GameStatus::DealerTurn;
    self.dealer_play();
  }

  fn determine_winner(&mut self) {
    let p_score = self.player_hand.score();
    let d_score = self.dealer_hand.score();

    if d_score > 21 || p_score > d_score {
      self.status = GameStatus::PlayerWon;
      self.balance.player_take_bet();
    } else if d_score > p_score {
      self.status = GameStatus::DealerWon;
      self.balance.dealer_take_bet();
    } else {
      self.status = GameStatus::Draw;
      self.balance.divide_bet();
    }
  }
}
