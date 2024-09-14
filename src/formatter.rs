use crate::*;


pub trait Formatter: Send {
  fn for_write(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>);
  fn for_print(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>);
}


pub struct DefaultFormatter;


impl Formatter for DefaultFormatter {
  fn for_write(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) {
    match ctx {
      Context::Header { time, location, .. } => {
        time.for_write(f);
        write!(f, "     ").unwrap();
        location.for_write(f);
        write!(f, " - Session Start").unwrap();
      }

      Context::Footer { time, location, elapsed, .. } => {
        time.for_write(f);
        write!(f, "     ").unwrap();
        location.for_write(f);
        write!(f, " - Session End ({}us)", elapsed.as_micros()).unwrap();
      }

      Context::Message { time, source, location, level, message, .. } => {
        time.for_write(f);
        write!(f, " ").unwrap();
        level.for_write(f);
        write!(f, " ").unwrap();

        if source.is_from_logger() {
          source.for_write(f);
          write!(f, " - ").unwrap();
        }

        location.for_write(f);
        write!(f, " - {message}").unwrap();
      }
    }
  }

  fn for_print(&self, ctx: &Context, f: &mut std::fmt::Formatter<'_>) {
    match ctx {
      Context::Header { time, source, location } => {
        time.for_print(f);
        write!(f, "     ").unwrap();
        source.for_print(f);
        write!(f, " - ").unwrap();
        location.for_print(f);
        write!(f, " - Session Start").unwrap();
      }

      Context::Footer { time, source, location, elapsed } => {
        time.for_print(f);
        write!(f, "     ").unwrap();
        source.for_print(f);
        write!(f, " - ").unwrap();
        location.for_print(f);
        write!(f, " - Session End ({}us)", elapsed.as_micros()).unwrap();
      }

      Context::Message { time, source, location, level, message, .. } => {
        time.for_print(f);
        write!(f, " ").unwrap();
        level.for_print(f);
        write!(f, " ").unwrap();
        source.for_print(f);
        write!(f, " - ").unwrap();
        location.for_print(f);
        write!(f, " - {message}").unwrap();
      }
    }
  }
}

