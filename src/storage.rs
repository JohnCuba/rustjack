use sled::Db;

pub fn get_u32_value(db: &Db, key: &str) -> Result<Option<u32>, Box<dyn std::error::Error>> {
  match db.get(key)? {
    Some(bytes) => {
      let array = bytes
        .as_ref()
        .try_into()
        .map_err(|_| "Value in DB is not u32 (wrong length)")?;

      Ok(Some(u32::from_be_bytes(array)))
    }
    None => Ok(None),
  }
}
