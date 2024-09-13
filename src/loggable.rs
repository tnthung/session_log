use crate::*;


/// Loggable trait is used to define the common behavior for the type that can be used as a logger.
pub trait Loggable {
  /// Log a message with the given level.
  #[track_caller]
  fn log(&self, level: Level, message: &str);

  /// Get the name of the root logger.
  fn root_name(&self) -> &str;

  /// Get the name of current logger.
  fn name(&self) -> &str;

  /// Get the writing level of the root logger.
  fn write_level(&self) -> Level;

  /// Get the printing level of the root logger.
  fn print_level(&self) -> Level;


  /// Log a message with debug level.
  #[track_caller]
  fn debug(&self, message: &str) {
    self.log(Level::Debug, message);
  }

  /// Log a message with verbose level.
  #[track_caller]
  fn verbose(&self, message: &str) {
    self.log(Level::Verbose, message);
  }

  /// Log a message with info level.
  #[track_caller]
  fn info(&self, message: &str) {
    self.log(Level::Info, message);
  }

  /// Log a message with warning level.
  #[track_caller]
  fn warning(&self, message: &str) {
    self.log(Level::Warning, message);
  }

  /// Log a message with critical level.
  #[track_caller]
  fn critical(&self, message: &str) {
    self.log(Level::Critical, message);
  }

  /// Log a message with error level.
  #[track_caller]
  fn error(&self, message: &str) {
    self.log(Level::Error, message);
  }

  /// Log a message with fatal level. The program will panic after logging the message.
  #[track_caller]
  fn fatal(&self, message: &str) -> ! {
    self.log(Level::Fatal, message);
    panic!("Fatal error occurred.");
  }
}
