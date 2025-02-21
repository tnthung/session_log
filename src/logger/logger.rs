use super::Loggable;
use crate::bundle::*;
use crate::components::*;
use crate::file::*;
use std::marker::PhantomData;
use std::io::Write;


#[derive(Debug)]
pub struct Logger<B: Bundle = DefaultBundle>(Source, File, PhantomData<B>);


impl<B: Bundle> Logger<B> {
  pub(crate) fn new_with_file(name: impl AsRef<str>, file: File) -> Self {
    let source = Source::new(&[name]);
    Logger(source, file, PhantomData)
  }
}


impl<B: Bundle> Loggable<B> for Logger<B> {
  #[track_caller]
  fn log(&mut self, level: Level, message: impl ToBundle<B=B>) {
    let bundle = message.to_bundle(Time::default(), level, self.0.clone(), Location::new());

    { // writing
      let mut s = String::new();
      bundle.write(&mut s);
      self.1.writeln(s.as_bytes());
    }

    { // printing
      let mut s = String::new();
      bundle.print(&mut s);
      s += "\n";
      std::io::stdout().write_all(s.as_bytes()).unwrap();
    }
  }
}
