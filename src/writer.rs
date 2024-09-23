use crate::*;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use chrono::{Datelike, Timelike};


#[derive(Clone)]
pub struct Writer(Arc<Mutex<WriterInner>>);
pub struct WriterInner {
  pub config     : Config,
  pub file       : File,
  pub path       : String,
  pub last_change: Instant,
  pub char_count : u64,
}


impl Writer {
  pub fn new(config: Config) -> Writer {
    let mut writers = unsafe {
      static mut WRITERS: Option<Mutex<HashMap<
        (String, Option<String>), Writer>>> = None;

      WRITERS.get_or_insert_with(
        || Mutex::new(HashMap::new())
      ).lock().unwrap()
    };

    let key = (
      config.directory  .clone(),
      config.file_prefix.clone());

    if let Some(writer) = writers.get(&key) {
      return writer.clone();
    }

    let (path, file) = config.new_file();

    let writer = Writer(Arc::new(Mutex::new(WriterInner {
      char_count : 0,
      last_change: Instant::now(),
      file,
      path,
      config,
    })));

    writers.insert(key, writer.clone());
    writer
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
      matches!(self.config.size_limit    , Some(limit) if limit != 0 && char_count >= limit) ||
      matches!(self.config.duration_limit, Some(limit) if limit != 0 && elapsed    >= limit);

    if !rotate { return; }

    self.char_count  = 0;
    self.last_change = Instant::now();

    (self.path, self.file) = self.config.new_file();
  }

  pub fn write(&mut self, message: String) {
    self.check_rotate();

    self.file.write_all(message.as_bytes()).unwrap();
    self.file.write_all(b"\n").unwrap();

    self.char_count += (message.len() + 1) as u64;
  }
}


impl Config {
  fn new_file_name(&self) -> String {
    let base_name = match self.duration_limit {
      Some(d) => {
        let time = chrono::Local::now();

        let mut s = 0;
        s += (time.ordinal() as u64 - 1) * 86400;
        s +=  time.hour   () as u64      * 3600;
        s +=  time.minute () as u64      * 60;
        s +=  time.second () as u64;

        // Align the time to the nearest duration.
        if d > 1 { s = s / d * d; }

        let (o, r) = (s / 86400, s % 86400);
        let (h, r) = (r / 3600 , r % 3600 );
        let (m, s) = (r / 60   , r % 60   );

        let time = time
          .with_ordinal0(o as u32).unwrap()
          .with_hour    (h as u32).unwrap()
          .with_minute  (m as u32).unwrap()
          .with_second  (s as u32).unwrap()
          .format("%Y-%m-%d_%H-%M-%S");

        if let Some(ref prefix) = self.file_prefix {
          format!("{prefix} {time}")
        }

        else {
          format!("{time}")
        }
      }

      None => self.file_prefix.clone()
        .unwrap_or("log".to_string())
    };

    match self.size_limit {
      Some(size) => {
        let mut i = 0;

        loop {
          let file_name = if i == 0 {
            base_name.clone()
          } else {
            format!("{} ({})", base_name, i)
          };

          if matches!(std::fs::metadata(&file_name), Ok(meta) if meta.len() >= size) {
            i += 1;
            continue;
          }

          return format!("{}.log", file_name);
        }
      }

      None => format!("{}.log", base_name)
    }
  }

  fn new_file(&self) -> (String, File) {
    create_dir_all(&self.directory).unwrap();

    let path = format!("{}/{}",
      self.directory,
      self.new_file_name());

    (
      path.clone(),
      File::options()
        .create(true)
        .append(true)
        .open(path)
        .unwrap(),
    )
  }
}
