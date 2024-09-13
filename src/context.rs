use crate::*;
use std::time::Duration;


pub struct Header {
  time    : Time,
  source  : Source,
  location: Location,
}


pub struct Footer {
  time    : Time,
  source  : Source,
  location: Location,
  elapsed : Duration,
}


pub struct Message {
  time    : Time,
  source  : Source,
  location: Location,
  level   : Level,
  message : String,
}


/// Context type is presenting one line of logging with their corresponding information. It has three
/// variants: Header, Footer, and Message. Header and Footer are used to represent the start and end of
/// the logging session, while Message is used to represent the actual logging message. This type cannot
/// be created outside of the library.
pub enum Context {
  Header (Header ),
  Footer (Footer ),
  Message(Message),
}


impl Context {
  #[track_caller]
  pub(crate) fn new_header(source: Source) -> Self {
    Self::Header(Header {
      time    : Time::new(),
      location: Location::new(),
      source,
    })
  }

  #[track_caller]
  pub(crate) fn new_footer(source: Source, start: &Time) -> Self {
    let elapsed = Time::new().raw().signed_duration_since(start.raw()).to_std().unwrap();

    Self::Footer(Footer {
      time    : Time::new(),
      location: Location::new(),
      source,
      elapsed,
    })
  }

  #[track_caller]
  pub(crate) fn new_message(source: Source, level: Level, message: impl Into<String>) -> Self {
    Self::Message(Message {
      time    : Time::new(),
      location: Location::new(),
      message : message.into(),
      source,
      level,
    })
  }
}


impl Output for Context {
  fn for_write(&self, f: &mut std::fmt::Formatter<'_>) {
    match self {
      Context::Header(Header{ time, location, .. }) => {
        time.for_write(f);
        write!(f, "     ").unwrap();
        location.for_write(f);
        write!(f, " - Session Start").unwrap();
      }

      Context::Footer(Footer{ time, location, elapsed, .. }) => {
        time.for_write(f);
        write!(f, "     ").unwrap();
        location.for_write(f);
        write!(f, " - Session End ({}us)", elapsed.as_micros()).unwrap();
      }

      Context::Message(Message{ time, source, location, level, message, .. }) => {
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

  fn for_print(&self, f: &mut std::fmt::Formatter<'_>) {
    match self {
      Context::Header(Header { time, source, location }) => {
        time.for_print(f);
        write!(f, "     ").unwrap();
        source.for_print(f);
        write!(f, " - ").unwrap();
        location.for_print(f);
        write!(f, " - Session Start").unwrap();
      }

      Context::Footer(Footer { time, source, location, elapsed }) => {
        time.for_print(f);
        write!(f, "     ").unwrap();
        source.for_print(f);
        write!(f, " - ").unwrap();
        location.for_print(f);
        write!(f, " - Session End ({}us)", elapsed.as_micros()).unwrap();
      }

      Context::Message(Message { time, source, location, level, message, .. }) => {
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
