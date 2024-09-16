use crate::*;
use std::time::Duration;


/// Context type is presenting one line of logging with their corresponding information. It has three
/// variants: Header, Footer, and Message. Header and Footer are used to represent the start and end of
/// the logging session, while Message is used to represent the actual logging message. This type cannot
/// be created outside of the library.
pub enum Context {
  Header {
    time    : Time,
    source  : Source,
    location: Location,
  },
  Footer {
    time    : Time,
    source  : Source,
    location: Location,
    elapsed : Duration,
  },
  Message {
    time    : Time,
    source  : Source,
    location: Location,
    level   : Level,
    message : String,
  },
}


impl Context {
  pub fn time(&self) -> &Time {
    match self {
      Self::Header  { time, .. } => time,
      Self::Footer  { time, .. } => time,
      Self::Message { time, .. } => time,
    }
  }

  pub fn source(&self) -> &Source {
    match self {
      Self::Header  { source, .. } => source,
      Self::Footer  { source, .. } => source,
      Self::Message { source, .. } => source,
    }
  }

  #[track_caller]
  pub(crate) fn new_header(source: Source) -> Self {
    Self::Header {
      time    : Time::new(),
      location: Location::new(),
      source,
    }
  }

  #[track_caller]
  pub(crate) fn new_footer(source: Source, start: &Time) -> Self {
    let elapsed = Time::new().raw().signed_duration_since(start.raw()).to_std().unwrap();

    Self::Footer {
      time    : Time::new(),
      location: Location::new(),
      source,
      elapsed,
    }
  }

  #[track_caller]
  pub(crate) fn new_message(source: Source, level: Level, message: impl Into<String>) -> Self {
    Self::Message {
      time    : Time::new(),
      location: Location::new(),
      message : message.into(),
      source,
      level,
    }
  }
}
