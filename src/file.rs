use crate::config::Config;
use crate::logger::Logger;
use std::io::Write;
use std::fs::File as StdFile;
use std::sync::{Arc, Mutex};
use chrono::{Local, Timelike};


#[allow(non_upper_case_globals)]
pub const KiB: u64 = 1024;

#[allow(non_upper_case_globals)]
pub const MiB: u64 = 1024 * KiB;

#[allow(non_upper_case_globals)]
pub const GiB: u64 = 1024 * MiB;


#[allow(non_upper_case_globals)]
pub const Min: u32 = 60;

#[allow(non_upper_case_globals)]
pub const Hr: u32 = 60 * Min;

#[allow(non_upper_case_globals)]
pub const Day: u32 = 24 * Hr;


/// Configuration for the logger.
#[derive(Debug, Clone)]
pub struct FileConfig {
  /// The directory where the log files will be stored in.
  ///
  /// Default: `./logs`
  pub directory: String,

  /// The prefix of the log file name.
  ///
  /// Default: `None`
  pub prefix: Option<String>,

  /// The size limit in bytes of the log file. When the file size exceeds this limit, the file will
  /// be rotated. If the 0 is set, the file will never be rotated by size.
  ///
  /// Default: `10 MiB`
  pub size_limit: u64,

  /// The duration hint in seconds of the log file. The hint is used to calculate the nearest duration
  /// that can divide 84600 seconds (1 day) without any remainder. Whenever the limit is reached, the
  /// file will be rotated. If the 0 is set, the file will never be rotated by duration.
  ///
  /// The duration will cap at 1 day.
  ///
  /// Default: `1 Hr`
  pub duration_hint: u32,
}


impl FileConfig {
  pub fn duration(&self) -> i64 {
    if self.duration_hint == 0 { return 0; }
    let hint = self.duration_hint;
    (Day / (Day / hint)) as i64
  }

  pub fn create(&self) -> File {
    File::new(self.clone())
  }
}


impl Default for FileConfig {
  fn default() -> Self {
    Self {
      directory    : "./logs".to_string(),
      prefix       : None,
      size_limit   : 10 * MiB,
      duration_hint: 1  * Hr,
    }
  }
}


#[derive(Debug, Clone)]
pub struct File(Arc<(Mutex<Inner>, FileConfig)>);


#[derive(Debug)]
struct Inner {
  file: StdFile,
  size: u64,
  dura: i64,
  part: i64,
  secs: i64,
}


impl File {
  pub(crate) fn new(config: FileConfig) -> Self {
    let dura = config.duration();
    let time = Local::now();
    let secs = time.num_seconds_from_midnight();
    let part = secs as i64 / dura;

    // Make sure the directory exists
    std::fs::create_dir_all(&config.directory).unwrap();

    // Create the file name
    let mut path = config.prefix.as_deref()
      .map(|prefix| format!("{prefix} "))
      .unwrap_or_default();

    path += &time.format("%Y-%m-%d").to_string();

    if config.duration_hint != 0 {
      let secs = part * dura;
      let h = secs / 3600;
      let m = (secs % 3600) / 60;
      let s = secs % 60;

      path += &format!("_{h:02}-{m:02}-{s:02}");
    }

    path += ".log";

    // Open the file
    let file = std::fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(format!("{}/{}", config.directory, path))
      .unwrap();

    Self(Arc::new((Mutex::new(Inner {
      size: file.metadata().unwrap().len(),
      secs: time.timestamp(),
      part,
      file,
      dura,
    }), config)))
  }

  pub(crate) fn try_rotate(&self) {
    let config = &self.0.1;

    if config.size_limit == 0
    && config.duration_hint == 0
    {
      return;
    }

    let mut this = self.0.0.lock().unwrap();
    let mut need = this.size > config.size_limit;

    let time = Local::now();
    let secs = time.timestamp();

    if secs != this.secs {
      let part = time.num_seconds_from_midnight() as i64 / this.dura;
      need |= part != this.part;

      this.secs = secs;
      this.part = part;
    }

    if !need { return; }

    let mut base_path = config.prefix.as_deref()
      .map(|prefix| format!("{prefix} "))
      .unwrap_or_default();

    base_path += &time.format("%Y-%m-%d").to_string();

    if config.duration_hint != 0 {
      let secs = this.part * this.dura;
      let h = secs / 3600;
      let m = (secs % 3600) / 60;
      let s = secs % 60;

      base_path += &format!("_{h:02}-{m:02}-{s:02}");
    }

    let mut count = 0;

    loop {
      let mut path = base_path.clone();

      if count != 0 {
        path += &format!(" ({})", count);
      }

      path += ".log";

      let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}/{}", config.directory, path))
        .unwrap();

      let size = file.metadata().unwrap().len();

      if size < config.size_limit {
        this.file = file;
        this.size = size;
        break;
      }

      count += 1;
    }
  }

  pub(crate) fn writeln(&self, s: &[u8]) {
    self.try_rotate();

    let mut inner = self.0.0.lock().unwrap();
    inner.file.write_all(s).unwrap();
    inner.file.write_all(b"\n").unwrap();
    inner.size += s.len() as u64 + 1;
  }

  /// Create a new logger with the given name and configuration.
  pub fn logger<'a>(&self, name: impl Into<String>, config: Config) -> Logger<'a> {
    Logger::new_with_file(name, self.clone(), config)
  }

  /// Create a new logger with the given name and default configuration.
  pub fn default_logger<'a>(&self, name: impl Into<String>) -> Logger<'a> {
    Logger::new_with_file(name, self.clone(), Config::default())
  }
}
