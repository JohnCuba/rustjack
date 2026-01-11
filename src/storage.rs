use sled::Db;

pub struct Storage {
  db: Option<Db>
}

impl Storage {
  pub fn new() -> Self {
    return Self {
      db: sled::open("rust_jack_db").ok(),
    }
  }

  pub fn set_u32(& self, key: &str, value: &u32) {
    let _ = match &self.db {
      Some(db) => db.insert(key, &value.to_be_bytes()),
      None => return,
    };
  }

  pub fn get_u32(& self, key: &str, default: u32) -> u32 {
    match &self.db {
      Some(db) => db.get(key).ok()
        .flatten()
        .and_then(|b| b.as_ref().try_into().ok())
        .map(u32::from_be_bytes)
        .unwrap_or(default),
      None => default
    }
  }
}
