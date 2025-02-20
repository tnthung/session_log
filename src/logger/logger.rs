use super::Loggable;
use crate::bundle::*;
use crate::components::*;
use std::marker::PhantomData;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use chrono::Timelike;


#[derive(Debug)]
pub struct Logger<B: Bundle = DefaultBundle>(Source, File, u32, PhantomData<B>);


impl<B: Bundle> Logger<B> {
  pub fn new(name: impl AsRef<str>) -> Self {
    let source = Source::new(&[name]);

    // create path
    let path = format!("./logs/{}", source.root().unwrap());

    // create dir
    std::fs::create_dir_all(&path).unwrap();

    // get time
    let time = Local::now();

    // add timestamp
    let path = format!("{path}/{}.log",
      time.format("%Y-%m-%d %H"));

    // create file
    let file = std::fs::OpenOptions::new()
      .create(true)
      .append(true)
      .open(path)
      .unwrap();

    Logger(source, file, time.hour(), PhantomData)
  }
}


impl<B: Bundle> Loggable<B> for Logger<B> {
  #[track_caller]
  fn log(&mut self, level: Level, message: impl crate::bundle::ToBundle<B=B>) {
    let bundle = message.to_bundle(Time::default(), level, self.0.clone(), Location::new());

    { // writing
      let mut s = String::new();
      bundle.write(&mut s);
      s += "\n";
      self.1.write_all(s.as_bytes()).unwrap();
    }

    { // printing
      let mut s = String::new();
      bundle.print(&mut s);
      s += "\n";
      std::io::stdout().write_all(s.as_bytes()).unwrap();
    }

    'change_file: {
      let time = Local::now();

      if self.2 == time.hour() {
        break 'change_file;
      }

      self.2 = time.hour();

      let path = format!("./logs/{}/{}.log",
        self.0.root().unwrap(),
        time.format("%Y-%m-%d %H").to_string());

      self.1 = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    }
  }
}
