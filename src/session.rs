use std::collections::HashMap;
use std::time::Instant;
use std::sync::{Mutex, LazyLock};


#[derive(Debug, Clone)]
pub(crate) struct Metadata {
  pub name:       Option<String>,
  pub current_id: Option<String>,
  pub parent_id:  Option<String>,
  pub nest_level: usize,
  pub start_time: Instant,
}

static SESSION_METADATA: LazyLock<Mutex<HashMap<String, Metadata>>> =
  LazyLock::new(|| Mutex::new(HashMap::new()));

impl Metadata {
  pub fn get(sid: &str) -> Option<Metadata> {
    SESSION_METADATA.lock().unwrap()
      .get(sid).cloned()
  }
}
