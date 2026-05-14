use std::collections::HashMap;
use std::time::Instant;
use std::sync::{Mutex, LazyLock};


#[derive(Debug, Clone)]
pub(crate) struct Metadata {
  pub current_id: u128,
  pub parent_id:  Option<u128>,
  pub name:       Option<String>,
  pub nest_level: usize,
  pub start_time: Instant,
}

static SESSION_METADATA: LazyLock<Mutex<HashMap<u128, Metadata>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

impl Metadata {
  pub fn get(sid: u128) -> Option<Metadata> {
    SESSION_METADATA.lock().unwrap()
      .get(&sid).cloned()
  }
}
