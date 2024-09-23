use crate::*;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::time::Duration;
use std::thread::{sleep, spawn};
use chrono::{Datelike, Timelike};


#[derive(Clone)]
pub struct Writer(Arc<Mutex<WriterInner>>);
pub struct WriterInner {
  pub config     : Config,
  pub file       : File,
  pub path       : String,
  pub last_change: Instant,
  pub char_count : u64,

  #[cfg(feature = "batch")] pub buffer: String,
  #[cfg(feature = "batch")] pub kill  : bool,
  #[cfg(feature = "batch")] pub died  : bool,
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

    #[cfg(feature = "batch")]
    let interval = Duration::from_millis(
      if config.batch_interval == 0 { 1 }
      else { config.batch_interval });

    let writer = Writer(Arc::new(Mutex::new(WriterInner {
      char_count : 0,
      last_change: Instant::now(),

      #[cfg(feature = "batch")] buffer: String::with_capacity(2 * config.batch_size as usize),
      #[cfg(feature = "batch")] kill  : false,
      #[cfg(feature = "batch")] died  : false,

      file,
      path,
      config,
    })));

    #[cfg(feature = "batch")] {
      let writer = writer.clone();
      spawn(move || {
        loop {
          {
            let mut writer = writer.lock().unwrap();

            if writer.kill {
              writer.died = true;
              writer.write_batch();
              break;
            }

            writer.write_batch();
          }

          sleep(interval);
        }
      });
    }

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

  #[cfg(not(feature = "batch"))]
  pub fn write(&mut self, message: String) {
    self.check_rotate();

    self.file.write_all(message.as_bytes()).unwrap();
    self.file.write_all(b"\n").unwrap();

    self.char_count += (message.len() + 1) as u64;
  }

  #[cfg(feature = "batch")]
  pub fn write(&mut self, message: String) {
    self.buffer.push_str(&message);
    self.buffer.push('\n');

    if self.buffer.len() >= self.config.batch_size as usize {
      self.write_batch();
    }
  }

  #[cfg(feature = "batch")]
  fn write_batch(&mut self) {
    self.check_rotate();

    if self.buffer.is_empty() { return; }

    self.file.write_all(self.buffer.as_bytes()).unwrap();
    self.char_count += self.buffer.len() as u64;

    self.buffer.clear();
  }
}


#[cfg(feature = "batch")]
impl Drop for Writer {
  fn drop(&mut self) {
    // minimum strong count is 3 because:
    //   HashMap + Worker Thread + Last Writer
    if Arc::strong_count(&self.0) != 3 { return; }

    // Stop the writer thread.
    self.lock().unwrap().kill = true;

    // Wait for the writer thread to finish.
    while !self.lock().unwrap().died {
      sleep(Duration::from_millis(1)); }
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
