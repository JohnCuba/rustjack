pub struct Balance {
  pub player: u32,
  pub dealer: u32,
  pub bet: u32,
}

impl Balance {
  pub fn new() -> Self {
    let balance = Self {
      player: 100,
      dealer: 100,
      bet: 0,
    };

    return balance;
  }

  pub fn increase_bet(&mut self) {
    if self.player == 0 || self.dealer == 0 {
      return;
    }

    self.player = std::cmp::max(self.player - 5, 0);
    self.dealer = std::cmp::max(self.dealer - 5, 0);
    self.bet += 10;
  }

  pub fn player_take_bet(&mut self) {
    self.player += self.bet;
    self.bet = 0;
  }

  pub fn dealer_take_bet(&mut self) {
    self.dealer += self.bet;
    self.bet = 0;
  }

  pub fn divide_bet(&mut self) {
    let profit = self.bet / 2;
    self.player += profit;
    self.dealer += profit;
    self.bet = 0;
  }
}
