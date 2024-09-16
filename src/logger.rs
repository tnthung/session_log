use crate::*;

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;
use std::time::Instant;
use std::marker::PhantomData;


/// Logger is the base type that actually handle the loggings
pub struct Logger<F: Formatter> {
  name       : String,
  write_level: Level,
  print_level: Level,
  directory  : String,
  file       : Mutex<File>,
  path       : Mutex<String>,
  last_change: Mutex<Instant>,
  dura_limit : Option<u64>,
  char_count : Mutex<u64>,
  size_limit : Option<u64>,
  _phantom   : PhantomData<F>,
}


impl Logger<DefaultFormatter> {
  /// Creating a new logger with the given name & default configurations.
  pub fn default(name: impl Into<String>) -> Self {
    Self::new(name, Config::default())
  }
}


impl<F: Formatter> Logger<F> {
  /// Creating a new logger with the given name.
  pub fn new(name: impl Into<String>, config: Config) -> Self {
    let (path, file) = new_file(
      &config.directory,
      &config.duration_limit,
      &config.size_limit);

    Logger {
      name       : name.into(),
      write_level: config.write_level,
      print_level: config.print_level,
      directory  : config.directory.clone(),
      file       : Mutex::new(file),
      path       : Mutex::new(path),
      char_count : Mutex::new(0),
      size_limit : config.size_limit,
      dura_limit : config.duration_limit,
      last_change: Mutex::new(Instant::now()),
      _phantom   : PhantomData,
    }
  }

  pub(crate) fn check_rotate(&self) {
    let mut char_count  = self.char_count .lock().unwrap();
    let mut last_change = self.last_change.lock().unwrap();

    let rotate =
      matches!(self.size_limit, Some(limit) if *char_count                      >= limit) ||
      matches!(self.dura_limit, Some(limit) if  last_change.elapsed().as_secs() >= limit);

    if !rotate { return; }

    *char_count  = 0;
    *last_change = Instant::now();

    (*self.path.lock().unwrap(), *self.file.lock().unwrap()) =
      new_file(&self.directory, &self.dura_limit, &self.size_limit);
  }
}


impl<F: Formatter> LoggableInner for Logger<F> {
  fn log(&self, level: Level, message: &str) {
    let ctx = Context::new_message(
      Source::new(&self.name),
      level,
      message
    );

    if level >= self.print_level {
      let mut string = String::new();
      F::for_print(&ctx, &mut string);

      println!("{string}");
    }

    if level >= self.write_level {
      self.check_rotate();

      let mut string = String::new();
      F::for_write(&ctx, &mut string);

      let mut file       = self.file      .lock().unwrap();
      let mut char_count = self.char_count.lock().unwrap();

      file.write_all(string.as_bytes()).unwrap();
      file.write_all(b"\n").unwrap();
      *char_count += string.len() as u64;
    }
  }
}


impl<F: Formatter> Loggable for Logger<F> {
  fn root_name(&self) -> &str {
    &self.name
  }

  fn name(&self) -> &str {
    &self.name
  }

  fn path(&self) -> String {
    self.path.lock().unwrap().clone()
  }

  fn write_level(&self) -> Level {
    self.write_level
  }

  fn print_level(&self) -> Level {
    self.print_level
  }
}
