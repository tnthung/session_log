use crate::*;
use chrono::prelude::*;


pub trait Loggable {
  /// Log a message with the dynamic level.
  fn log(&self, ctx: crate::Context);

  /// Create a new session with the given name.
  fn session(&self, name: impl Into<String>) -> Session;

  /// Get the name of the current loggable.
  fn get_name(&self) -> &str;

  /// Get the current logging entry name.
  fn get_logger_name(&self) -> &str;

  /// Get the logger of the current loggable.
  fn get_logger(&self) -> Logger;

  /// Get the current session name.
  fn get_session(&self) -> Option<&str>;

  /// Get the log level of current loggable.
  fn get_log_level(&self) -> Level {
    self.get_logger().get_log_level()
  }

  /// Get the write level of current loggable.
  fn get_write_level(&self) -> Level {
    self.get_logger().get_write_level()
  }

  /// Log a message at the debug level with caller position.
  #[track_caller]
  fn debug(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Debug,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the verbose level with caller position.
  #[track_caller]
  fn verbose(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Verbose,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the info level with caller position.
  #[track_caller]
  fn info(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Info,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the warning level with caller position.
  #[track_caller]
  fn warning(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Warning,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the critical level with caller position.
  #[track_caller]
  fn critical(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Critical,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the error level with caller position.
  #[track_caller]
  fn error(&self, message: impl Into<String>) {
    let loc = std::panic::Location::caller();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Error,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message.into(),
    });
  }

  /// Log a message at the fatal level with caller position then panic.
  ///
  /// **THIS WILL CAUSE THE PROGRAM TO PANIC**\
  /// **ONLY USE THIS FOR UNRECOVERABLE ERRORS**
  #[track_caller]
  fn fatal(&self, message: impl Into<String>) -> ! {
    let loc = std::panic::Location::caller();
    let message = message.into();

    self.log(Context::Log {
      time   : Local::now(),
      level  : Level::Fatal,
      file   : loc.file(),
      line   : loc.line(),
      logger : self.get_logger_name(),
      session: self.get_session(),
      message: &message,
    });

    panic!("{message}");
  }
}
