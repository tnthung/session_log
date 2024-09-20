use crate::*;


/// Logger is the base type that actually handle the loggings
pub struct Logger {
  pub(crate) name       : String,
  pub(crate) write_level: Level,
  pub(crate) print_level: Level,
  pub(crate) writer     : Writer,
  pub(crate) for_write  : fn(&Context) -> String,
  pub(crate) for_print  : fn(&Context) -> String,
}


impl Logger {
  /// Creating a new logger with the given name & default configurations.
  pub fn default(name: impl Into<String>) -> Self {
    Self::new::<DefaultFormatter>(name, Config::default())
  }
}


impl Logger {
  /// Creating a new logger with the given name.
  pub fn new<F: Formatter>(name: impl Into<String>, config: Config) -> Self {
    Logger {
      name       : name.into(),
      write_level: config.write_level,
      print_level: config.print_level,
      writer     : Writer::new(
        &config.directory,
        config.duration_limit,
        config.size_limit,
      ),
      for_write  : F::for_write,
      for_print  : F::for_print,
    }
  }

  /// Create a new session from the logger.
  ///
  /// There are two types of session: silent & non-silent. Silent session will not print the header
  /// and footer of the session when no message is logged before the session is dropped. It's useful
  /// session that may potentially not log anything. Non-silent session will always print the header
  /// and footer of the session.
  ///
  /// Due to the uncertainty of if the session will log anything, the header will be deferred until
  /// the first log. If the session logged anything, it'll act like a non-silent session.
  #[track_caller]
  pub fn session(&self, name: &str, silent: bool) -> Session {
    Session::new(name, SessionSrc::Logger(self), silent)
  }
}


impl LoggableInner for Logger {
  fn log(&self, level: Level, message: &str) {
    let ctx = Context::new_message(
      Source::new(&self.name),
      level,
      message
    );

    if level >= self.print_level {
      println!("{}", (self.for_print)(&ctx));
    }

    if level >= self.write_level {
      self.writer.lock().unwrap()
        .write(&(self.for_write)(&ctx));
    }
  }
}


impl Loggable for Logger {
  fn root_name(&self) -> &str {
    &self.name
  }

  fn name(&self) -> &str {
    &self.name
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
