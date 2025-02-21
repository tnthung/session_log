use crate::bundle::*;
use crate::components::*;
use crate::file::*;
use crate::config::*;
use std::io::Write;
use std::marker::PhantomData;


#[derive(Debug)]
pub struct Logger<'a, B: Bundle = DefaultBundle<'a>>(Vec<String>, File, Config, PhantomData<&'a B>);


impl<'a, B: Bundle> Logger<'a, B> {
  pub(crate) fn new_with_file(name: impl Into<String>, file: File, config: Config) -> Self {
    Logger(vec![name.into()], file, config, PhantomData)
  }

  /// Log with the specified level.
  #[track_caller]
  pub fn log(&'a self, level: Level, message: impl ToBundle<'a, B>) {
    let bundle = message.to_bundle(Time::default(), level,
      Source::new(self.0.as_slice()), Location::new());

    if self.2.write_level <= level {
      let mut s = String::new();
      bundle.write(&mut s);
      self.1.writeln(s.as_bytes());
    }

    if self.2.print_level <= level {
      let mut s = String::new();
      bundle.print(&mut s);
      s += "\n";
      std::io::stdout().write_all(s.as_bytes()).unwrap();
    }
  }

  /// Log with the verbose level.
  #[track_caller]
  pub fn verbose(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Verbose, message);
  }

  /// Log with the debug level.
  #[track_caller]
  pub fn debug(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Debug, message);
  }

  /// Log with the info level.
  #[track_caller]
  pub fn info(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Info, message);
  }

  /// Log with the warning level.
  #[track_caller]
  pub fn warning(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Warning, message);
  }

  /// Log with the critical level.
  #[track_caller]
  pub fn critical(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Critical, message);
  }

  /// Log with the error level.
  #[track_caller]
  pub fn error(&'a self, message: impl ToBundle<'a, B>) {
    self.log(Level::Error, message);
  }

  /// Log with the fatal level.
  ///
  /// `Caution`: This function will **EXIT** the process.
  #[track_caller]
  pub fn fatal(&'a self, message: impl ToBundle<'a, B>) -> ! {
    self.log(Level::Fatal, message);
    std::process::exit(1);
  }
}


impl<'a, B: Bundle> Clone for Logger<'a, B> {
  fn clone(&self) -> Self {
    Logger(self.0.clone(), self.1.clone(), self.2.clone(), PhantomData)
  }
}
