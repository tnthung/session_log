use crate::*;

use std::io::Write;
use std::fs::{File, OpenOptions, create_dir_all};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use chrono::prelude::*;
use once_cell::sync::Lazy;


#[derive(Debug, Clone)]
pub struct Logger(String);


#[derive(Debug)]
struct Inner {
  lvl: Level,
  hr : u32,
  dir: String,
}


static LOGGERS: Lazy<Mutex<HashMap<String, Inner>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));

static FILES: Lazy<Mutex<HashMap<String, Arc<Mutex<File>>>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));


fn get_time_tuple() -> (u32, u32, u32, u32) {
  let now = Local::now();

  (
    now.year () as u32,
    now.month(),
    now.day  (),
    now.hour ()
  )
}


fn get_file_name(y: u32, m: u32, d: u32) -> String {
  format!("{y:04}-{m:02}-{d:02}.log")
}


impl Logger {
  /// Create a new logging entry with the given name & options.\
  /// **OR**\
  /// Retrieve an existing logging entry if the entry already exists and have same options.
  ///
  /// # Parameters
  ///
  /// - `name` - The name of the logging entry.
  /// - `level` - The level of the logging entry.
  /// - `directory` - The directory where the log files will be stored. (Relative to the current working directory)
  ///
  /// # Errors
  ///
  /// - `ErrorKind::DifferentLevel` if the existing logger has different level than the given options.
  /// - `ErrorKind::DifferentDirectory` if the existing logger has different directory than the given options.
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let logger = Logger::with_options(
  ///     "main", Level::Debug, "logs/main");
  ///
  ///   let logger2 = Logger::with_options(
  ///     "main", Level::Debug, "logs/main");  // Retrieve the existing logger
  ///
  ///   let logger3 = Logger::with_options(
  ///     "main", Level::Info , "logs/main");  // Error: Different level
  ///
  ///   let logger4 = Logger::with_options(
  ///     "main", Level::Debug, "logs/other"); // Error: Different directory
  /// }
  /// ```
  pub fn with_options(
    name: impl Into<String>,
    level: Level,
    directory: impl Into<String>
  ) -> Result<Logger, ErrorKind>
  {
    let mut loggers = LOGGERS.lock().unwrap();

    let name: String = name.into();

    let lvl: Level  = level;
    let dir: String = directory.into();

    let inner = loggers.get(&name);

    if let None = inner {
      create_dir_all(&dir).map_err(|_|
        ErrorKind::FailedToCreateFolder)?;

      loggers.insert(name.clone(), Inner {
        lvl, dir, hr: Local::now().hour() });
    }

    else if let Some(inner) = inner {
      if inner.lvl != level {
        return Err(ErrorKind::DifferentLevel);
      }

      if inner.dir != dir {
        return Err(ErrorKind::DifferentDirectory);
      }
    }

    Ok(Logger(name))
  }

  /// Create a new logging entry with the given name & options.\
  /// **OR**\
  /// Retrieve an existing logging entry if the entry already exists.
  ///
  /// Unlike `Logger::with_options`, this method doesn't return an error if the existing logger has different options.
  /// Instead, it will just ignore the given options and return the existing logger.
  ///
  /// # Parameters
  /// - `name` - The name of the logging entry.
  /// - `level` - The level of the logging entry.
  /// - `directory` - The directory where the log files will be stored. (Relative to the current working directory)
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let logger = Logger::try_with_option(
  ///     "main", Level::Debug, "logs/main");
  ///
  ///   let logger2 = Logger::try_with_option(
  ///     "main", Level::Debug, "logs/main");  // Retrieve the existing logger
  ///
  ///   let logger3 = Logger::try_with_option(
  ///     "main", Level::Info , "logs/main");  // Retrieve the existing logger
  ///
  ///   let logger4 = Logger::try_with_option(
  ///     "main", Level::Debug, "logs/other"); // Retrieve the existing logger
  /// }
  pub fn try_with_option(
    name: impl Into<String>,
    level: Level,
    directory: impl Into<String>
  ) -> Logger
  {
    let mut loggers = LOGGERS.lock().unwrap();

    let name: String = name.into();

    let lvl: Level  = level;
    let dir: String = directory.into();

    let inner = loggers.get(&name);

    if let None = inner {
      create_dir_all(&dir).unwrap();

      loggers.insert(name.clone(), Inner {
        lvl, dir, hr: Local::now().hour() });
    }

    Logger(name)
  }

  /// Create a new logging entry with the given name. Level and directory is
  /// defaulted to Info and "logs"\
  /// **OR**\
  /// Retrieve an existing logging entry if the entry already exists.
  ///
  /// Unlike `Logger::with_options`, this method doesn't return an error if the existing logger has different options.
  /// Instead, it will just ignore the given options and return the existing logger.
  ///
  /// # Parameters
  /// - `name` - The name of the logging entry.
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let logger  = Logger::new("main");
  ///   let logger2 = Logger::new("main"); // Retrieve the existing logger
  /// }
  pub fn new(name: impl Into<String>) -> Logger {
    Self::try_with_option(name, Level::Info, "logs")
  }

  /// Get logging level for this entry.
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let logger = Logger::new("main");
  ///
  ///   assert_eq!(logger.get_level(), Level::Info);
  /// }
  /// ```
  pub fn get_level(&self) -> Level {
    let loggers = LOGGERS.lock().unwrap();

    let Some(inner) = loggers.get(&self.0)
      else { unreachable!() };

    return inner.lvl;
  }

  /// Set logging level for this entry
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let mut logger = Logger::new("main");
  ///
  ///   logger.set_level(Level::Debug);
  ///   assert_eq!(logger.get_level(), Level::Debug);
  ///
  ///   logger.set_level(Level::Info);
  ///   assert_eq!(logger.get_level(), Level::Info);
  /// }
  /// ```
  pub fn set_level(&mut self, level: Level) {
    let mut loggers = LOGGERS.lock().unwrap();

    let Some(inner) = loggers.get_mut(&self.0)
      else { unreachable!() };

    inner.lvl = level;
  }

  /// Get logging directory for this entry.
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let logger = Logger::new("main");
  ///
  ///   assert_eq!(logger.get_directory(), "logs");
  /// }
  /// ```
  pub fn get_directory(&self) -> String {
    let loggers = LOGGERS.lock().unwrap();

    let Some(inner) = loggers.get(&self.0)
      else { unreachable!() };

    return inner.dir.clone();
  }

  /// Set logging directory for this entry.
  ///
  /// # Examples
  ///
  /// ```
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let mut logger = Logger::new("main");
  ///
  ///   logger.set_directory("logs/main");
  ///   assert_eq!(logger.get_directory(), "logs/main");
  ///
  ///   logger.set_directory("logs/other");
  ///   assert_eq!(logger.get_directory(), "logs/other");
  /// }
  /// ```
  pub fn set_directory(&mut self, directory: impl Into<String>) {
    let mut loggers = LOGGERS.lock().unwrap();

    let Some(inner) = loggers.get_mut(&self.0)
      else { unreachable!() };

    inner.dir = directory.into();
  }

  /// Create a new session with the given name under this logging entry.
  pub fn session(&self, name: &str) -> Session {
    Session::new(name, &self.0)
  }

  fn get_file(&self) -> Arc<Mutex<File>> {
    let mut loggers = LOGGERS.lock().unwrap();
    let mut files   = FILES  .lock().unwrap();

    let Some(inner) = loggers.get_mut(&self.0)
      else { unreachable!() };

    let hr   = &mut inner.hr;
    let dir  = &inner.dir;
    let now  = get_time_tuple();
    let file = files.get(dir).clone();

    if file.is_none() || now.3 != *hr {
      let file_name = get_file_name(now.0, now.1, now.2);
      let file_path = format!("{}/{}", dir, file_name);

      let file = Arc::new(Mutex::new(OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .unwrap()));

      files.insert(
        dir.clone(),
        file.clone());
      *hr = now.3;

      return file;
    }

    return file.unwrap().clone();
  }

  pub(crate) fn write_line(&self, message: &str) -> std::io::Result<()> {
    let file = self.get_file();
    let mut file = file.lock().unwrap();
    writeln!(file, "{}", message)
  }
}


impl Loggable for Logger {
  fn log(&self, level: Level, message: &str) {
    if level >= self.get_level() {
      let time = Local::now().to_rfc3339_opts(SecondsFormat::Micros, true);
      let name = &self.0;

      println!("{time} {level:#} {name} - {message}");
      self.write_line(&format!("{time} {level} {name} - {message}")).unwrap()
    }
  }
}
