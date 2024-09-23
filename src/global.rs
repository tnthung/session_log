use crate::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;


/// The `Global` logger will share the same logger instance across the entire program that has the same
/// name. This is useful when the logger is configured once and used in multiple places. The logger
/// will be created with default configurations if it does not exist.
#[derive(Clone)]
pub struct Global(Arc<Logger>);


static GLOBAL: Lazy<Mutex<HashMap<Arc<str>, Global>>> = Lazy::new(|| Mutex::new(HashMap::new()));


impl Global {
  /// Get a logger with the given name. If the logger does not exist, a new logger will be created and
  /// registered with the given name & default configurations.
  pub fn new(name: &str) -> Self {
    let mut map = GLOBAL.lock().unwrap();

    if let Some(logger) = map.get(name) {
      return logger.clone();
    }

    let logger = Self(Arc::new(Logger::default(name)));
    map.insert(name.into(), logger.clone());
    logger
  }

  /// Register a logger with the given name.
  pub fn register<F: Formatter>(name: &str, config: Config) -> Self {
    let logger = Self(Arc::new(Logger::new::<F>(name, config)));
    GLOBAL.lock().unwrap().insert(name.into(), logger.clone());
    logger
  }

  /// Unregister a logger with the given name. Logger will be dropped if no other reference exists.
  pub fn unregister(name: &str) {
    GLOBAL.lock().unwrap().remove(name);
  }

  /// Create a session from the global logger without constructing a new logger first.
  #[track_caller]
  pub fn session(name: &str, session: &str, silent: bool) -> Session {
    Self::new(name).session(session, silent)
  }

  /// Create a new session from the session and execute the callable with the session passed in.
  ///
  /// This can be handy for isolating the environment of each callable while also providing a common
  /// logging interface for any callable that needs to be logged.
  #[track_caller]
  pub fn session_then<F, T>(&self, name: &str, silent: bool, callable: F) -> T
  where F: FnOnce(Session) -> T
  {
    callable(self.session(name, silent))
  }
}


impl std::ops::Deref for Global {
  type Target = Logger;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}
