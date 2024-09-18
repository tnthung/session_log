use std::sync::{Arc, Mutex};

use crate::*;


pub(crate) enum Ctx {
  Raw(String),
  Context(Context),
}


type Ctxs = Arc<Mutex<Vec<Ctx>>>;


/// A session is a temporary logger that will print immediately but only write when dropped. All the
/// logs during the session will be grouped together and write once. It's useful when you want to
/// log a series of messages such as one session of a request. The session is panic-safe, which means
/// it will write all the logs when panic.
///
/// Apart from the formatter, the session can be `Silent`. If it's silent, then the header & footer
/// will not be printed or written if the session never logged anything. Due to the uncertainty of if
/// the session will log anything, the header will be deferred until the first log. If the session
/// logged anything, then it acts like a normal session.
pub struct Session<'a> {
  name  : String,
  source: Source,
  logger: &'a Logger,
  ctxs  : Ctxs,
  parent: Option<&'a Self>,
  silent: Mutex<bool>,
}


impl<'a> Session<'a> {
  #[track_caller]
  pub(crate) fn new(name: &str, source: Source, logger: &'a Logger, parent: Option<&'a Self>, silent: bool) -> Self {
    let mut ctxs   = Vec::new();
    let     source = source.session(name);

    let header = Context::new_header(source.clone());

    if !silent {
      println!("{}", (logger.for_print)(&header));
    }

    ctxs.push(Ctx::Context(header));

    Self {
      name  : name.to_string(),
      ctxs  : Arc::new(Mutex::new(ctxs)),
      silent: Mutex::new(silent),
      logger,
      parent,
      source,
    }
  }

  /// Create a new session from the session.
  ///
  /// This is useful when you want to create a sub-session from a session. The sub-session will later
  /// be nested under the parent session when written.
  #[track_caller]
  pub fn session(&'a self, name: &str, silent: bool) -> Self {
    Self::new(name, self.source.clone(), self.logger, Some(self), silent)
  }
}


impl Drop for Session<'_> {
  fn drop(&mut self) {
    if *self.silent.lock().unwrap() { return; }

    let mut ctxs = self.ctxs.lock().unwrap();

    let Ctx::Context(header) = ctxs.first().unwrap()
      else { unreachable!() };

    let footer = Context::new_footer(
      self.source.clone(),
      &header.start().unwrap(),
      *header.location());

    println!("{}", (self.logger.for_print)(&footer));

    let mut lines = Vec::new();

    lines.push("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());

    if self.parent.is_none() {
      lines.push(format!("┃ Logger : {}", self.logger.name()));
    }

    lines.push(format!("┃ Session: {}", self.name));
    lines.push(format!("┃ Elapsed: {}", footer.elapsed().unwrap().as_micros()));
    lines.push("┃".to_string());

    ctxs.push(Ctx::Context(footer));

    for ctx in ctxs.drain(..) {
      match ctx {
        Ctx::Raw(string) => {
          let tmp = string.trim_start_matches("┃");

          if tmp.starts_with("┏━")
          || tmp.starts_with("┗━")
          {
            lines.push(format!("┃{}", &string[..string.len()-3]));
            continue;
          }

          lines.push(format!("┃{string}"));
        }

        Ctx::Context(ctx) => {
          for line in (self.logger.for_print)(&ctx).split('\n') {
            lines.push(format!("┃ {line}"));
          }
        }
      }
    }

    lines.push("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());

    match &self.parent {
      Some(parent) => {
        let mut parent = parent.ctxs.lock().unwrap();

        for line in lines {
          parent.push(Ctx::Raw(line));
        }
      }

      None => {
        self.logger.write(&lines.join("\n"));
      }
    }
  }
}


impl LoggableInner for Session<'_> {
  fn log(&self, level: Level, message: &str) {
    let mut ctxs   = self.ctxs  .lock().unwrap();
    let mut silent = self.silent.lock().unwrap();

    let ctx = Context::new_message(
      self.source.clone(), level, message);

    if level >= self.logger.print_level() {
      if *silent {
        let Ctx::Context(header) = ctxs.first().unwrap()
          else { unreachable!() };

        println!("{}", (self.logger.for_print)(&header));
        *silent = false;
      }

      println!("{}", (self.logger.for_print)(&ctx));
    }

    if level >= self.logger.write_level() {
      ctxs.push(Ctx::Context(ctx));
    }
  }
}


impl Loggable for Session<'_> {
  fn root_name(&self) -> &str {
    self.logger.root_name()
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn path(&self) -> String {
    self.logger.path()
  }

  fn write_level(&self) -> Level {
    self.logger.write_level()
  }

  fn print_level(&self) -> Level {
    self.logger.print_level()
  }
}
