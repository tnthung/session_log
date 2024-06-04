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
/// ```no_run
/// use session_log::{Logger, Loggable};
///
/// fn main() {
///   let logger = Logger::new("main");
///
///   foo(logger.session("foo"), 10);
///   bar(logger.session("bar"), 10);
/// }
///
/// fn foo(logger: impl Loggable, n: usize) {
///   for i in 0..n {
///     logger.info(&format!("message-{}", i));
///   }
/// }
///
/// fn bar(logger: impl Loggable, n: usize) {
///   for i in 0..n {
///     logger.warning(&format!("message-{}", i));
///   }
/// }
/// ```
pub struct Session {
  died: bool,
  pass: bool,
  name: String,
  root: String,
  msgs: LogContext,
  sire: Option<LogContext>,
  time: DateTime<Local>,
  file: &'static str,
  line: u32,
}


impl Session {
  pub(crate) fn new(name: impl Into<String>, logger: &str, file: &'static str, line: u32) -> Session {
    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = None;
    let time = Local::now();
    let name = name.into();

    let ses = Session {
      died: false,
      pass: false,
      root: logger.to_string(),
      name: name.clone(),
      time,
      msgs,
      sire,
      file,
      line,
    };

    ses.log(Context::SessionStart {
      time,
      file,
      line,
      logger,
      session: &name,
    });

    ses
  }

  /// Re-enable the session so it can log messages again.
  pub fn enable(&mut self) {
    self.pass = false;
  }

  /// Temporarily disable the session so it will not log messages.
  pub fn disable(&mut self) {
    self.pass = true;
  }

  pub(self) fn dump(&mut self) {
    if self.died { return; }
    self.died = true;

    let mut rslt = Vec::new();
    let     time = Local::now();
    let     msgs = self.msgs.lock().unwrap();

    rslt.reserve(7 + msgs.len());

    rslt.push(format!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));
    rslt.push(format!("┃ Session: {}", self.name));
    rslt.push(format!("┃ Elapsed: {}us", time.signed_duration_since(self.time).num_milliseconds()));
    rslt.push(format!("┃"));

    for msg in msgs.iter() {
      for line in msg.lines() {
        let is_border = line.starts_with("┏") || line.starts_with("┗");
        let is_nested = line.starts_with("┃") || is_border;

        let space = if is_nested { ""                    } else { " "  };
        let line  = if is_border { &line[..line.len()-3] } else { line };

        rslt.push(format!("┃{space}{line}"));
      }
    }

    rslt.push(format!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"));

    if let Some(sire) = &self.sire {
      sire.lock().unwrap().append(&mut rslt);
      return;
    }

    Logger::new(&self.root).write_line(&rslt.join("\n"));
  }
}


impl Loggable for Session {
  fn log(&self, ctx: crate::Context) {
    if self.died { return; }
    if self.pass { return; }

    let logger = Logger::new(&self.root);
    if let Some(level) = ctx.get_level() {
      if level < &logger.get_level() { return; }
    }

    let message = (logger.get_processor())(&ctx);
    self.msgs.lock().unwrap().push(message);
  }

  #[track_caller]
  fn session(&self, name: impl Into<String>) -> Session {
    if self.died { unreachable!("This should not be happened") }

    let msgs = Arc::new(Mutex::new(Vec::new()));
    let sire = Some(self.msgs.clone());
    let time = Local::now();

    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    let ses = Session {
      died: false,
      pass: false,
      name: name.into(),
      root: self.root.clone(),
      time,
      msgs,
      sire,
      file,
      line,
    };

    ses.log(Context::SessionStart {
      time,
      file,
      line,
      logger : &self.root,
      session: &self.name,
    });

    ses
  }

  fn get_logger(&self) -> &str {
    &self.root
  }

  fn get_session(&self) -> Option<&str> {
    Some(&self.name)
  }
}


impl Drop for Session {
  fn drop(&mut self) {
    self.log(Context::SessionEnd {
      time   : Local::now(),
      file   :  self.file,
      line   :  self.line,
      logger : &self.root,
      session: &self.name,
    });

    self.dump();
  }
}
