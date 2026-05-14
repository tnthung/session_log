use super::formatter::FormatterTrait;
use std::sync::{Mutex, RwLock};
use log::Log;

pub use log::Level;


pub struct Logger {
  log_level:  RwLock<Level>,
  formatters: Mutex<Vec<Box<dyn FormatterTrait>>>,
}

static LOGGER: Logger = Logger {
  log_level:  RwLock::new(Level::Info),
  formatters: Mutex::new(Vec::new()),
};

impl Logger {
  pub fn set_log_level(level: Level) {
    LOGGER.log_level.write().unwrap().clone_from(&level);
    log::set_max_level(level.to_level_filter());
  }

  pub(crate) fn add_formatter(formatter: Box<dyn FormatterTrait>) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| { log::set_logger(&LOGGER).unwrap(); });
    LOGGER.formatters.lock().unwrap().push(formatter);
  }

  fn for_each(f: impl Fn(&mut Box<dyn FormatterTrait>)) {
    for formatter in &mut *LOGGER.formatters.lock().unwrap() {
      f(formatter);
    }
  }
}

impl Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= *self.log_level.read().unwrap()
  }

  fn log(&self, record: &log::Record) {
    if !self.enabled(record.metadata()) { return; }
    Self::for_each(|fmt| fmt.add_log(record));
  }

  fn flush(&self) {
    Self::for_each(|fmt| fmt.flush());
  }
}
