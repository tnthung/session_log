use crate::*;

use std::io::Write;
use std::fs::{File, OpenOptions, create_dir_all};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[cfg(feature = "async")]
use std::sync::mpsc::{channel, Sender};

use chrono::prelude::*;
use once_cell::sync::Lazy;


#[derive(Debug, Clone)]
pub struct Logger(String);


struct Inner {
  wrt_level: Level,
  log_level: Level,
  hour     : u32,
  dir      : String,
  processor: Arc<ContextProcessor>
}


static LOGGERS: Lazy<Mutex<HashMap<String, Inner>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));

static FILES: Lazy<Mutex<HashMap<String, Arc<Mutex<File>>>>> =
  Lazy::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "async")]
static SENDER: Lazy<Sender<(Arc<Mutex<File>>, String)>> = Lazy::new(|| {
  let (tx, rx) = channel::<(Arc<Mutex<File>>, String)>();

  unsafe {
    THREAD = Some(std::thread::spawn(move || {
      use std::sync::mpsc::TryRecvError;

      loop {
        match rx.try_recv() {
          Ok((file, message)) => {
            let mut file = file.lock().unwrap();
            writeln!(file, "{}", message).unwrap();
          }

          Err(TryRecvError::Empty) => {
            if THREAD.is_none() { break; }
            std::thread::sleep(std::time::Duration::from_micros(1));
          }

          Err(TryRecvError::Disconnected) =>
            break,
        }
      }
    }));
  }

  tx
});

#[cfg(feature = "async")]
static mut THREAD: Option<std::thread::JoinHandle<()>> = None;

static mut DEFAULT_PATH     : Lazy<String>     = Lazy::new(|| "./logs".to_string());
static mut DEFAULT_PROC     : ContextProcessor = crate::context::processor;
static mut DEFAULT_WRT_LEVEL: Level            = Level::Verbose;
static mut DEFAULT_LOG_LEVEL: Level            = Level::Info;


fn get_time_tuple() -> (u32, u32, u32, u32) {
  let now = Local::now();

  (
    now.year () as u32,
    now.month(),
    now.day  (),
    now.hour ()
  )
}


fn get_file_name(y: u32, m: u32, d: u32, h: u32) -> String {
  format!("{y:04}-{m:02}-{d:02}-{h:02}.log")
}


impl Logger {
  /// Create a new logging entry with the given name. Level and directory is
  /// defaulted to Info and "logs"\
  /// **OR**\
  /// Retrieve an existing logging entry if the entry already exists.
  ///
  /// You can chain the configuration methods to set the level and directory.
  ///
  /// # Parameters
  /// - `name` - The name of the logging entry.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let logger  = Logger::new("main");
  ///   let logger2 = Logger::new("main"); // Retrieve the existing logger
  ///
  ///   let logger3 = Logger::new("main")
  ///     .set_log_level(session_log::Level::Debug);
  ///     .set_directory("logs/main");
  /// }
  pub fn new(name: impl Into<String>) -> Logger {
    let mut loggers = LOGGERS.lock().unwrap();

    #[cfg(feature = "async")]
    let _ = *SENDER;

    let name: String = name.into();

    let inner = loggers.get(&name);

    if let None = inner {
      loggers.insert(name.clone(), Inner {
        wrt_level: unsafe { DEFAULT_WRT_LEVEL },
        log_level: unsafe { DEFAULT_LOG_LEVEL },
        dir      : unsafe { DEFAULT_PATH.clone() },
        hour     : Local::now().hour(),
        processor: Arc::new(unsafe { DEFAULT_PROC }),
      });
    }

    Logger(name)
  }

  /// Set the default logging directory for all new logging entries.
  /// The directory will be created if it doesn't exist.
  /// Old loggers will not be affected by this change.
  ///
  /// The default directory is `./logs`.
  pub fn set_default_directory(directory: impl Into<String>) {
    unsafe { *DEFAULT_PATH = directory.into(); };
  }

