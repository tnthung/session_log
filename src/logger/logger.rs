use crate::bundle::*;
use crate::components::*;
use crate::file::*;
use crate::config::*;
use std::marker::PhantomData;
use std::io::Write;


#[derive(Debug)]
pub struct Logger<B: Bundle = DefaultBundle>(Source, File, Config, PhantomData<B>);


impl<B: Bundle> Logger<B> {
  pub(crate) fn new_with_file(name: impl AsRef<str>, file: File, config: Config) -> Self {
    let source = Source::new(&[name]);
    Logger(source, file, config, PhantomData)
  }
}


impl<B: Bundle> Logger<B> {
  /// Log with the specified level.
  #[track_caller]
  pub fn log(&self, level: Level, message: impl ToBundle<B=B>) {
    let bundle = message.to_bundle(Time::default(), level, self.0.clone(), Location::new());

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
  pub fn verbose(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Verbose, message);
  }

  /// Log with the debug level.
  #[track_caller]
  pub fn debug(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Debug, message);
  }

  /// Log with the info level.
  #[track_caller]
  pub fn info(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Info, message);
  }

  /// Log with the warning level.
  #[track_caller]
  pub fn warning(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Warning, message);
  }

  /// Log with the critical level.
  #[track_caller]
  pub fn critical(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Critical, message);
  }

  /// Log with the error level.
  #[track_caller]
  pub fn error(&self, message: impl ToBundle<B=B>) {
    self.log(Level::Error, message);
  }

  /// Log with the fatal level.
  ///
  /// `Caution`: This function will **EXIT** the process.
  #[track_caller]
  pub fn fatal(&self, message: impl ToBundle<B=B>) -> ! {
    self.log(Level::Fatal, message);
    std::process::exit(1);
  }
}
