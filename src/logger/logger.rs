use super::Loggable;
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


impl<B: Bundle> Loggable<B> for Logger<B> {
  #[track_caller]
  fn log(&self, level: Level, message: impl ToBundle<B=B>) {
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
}
