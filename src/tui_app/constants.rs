use std::time::Duration;

pub const POOL_TIMEOUT: Duration = Duration::from_millis(16);

pub enum InputResult {
  Continue,
  Exit,
}