  /// Get the default logging directory for all new logging entries.
  pub fn get_default_directory() -> String {
    unsafe { DEFAULT_PATH.clone() }
  }

  /// Set the default writing level for all new writing entries.
  /// Old loggers will not be affected by this change.
  ///
  /// The default writing level is `Verbose`.
  pub fn set_default_write_level(level: Level) {
    unsafe { DEFAULT_WRT_LEVEL = level; };
  }

  /// Get the default writing level for all new writing entries.
  pub fn get_default_write_level() -> Level {
    unsafe { DEFAULT_WRT_LEVEL }
  }

  /// Set the default logging level for all new logging entries.
  /// Old loggers will not be affected by this change.
  ///
  /// The default logging level is `Info`.
  pub fn set_default_log_level(level: Level) {
    unsafe { DEFAULT_LOG_LEVEL = level; };
  }

  /// Get the default logging level for all new logging entries.
  pub fn get_default_log_level() -> Level {
    unsafe { DEFAULT_LOG_LEVEL }
  }

  /// Set the default processor for all new logging entries.
  /// Old loggers will not be affected by this change.
  ///
  /// The default processor is `$crate::context::processor`.
  pub fn set_default_processor(proc: fn(&Context) -> (String, String)) {
    unsafe { DEFAULT_PROC = proc; };
  }

  /// Get the default processor for all new logging entries.
  pub fn get_default_processor() -> fn(&Context) -> (String, String) {
    unsafe { DEFAULT_PROC }
  }

  #[cfg(feature = "async")]
  /// This method will join the async thread and wait for it to finish all writing operations.\
  /// It's crucial to call this method before the program exits to ensure no logs are lost.
  ///
  /// This is only available when the `async` feature is enabled.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let logger = Logger::new("main");
  ///
  ///   // Do some logging
  ///   for i in 0..10000 {
  ///     logger.info(format!("Info {i}"));
  ///   }
  ///
  ///   // Flush the logs
  ///   Logger::flush();
  /// }
  /// ```
  pub fn flush() {
    if let Some(thread) = unsafe { THREAD.take() } {
      thread.join().unwrap();
    }
  }

  /// Get writing level for this entry.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let logger = Logger::new("main");
  ///
  ///   assert_eq!(logger.get_write_level(), Level::Info);
  /// }
  /// ```
  pub fn get_write_level(&self) -> Level {
    let loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get(&self.0).unwrap();

    inner.wrt_level
  }

  /// Set writing level for this entry
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let mut logger = Logger::new("main");
  ///
  ///   logger = logger.set_write_level(Level::Debug);
  ///   assert_eq!(logger.get_write_level(), Level::Debug);
  ///
  ///   logger = logger.set_write_level(Level::Info);
  ///   assert_eq!(logger.get_write_level(), Level::Info);
  /// }
  /// ```
  pub fn set_write_level(self, level: Level) -> Self {
    let mut loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get_mut(&self.0).unwrap();

    inner.wrt_level = level;

    self
  }

  /// Get logging level for this entry.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let logger = Logger::new("main");
  ///
  ///   assert_eq!(logger.get_log_level(), Level::Info);
  /// }
  /// ```
  pub fn get_log_level(&self) -> Level {
    let loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get(&self.0).unwrap();

    inner.log_level
  }

  /// Set logging level for this entry
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::{Logger, Level};
  ///
  /// fn main() {
  ///   let mut logger = Logger::new("main");
  ///
  ///   logger = logger.set_log_level(Level::Debug);
  ///   assert_eq!(logger.get_log_level(), Level::Debug);
  ///
  ///   logger = logger.set_log_level(Level::Info);
  ///   assert_eq!(logger.get_log_level(), Level::Info);
  /// }
  /// ```
  pub fn set_log_level(self, level: Level) -> Self {
    let mut loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get_mut(&self.0).unwrap();

    inner.log_level = level;

    self
  }

