use crate::*;
use std::fmt::Write;


pub trait Formatter: Send + Sync + 'static {
  fn for_write(ctx: &Context) -> String;
  fn for_print(ctx: &Context) -> String;
}


#[derive(Debug, Clone, Copy)]
pub struct DefaultFormatter;


impl Formatter for DefaultFormatter {
  fn for_write(ctx: &Context) -> String {
    let mut string = String::new();

    match ctx {
      Context::Header { time, location, .. } => {
        time.for_write(&mut string);
        write!(string, "     ").unwrap();
        location.for_write(&mut string);
        write!(string, " - Session Start").unwrap();
      }

      Context::Footer { time, location, .. } => {
        time.for_write(&mut string);
        write!(string, "     ").unwrap();
        location.for_write(&mut string);
        write!(string, " - Session End").unwrap();
      }

      Context::Message { time, source, location, level, message, .. } => {
        time.for_write(&mut string);
        write!(string, " ").unwrap();
        level.for_write(&mut string);
        write!(string, " ").unwrap();

        if source.is_from_logger() {
          source.for_write(&mut string);
          write!(string, " - ").unwrap();
        }

        location.for_write(&mut string);
        write!(string, " - {message}").unwrap();
      }
    }

    string
  }

  fn for_print(ctx: &Context) -> String {
    let mut string = String::new();

    match ctx {
      Context::Header { time, source, location } => {
        time.for_print(&mut string);
        write!(string, "     ").unwrap();
        source.for_print(&mut string);
        write!(string, " - ").unwrap();
        location.for_print(&mut string);
        write!(string, " - Session Start").unwrap();
      }

      Context::Footer { time, source, location, elapsed } => {
        time.for_print(&mut string);
        write!(string, "     ").unwrap();
        source.for_print(&mut string);
        write!(string, " - ").unwrap();
        location.for_print(&mut string);
        write!(string, " - Session End ({:?})", elapsed).unwrap();
      }

      Context::Message { time, source, location, level, message, .. } => {
        time.for_print(&mut string);
        write!(string, " ").unwrap();
        level.for_print(&mut string);
        write!(string, " ").unwrap();
        source.for_print(&mut string);
        write!(string, " - ").unwrap();
        location.for_print(&mut string);
        write!(string, " - {message}").unwrap();
      }
    }

    string
  }
}

