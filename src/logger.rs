use super::formatter::FormatterTrait;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Mutex;
use log::Log;

pub use log::Level;


pub struct Logger {
  log_level:  AtomicU8,
  formatters: Mutex<Vec<Box<dyn FormatterTrait>>>,
}

static LOGGER: Logger = Logger {
  log_level:  AtomicU8::new(level_rank(Level::Info)),
  formatters: Mutex::new(Vec::new()),
};

impl Logger {
  pub fn set_log_level(level: Level) {
    LOGGER.log_level.store(level_rank(level), Ordering::Relaxed);
    log::set_max_level(level.to_level_filter());
  }

  pub(crate) fn add_formatter(formatter: Box<dyn FormatterTrait>) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| log::set_logger(&LOGGER).expect("Failed to set logger"));
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
    level_rank(metadata.level()) <= self.log_level.load(Ordering::Relaxed)
  }

  fn log(&self, record: &log::Record) {
    if !self.enabled(record.metadata()) { return; }
    Self::for_each(|fmt| fmt.add_log(record));
  }

  fn flush(&self) {
    Self::for_each(|fmt| fmt.flush());
  }
}


const fn level_rank(level: Level) -> u8 {
  level as usize as u8
}
