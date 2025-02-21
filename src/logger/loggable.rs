use crate::bundle::*;
use crate::components::*;


pub trait Loggable<B: Bundle> {
  /// Log with the specified level.
  #[track_caller]
  fn log(&self, level: Level, message: impl ToBundle<B=B>);

  /// Log with the verbose level.
  #[track_caller]
  fn verbose(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Verbose, message);
  }

  /// Log with the debug level.
  #[track_caller]
  fn debug(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Debug, message);
  }

  /// Log with the info level.
  #[track_caller]
  fn info(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Info, message);
  }

  /// Log with the warning level.
  #[track_caller]
  fn warning(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Warning, message);
  }

  /// Log with the critical level.
  #[track_caller]
  fn critical(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Critical, message);
  }

  /// Log with the error level.
  #[track_caller]
  fn error(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Error, message);
  }

  /// Log with the fatal level.
  ///
  /// `Caution`: This function will **EXIT** the process.
  #[track_caller]
  fn fatal(&self, message: impl ToBundle<B=B>) -> ! {
    self.log(Level::Fatal, message);
    std::process::exit(1);
  }
}
