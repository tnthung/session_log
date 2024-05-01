use crate::*;


pub trait Loggable {
  /// Log a message with the dynamic level.
  fn log(&self, level: Level, message: &str);

  /// Log a message at the debug level.
  fn debug(&self, message: &str) {
    self.log(Level::Debug, message);
  }

  /// Log a message at the verbose level.
  fn verbose(&self, message: &str) {
    self.log(Level::Verbose, message);
  }

  /// Log a message at the info level.
  fn info(&self, message: &str) {
    self.log(Level::Info, message);
  }

  /// Log a message at the warning level.
  fn warning(&self, message: &str) {
    self.log(Level::Warning, message);
  }

  /// Log a message at the critical level.
  fn critical(&self, message: &str) {
    self.log(Level::Critical, message);
  }

  /// Log a message at the error level.
  fn error(&self, message: &str) {
    self.log(Level::Error, message);
  }

  /// Log a message at the fatal level then panic.
  ///
  /// **THIS WILL CAUSE THE PROGRAM TO PANIC**\
  /// **ONLY USE THIS FOR UNRECOVERABLE ERRORS**
  fn fatal(&self, message: &str) {
    self.log(Level::Fatal, message);
    panic!("{}", message);
  }
}
