use crate::*;
use std::time::Duration;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};


/// Configuration for the logger.
pub struct Config {
  /// The level that the logger will accept for writing to a file.
  pub write_level: Level,

  /// The level that the logger will accept for printing to the console.
  pub print_level: Level,

  /// The directory where the log files will be stored.
  pub directory: String,

  /// The formatter that will be used to format the log messages.
  pub formatter: &'static dyn Formatter,

  /// The size limit of the log file. When the file size exceeds this limit, the file will be rotated.
  pub size_limit: Option<u64>,

  /// The duration limit of the log file. When the file duration exceeds this limit, the file will be rotated.
  pub duration_limit: Option<Duration>,
}


static DEFAULT_CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config {
  write_level   : Level::Info,
  print_level   : Level::Info,
  directory     : "./log".to_string(),
  formatter     : &DefaultFormatter,
  size_limit    : Some(10 * 1024 * 1024),
  duration_limit: Some(Duration::from_secs(60*60)),
})));


impl Default for Config {
  fn default() -> Self {
    let config = DEFAULT_CONFIG.lock().unwrap();

    Config {
      write_level   : config.write_level,
      print_level   : config.print_level,
      directory     : config.directory.clone(),
      formatter     : config.formatter,
      size_limit    : config.size_limit,
      duration_limit: config.duration_limit,
    }
  }
}


impl Config {
  pub fn update_default(config: Config) {
    let mut default = DEFAULT_CONFIG.lock().unwrap();
    default.write_level    = config.write_level;
    default.print_level    = config.print_level;
    default.directory      = config.directory;
    default.formatter      = config.formatter;
    default.size_limit     = config.size_limit;
    default.duration_limit = config.duration_limit;
  }
}
