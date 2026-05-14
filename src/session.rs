use std::collections::HashMap;
use std::time::Instant;
use std::sync::{Mutex, LazyLock};
use uuid::Uuid;


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

  fn new(name: Option<String>, parent_id: Option<u128>) -> Self {
    let nest_level = parent_id.as_ref()
      .and_then(|pid| SESSION_METADATA.lock().unwrap()
        .get(pid).map(|parent| parent.nest_level + 1))
      .unwrap_or(1);

    let ret = Self {
      name,
      parent_id,
      nest_level,
      current_id: Uuid::now_v7().to_u128_le(),
      start_time: Instant::now(),
    };

    SESSION_METADATA.lock().unwrap()
      .insert(ret.current_id, ret.clone());

    ret
  }
}
