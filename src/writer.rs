use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::util::*;


#[derive(Clone)]
pub struct Writer(Arc<Mutex<WriterInner>>);
pub struct WriterInner {
  pub directory  : String,
  pub file       : File,
  pub path       : String,
  pub last_change: Instant,
  pub char_count : u64,
  pub dura_limit : Option<u64>,
  pub size_limit : Option<u64>,
}


impl Writer {
  pub fn new(
    directory : &str,
    dura_limit: Option<u64>,
    size_limit: Option<u64>
  ) -> Writer {
    let (path, file) = new_file(
      directory,
      &dura_limit,
      &size_limit);

    Writer(Arc::new(Mutex::new(WriterInner {
      directory  : directory.to_string(),
      last_change: Instant::now(),
      char_count : 0,
      file,
      path,
      dura_limit,
      size_limit,
    })))
  }
}


impl std::ops::Deref for Writer {
  type Target = Arc<Mutex<WriterInner>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}


impl WriterInner {
  fn check_rotate(&mut self) {
    let char_count = self.char_count;
    let elapsed    = self.last_change.elapsed().as_secs();
    let rotate     =
      matches!(self.size_limit, Some(limit) if char_count >= limit) ||
      matches!(self.dura_limit, Some(limit) if elapsed    >= limit);

    if !rotate { return; }

    self.char_count  = 0;
    self.last_change = Instant::now();

    (self.path, self.file) = new_file(
      &self.directory,
      &self.dura_limit,
      &self.size_limit);
  }

  pub fn write(&mut self, message: &str) {
    self.check_rotate();

    writeln!(self.file, "{message}").unwrap();
    self.char_count += message.len() as u64;
  }
}
