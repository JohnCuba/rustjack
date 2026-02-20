use super::storage::Storage;

pub struct Balance {
  pub player: u32,
  pub bet: u32,
  storage: Storage,
}

impl Balance {
  pub fn new() -> Self {
    let storage = Storage::new();

    return Self {
      player: storage.get_u32("player_balance", 100),
      bet: 0,
      storage,
    };
  }

  pub fn reset(&mut self) {
    self.player = 100;
    self.bet = 0;
    self.save();
  }

  fn save(&mut self) {
    self.storage.set_u32("player_balance", &self.player);
  }

  pub fn increase_bet(&mut self) {
    if self.player == 0 {
      return;
    }

    self.player = std::cmp::max(self.player - 5, 0);
    self.bet += 10;
  }

  pub fn decrease_bet(&mut self) {
    if self.bet == 10 {
      return;
    }

    self.player = self.player + 5;
    self.bet -= 10;
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
