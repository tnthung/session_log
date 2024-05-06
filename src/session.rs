use crate::*;


use std::sync::{Arc, Mutex};

use chrono::prelude::*;


type LogContext = Arc<Mutex<Vec<String>>>;


/// Sessions are a way to group log messages together. They are useful for tracking the flow of a
/// program. They can be nested, and when a session is dropped, it will dump all of its messages to
/// the file at once. Despite the file being written to at once, the messages are still written in
/// the traditional order on the terminal. It can also be used as a simple profiling tool, as it
/// will track the starting time of the session and the elapsed time when it is dropped.
///
/// # Example
///
/// ```
/// use session_log::{Logger, Loggable};
///
///
/// fn main() {
///   let logger = Logger::new("main");
///
///   foo(logger.session("foo"), 10);
///   bar(logger.session("bar"), 10);
/// }
///
///
/// fn foo(logger: impl Loggable, n: usize) {
///   for i in 0..n {
///     logger.info(&format!("message-{}", i));
///   }
/// }
///
///
/// fn bar(logger: impl Loggable, n: usize) {
///   for i in 0..n {
///     logger.warning(&format!("message-{}", i));
///   }
/// }
/// ```
pub struct Session {
  died: bool,
  name: String,
  root: String,
  msgs: LogContext,
  sire: Option<LogContext>,
  time: DateTime<Local>,
}


impl Session {
  pub(crate) fn new(name: &str, logger: &str) -> Session {
    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = None;
    let time = Local::now();

    if Logger::new(logger).get_level() >= Level::Info {
      println!("{} {:#} {logger}:{name} - Session started",
        time.to_rfc3339_opts(SecondsFormat::Micros, true),
        Level::Info);
    }

    Session {
      died: false,
      name: name.to_string(),
      root: logger.to_string(),
      time,
      msgs,
      sire,
    }
  }

  /// Create a nested session under the current session.
  pub fn session(&self, name: &str) -> Result<Session, SessionErrorKind> {
    if self.died { return Err(SessionErrorKind::SessionDied); }

    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = Some(self.msgs.clone());
    let time = Local::now();

    if Logger::new(&self.root).get_level() >= Level::Info {
      println!("{} {:#} {}:{name} - Session started",
        time.to_rfc3339_opts(SecondsFormat::Micros, true),
        Level::Info,
        self.root);
    }

    Ok(Session {
      died: false,
      name: name.to_string(),
      root: self.root.clone(),
      time,
      msgs,
      sire,
    })
  }

  pub(self) fn dump(&mut self) {
    if self.died { return; }
    self.died = true;

    let mut rslt = Vec::new();
    let     time = Local::now();
    let     msgs = self.msgs.lock().unwrap();

    rslt.reserve(7 + msgs.len());

    println!("{} {:#} {}:{} - Session ended",
      time.to_rfc3339_opts(SecondsFormat::Micros, true),
      Level::Info,
      self.root,
      self.name);

    rslt.push(format!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    rslt.push(format!("┃ Session: {}", self.name));
    rslt.push(format!("┃ Elapsed: {}us", (time - self.time).num_microseconds().unwrap()));
    rslt.push(format!("┃"));
    rslt.push(format!("┃ {} Start", self.time.to_rfc3339_opts(SecondsFormat::Micros, true)));

    for msg in msgs.iter() {
      for line in msg.lines() {
        let is_border  = line.starts_with("┏") || line.starts_with("┗");
        let is_content = line.starts_with("┃");

        let space = if is_border || is_content { "" } else { " " };
        let line  = if is_border { &line[..line.len()-3] } else { line };

        rslt.push(format!("┃{space}{line}"));
      }
    }

    rslt.push(format!("┃ {} End", time.to_rfc3339_opts(SecondsFormat::Micros, true)));
    rslt.push(format!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    if let Some(sire) = &self.sire {
      sire.lock().unwrap().append(&mut rslt);
      return;
    }

    Logger::new(&self.root).write_line(&rslt.join("\n")).unwrap();
  }
}


impl Loggable for Session {
  fn log(&self, level: Level, message: &str) {
    if self.died { return; }

    let logger = Logger::new(&self.root);
    if level < logger.get_level() { return; }

    let time = Local::now().to_rfc3339_opts(SecondsFormat::Micros, true);
    let root = &self.root;
    let name = &self.name;

    println!("{time} {level:#} {root}:{name} - {message}");
    let message = format!("{time} {level} {root}:{name} - {message}");
    self.msgs.lock().unwrap().push(message);
  }
}


impl Drop for Session {
  fn drop(&mut self) {
    self.dump();
  }
}

