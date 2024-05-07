use crate::*;


pub trait Loggable {
  /// Log a message with the dynamic level.
  fn log(&self, level: Level, message: &str);

  /// Log a message at the debug level with caller position.
  #[track_caller]
  fn debug(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Debug, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the verbose level with caller position.
  #[track_caller]
  fn verbose(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Verbose, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the info level with caller position.
  #[track_caller]
  fn info(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Info, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the warning level with caller position.
  #[track_caller]
  fn warning(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Warning, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the critical level with caller position.
  #[track_caller]
  fn critical(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Critical, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the error level with caller position.
  #[track_caller]
  fn error(&self, message: &str) {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Error, &format!("{file}:{line} - {message}"));
  }

  /// Log a message at the fatal level with caller position then panic.
  ///
  /// **THIS WILL CAUSE THE PROGRAM TO PANIC**\
  /// **ONLY USE THIS FOR UNRECOVERABLE ERRORS**
  #[track_caller]
  fn fatal(&self, message: &str) -> ! {
    let loc  = std::panic::Location::caller();
    let file = loc.file();
    let line = loc.line();

    self.log(Level::Fatal, &format!("{file}:{line} - {message}"));
    panic!("{}", message);
  }
}
