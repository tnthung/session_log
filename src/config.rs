use crate::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};


/// Configuration for the logger.
#[derive(Debug, Clone)]
pub struct Config {
  /// The level that the logger will accept for writing to a file.
  pub write_level: Level,

  /// The level that the logger will accept for printing to the console.
  pub print_level: Level,

  /// The directory where the log files will be stored.
  pub directory: String,

  /// The prefix of the log file. The log file will be named as `{prefix} {date}_{time}.log`.
  /// If the field is None, the log file will be named as `{date}_{time}.log`. Error of unable to
  /// create the log file with the unsupported character in prefix is not handled.
  pub file_prefix: Option<String>,

  /// The size limit of the log file. When the file size exceeds this limit, the file will be rotated.
  pub size_limit: Option<u64>,

  /// The duration limit in seconds of the log file. When the file duration exceeds this limit, the
  /// file will be rotated.
  pub duration_limit: Option<u64>,
}


static DEFAULT_CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config {
  write_level   : Level::Info,
  print_level   : Level::Info,
  directory     : "./logs".to_string(),
  file_prefix   : None,
  size_limit    : Some(10 * 1024 * 1024),
  duration_limit: Some(60 * 60),
})));


impl Default for Config {
  fn default() -> Self {
    let config = DEFAULT_CONFIG.lock().unwrap();

    Config {
      write_level   : config.write_level,
      print_level   : config.print_level,
      directory     : config.directory.clone(),
      file_prefix   : config.file_prefix.clone(),
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
    default.file_prefix    = config.file_prefix;
    default.size_limit     = config.size_limit;
    default.duration_limit = config.duration_limit;
  }
}