  /// Get logging directory for this entry.
  ///
  /// # Examples
  ///
  /// ```no_run
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
    let inner = loggers.get(&self.0).unwrap();

    inner.dir.clone()
  }

  /// Set logging directory for this entry and create the directory if it doesn't exist. the result
  /// of creating the directory is returned.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let mut logger = Logger::new("main");
  ///
  ///   logger = logger.set_directory("logs/main");
  ///   assert_eq!(logger.get_directory(), "logs/main");
  ///
  ///   logger = logger.set_directory("logs/other");
  ///   assert_eq!(logger.get_directory(), "logs/other");
  /// }
  /// ```
  pub fn set_directory(self, directory: impl Into<String>) -> Self {
    let mut loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get_mut(&self.0).unwrap();

    inner.dir = directory.into();

    self
  }

  pub(crate) fn get_processor(&self) -> Arc<fn(&Context) -> (String, String)> {
    let loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get(&self.0).unwrap();

    inner.processor.clone()
  }

  /// Set the processor for this entry.
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use session_log::Logger;
  ///
  /// fn main() {
  ///   let logger = Logger::new("main")
  ///     .set_proc(|ctx| (
  ///       // The console will always print "Hello"
  ///       "Hello".to_string(),
  ///       // The file will always write "World"
  ///       "World".to_string()
  ///     ));
  /// }
  /// ```
  pub fn set_processor(self, proc: fn(&Context) -> (String, String)) -> Self {
    let mut loggers = LOGGERS.lock().unwrap();
    let inner = loggers.get_mut(&self.0).unwrap();

    inner.processor = Arc::new(proc);

    self
  }

  fn get_file(&self) -> Arc<Mutex<File>> {
    let mut loggers = LOGGERS.lock().unwrap();
    let mut files   = FILES  .lock().unwrap();

    let inner = loggers.get_mut(&self.0).unwrap();

    let hr   = &mut inner.hour;
    let dir  = &inner.dir;
    let now  = get_time_tuple();
    let file = files.get(dir).clone();

    if file.is_none() || now.3 != *hr {
      create_dir_all(dir).unwrap();

      let name = get_file_name(now.0, now.1, now.2, now.3);
      let path = format!("{dir}/{name}");
      let file = Arc::new(Mutex::new(OpenOptions::new()
        .create(true).append(true).open(&path).unwrap()));

      files.insert(
        dir .clone(),
        file.clone());
      *hr = now.3;

      return file;
    }

    return file.unwrap().clone();
  }

  pub(crate) fn write_line(&self, message: &str) {
    let file = self.get_file();

    #[cfg(not(feature = "async"))] {
      let mut file = file.lock().unwrap();
      writeln!(file, "{message}").unwrap();
    }

    #[cfg(feature = "async")] {
      SENDER.send((file, message.to_string()))
        .expect("Failed to send message to async thread");
    }
  }
}


impl Loggable for Logger {
  fn log(&self, ctx: crate::Context) {
    let loggers = LOGGERS.lock().unwrap();
    let inner   = loggers.get(&self.0).unwrap();

    let (l, f) = (inner.processor)(&ctx);

    let log_level = inner.log_level;
    let wrt_level = inner.wrt_level;

    drop(loggers);

    if log_level <= *ctx.get_level().unwrap() {
      println!("{}", l);
    }

    if wrt_level <= *ctx.get_level().unwrap() {
      self.write_line(&f);
    }
  }

  #[track_caller]
  fn session(&self, name: impl Into<String>) -> Session {
    let loc = std::panic::Location::caller();
    Session::new(name, &self.0, loc.file(), loc.line())
  }

  fn get_name(&self) -> &str {
    &self.0
  }

  fn get_logger_name(&self) -> &str {
    &self.0
  }

  fn get_logger(&self) -> Logger {
    self.clone()
  }

  fn get_session(&self) -> Option<&str> {
    None
  }
}
