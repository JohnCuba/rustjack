use sled::Db;
use std::{env::home_dir, path::PathBuf};

pub struct Storage {
  db: Option<Db>,
}

impl Storage {
  pub fn new() -> Self {
    let binding = home_dir().unwrap_or(PathBuf::new());
    let root_path = binding.to_str().unwrap_or(".");
    return Self {
      db: sled::open(format!("{}/.config/rustjack/db", root_path)).ok(),
    };
  }

  pub fn set_u32(&self, key: &str, value: &u32) {
    let _ = match &self.db {
      Some(db) => db.insert(key, &value.to_be_bytes()),
      None => return,
    };
  }

  pub fn get_u32(&self, key: &str, default: u32) -> u32 {
    match &self.db {
      Some(db) => db
        .get(key)
        .ok()
        .flatten()
        .and_then(|b| b.as_ref().try_into().ok())
        .map(u32::from_be_bytes)
        .unwrap_or(default),
      None => default,
    }
  }
}
