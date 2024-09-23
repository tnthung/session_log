use crate::*;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};


/// Configuration for the logger.
#[derive(Debug, Clone)]
pub struct Config {
  /// The level that the logger will accept for writing to a file.
  ///
  /// Default: `Level::Info`
  pub write_level: Level,

  /// The level that the logger will accept for printing to the console.
  ///
  /// Default: `Level::Info`
  pub print_level: Level,

  /// The directory where the log files will be stored. The directory along with prefix will be used
  /// as a key for getting the writer. If 2 logger having same `directory` and `file_prefix`, the writer
  /// will be the same one regardless of if limits are different. This is to prevent multiple writers
  /// working on the same file.
  ///
  /// Default: `./logs`
  pub directory: String,

  /// The prefix of the log file. The log file will be named as `{prefix} {date}_{time}.log`.
  /// If the field is None, the log file will be named as `{date}_{time}.log`. Error of unable to
  /// create the log file with the unsupported character in prefix is not handled.
  ///
  /// If one logger having no `prefix`, `size_limit` and `duration_limit` the file name will always
  /// be `log.log`.
  ///
  /// Default: `None`
  pub file_prefix: Option<String>,

  /// The size limit in bytes of the log file. When the file size exceeds this limit, the file will
  /// be rotated. If the 0 is set, the file will never be rotated by size.
  ///
  /// Default: `10 * 1024 * 1024`
  pub size_limit: Option<u64>,

  /// The duration limit in seconds of the log file. When the file duration exceeds this limit, the
  /// file will be rotated. If the 0 is set, the file will never be rotated by duration.
  pub duration_limit: Option<u64>,

  /// The maximum size in bytes of one batch of logs. If the 0 is set, the logs will be written
  /// immediately.
  ///
  /// Default: `1024`
  #[cfg(feature = "batch")]
  pub batch_size: u64,

  /// The interval in milliseconds between each batch write. If the 0 is set, the value will be set
  /// to 1 millisecond to automatically.
  ///
  /// Default: `100`
  #[cfg(feature = "batch")]
  pub batch_interval: u64,
}


static DEFAULT_CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config {
  write_level   : Level::Info,
  print_level   : Level::Info,
  directory     : "./logs".to_string(),
  file_prefix   : None,
  size_limit    : Some(10 * 1024 * 1024),
  duration_limit: Some(60 * 60),

  #[cfg(feature = "batch")] batch_size    : 1024,
  #[cfg(feature = "batch")] batch_interval: 100,
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

      #[cfg(feature = "batch")] batch_size    : config.batch_size,
      #[cfg(feature = "batch")] batch_interval: config.batch_interval,
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

    #[cfg(feature = "batch")] {
      default.batch_size     = config.batch_size;
      default.batch_interval = config.batch_interval;
    }
  }
}
