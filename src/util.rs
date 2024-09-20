use std::fs::{File, create_dir_all};
use chrono::{Datelike, Timelike};


pub fn new_file_name(duration: &Option<u64>, size: &Option<u64>) -> String {
  let base_name = match duration {
    Some(d) => {
      let time = chrono::Local::now();

      let mut sec = 0;

      sec += (time.ordinal() as u64 - 1) * 86400;
      sec +=  time.hour   () as u64      * 3600;
      sec +=  time.minute () as u64      * 60;
      sec +=  time.second () as u64;

      let sec = sec / d * d;

      let time = time
        .with_ordinal0((sec / 86400       ) as u32).unwrap()
        .with_hour    ((sec % 86400 / 3600) as u32).unwrap()
        .with_minute  ((sec % 3600  / 60  ) as u32).unwrap()
        .with_second  ((sec % 60          ) as u32).unwrap();

      time.format("log %Y-%m-%d_%H-%M-%S").to_string()
    }

    None => "log".to_string()
  };

  match size {
    Some(size) => {
      let mut i = 0;

      loop {
        let file_name = if i == 0 {
          base_name.clone()
        } else {
          format!("{} ({})", base_name, i)
        };

        if matches!(std::fs::metadata(&file_name), Ok(meta) if meta.len() >= *size) {
          i += 1;
          continue;
        }

        return format!("{}.log", file_name);
      }
    }

    None => format!("{}.log", base_name)
  }
}


pub fn new_file(
  directory: &str,
  duration : &Option<u64>,
  size     : &Option<u64>,
) -> (String, File) {
  create_dir_all(directory).unwrap();

  let path = format!("{directory}/{}",
    new_file_name(duration, size));

  (
    path.clone(),
    File::options()
      .create(true)
      .append(true)
      .open(path)
      .unwrap(),
  )
}
