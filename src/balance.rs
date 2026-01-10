use sled::Db;

use crate::storage;

pub struct Balance {
  pub player: u32,
  pub bet: u32,
  db: Db,
}

impl Balance {
  pub fn new(db: Db) -> Self {
    let mut player_balance = 100;

    let res = (|| -> Result<(), Box<dyn std::error::Error>> {
      match storage::get_u32_value(&db, "player_balance")? {
        Some(val) => player_balance = val,
        None => {},
      }
      Ok(())
    })();

    if let Err(e) = res {
      eprintln!("Error: {}", e);
    }

    let balance = Self {
      player: player_balance,
      bet: 0,
      db,
    };

    return balance;
  }

  pub fn reset(&mut self) {
    self.player = 100;
    self.bet = 0;
    self.save();
  }

  fn save(&mut self) {
    let res = (|| -> Result<(), Box<dyn std::error::Error>> {
      self
        .db
        .insert("player_balance", &self.player.to_be_bytes())?;
      Ok(())
    })();

    if let Err(e) = res {
      eprintln!("Error: {}", e);
    }
  }

  pub fn increase_bet(&mut self) {
    if self.player == 0 {
      return;
    }

    self.player = std::cmp::max(self.player - 5, 0);
    self.bet += 5;
  }

  pub fn player_take_bet(&mut self) {
    self.player += self.bet;
    self.bet = 0;

    self.save();
  }

  pub fn dealer_take_bet(&mut self) {
    self.bet = 0;

    self.save();
  }

  pub fn divide_bet(&mut self) {
    self.player += self.bet / 2;
    self.bet = 0;

    self.save();
  }
}
