use crate::*;
use std::sync::{Arc, Mutex};


pub(crate) enum Ctx {
  Raw(String),
  Context(Context),
}


type Ctxs = Arc<Mutex<Vec<Ctx>>>;


pub(crate) enum SessionSrc<'a> {
  Logger (&'a Logger),
  Session(&'a Session),
}


/// A session is a temporary logger that will print immediately but only write when dropped. All the
/// logs during the session will be grouped together and write once. It's useful when you want to
/// log a series of messages such as one session of a request. The session is panic-safe, which means
/// it will write all the logs when panic.
///
/// Apart from the formatter, the session can be `Silent`. If it's silent, then the header & footer
/// will not be printed or written if the session never logged anything. Due to the uncertainty of if
/// the session will log anything, the header will be deferred until the first log. If the session
/// logged anything, then it acts like a normal session.
pub struct Session {
  name       : String,
  source     : Source,
  writer     : Writer,
  for_write  : fn(&Context) -> String,
  for_print  : fn(&Context) -> String,
  write_level: Level,
  print_level: Level,
  ctxs       : Ctxs,
  parent     : Option<Ctxs>,
  silent     : Mutex<bool>,
}


impl Session {
  #[track_caller]
  pub(crate) fn new(name: &str, source: SessionSrc<'_>, silent: bool) -> Self {
    let (
      source,
      writer,
      for_write,
      for_print,
      write_level,
      print_level,
      parent,
    ) = match source {
      SessionSrc::Logger(l) => (
        Source::new(&l.name).session(name),
        l.writer.clone(),
        l.for_write,
        l.for_print,
        l.write_level,
        l.print_level,
        None,
      ),

      SessionSrc::Session(s) => (
        s.source.session(name),
        s.writer.clone(),
        s.for_write,
        s.for_print,
        s.write_level,
        s.print_level,
        Some(s.ctxs.clone()),
      ),
    };

    let header = Context::new_header(source.clone());
    if !silent {
      println!("{}", for_print(&header));
    }

    Self {
      name: name.to_string(),
      ctxs: Arc::new(Mutex::new(
        vec![Ctx::Context(header)])),
      silent: Mutex::new(silent),
      parent,
      source,
      writer,
      for_write,
      for_print,
      write_level,
      print_level,
    }
  }

  /// Create a new session from the session.
  ///
  /// This is useful when you want to create a sub-session from a session. The sub-session will later
  /// be nested under the parent session when written.
  #[track_caller]
  pub fn session(&self, name: &str, silent: bool) -> Self {
    Self::new(name, SessionSrc::Session(self), silent)
  }

  /// Create a new session from the session and execute the callable with the session passed in.
  ///
  /// This can be handy for isolating the environment of each callable while also providing a common
  /// logging interface for any callable that needs to be logged.
  #[track_caller]
  pub fn session_then<F, T>(&self, name: &str, silent: bool, callable: F) -> T
  where F: FnOnce(Self) -> T
  {
    callable(self.session(name, silent))
  }
}


impl Drop for Session {
  fn drop(&mut self) {
    if *self.silent.lock().unwrap() { return; }

    let mut ctxs = self.ctxs.lock().unwrap();

    let Ctx::Context(header) = ctxs.first().unwrap()
      else { unreachable!() };

    let footer = Context::new_footer(
      self.source.clone(),
      &header.start().unwrap(),
      *header.location());

    println!("{}", (self.for_print)(&footer));

    let mut lines = Vec::new();

    lines.push("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());

    if self.parent.is_none() {
      lines.push(format!("┃ Logger : {}", self.source.logger()));
    }

    lines.push(format!("┃ Session: {}", self.name));
    lines.push(format!("┃ Elapsed: {:?}", footer.elapsed().unwrap()));
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
          for line in (self.for_write)(&ctx).split('\n') {
            lines.push(format!("┃ {line}"));
          }
        }
      }
    }

    lines.push("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".to_string());

    match &self.parent {
      Some(parent) => {
        let mut parent = parent.lock().unwrap();

        for line in lines {
          parent.push(Ctx::Raw(line));
        }
      }

      None => {
        self.writer.lock().unwrap()
          .write(lines.join("\n"));
      }
    }
  }
}


impl LoggableInner for Session {
  fn log(&self, level: Level, message: &str) {
    let mut ctxs = self.ctxs.lock().unwrap();

    let ctx = Context::new_message(
      self.source.clone(), level, message);

    if level >= self.print_level {
      let mut silent = self.silent.lock().unwrap();

      if *silent {
        let Ctx::Context(header) = ctxs.first().unwrap()
          else { unreachable!() };

        println!("{}", (self.for_print)(&header));
        *silent = false;
      }

      println!("{}", (self.for_print)(&ctx));
    }

    if level >= self.write_level {
      ctxs.push(Ctx::Context(ctx));
    }
  }
}


impl Loggable for Session {
  fn root_name(&self) -> &str {
    self.source.logger().as_ref()
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn path(&self) -> String {
    self.writer.lock().unwrap().path.clone()
  }

  fn write_level(&self) -> Level {
    self.write_level
  }

  fn print_level(&self) -> Level {
    self.print_level
  }
}
