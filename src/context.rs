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
  /// Get the time of the context.
  pub fn time(&self) -> &Time {
    match self {
      Self::Header  { time, .. } => time,
      Self::Footer  { time, .. } => time,
      Self::Message { time, .. } => time,
    }
  }

  /// Get the source of the context.
  pub fn source(&self) -> &Source {
    match self {
      Self::Header  { source, .. } => source,
      Self::Footer  { source, .. } => source,
      Self::Message { source, .. } => source,
    }
  }

  /// Get the location of the context.
  pub fn location(&self) -> &Location {
    match self {
      Self::Header  { location, .. } => location,
      Self::Footer  { location, .. } => location,
      Self::Message { location, .. } => location,
    }
  }

  /// Get the level of the context. (Only available for Message variant)
  pub fn level(&self) -> Option<Level> {
    match self {
      Self::Message { level, .. } => Some(*level),
      _ => None,
    }
  }

  /// Get the message of the context. (Only available for Message variant)
  pub fn message(&self) -> Option<&str> {
    match self {
      Self::Message { message, .. } => Some(message),
      _ => None,
    }
  }

  /// Get the start time of the context. (Only available for Header variant)
  pub fn start(&self) -> Option<&Time> {
    match self {
      Self::Header { time, .. } => Some(time),
      _ => None,
    }
  }

  /// Get the elapsed time of the context. (Only available for Footer variant)
  pub fn elapsed(&self) -> Option<Duration> {
    match self {
      Self::Footer { elapsed, .. } => Some(*elapsed),
      _ => None,
    }
  }

  /// Check if the context is a header.
  pub fn is_header(&self) -> bool {
    matches!(self, Self::Header { .. })
  }

  /// Check if the context is a footer.
  pub fn is_footer(&self) -> bool {
    matches!(self, Self::Footer { .. })
  }

  /// Check if the context is a message.
  pub fn is_message(&self) -> bool {
    matches!(self, Self::Message { .. })
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
  pub(crate) fn new_footer(source: Source, start: &Time, location: Location) -> Self {
    let elapsed = Time::new().raw().signed_duration_since(start.raw()).to_std().unwrap();

    Self::Footer {
      time: Time::new(),
      source,
      location,
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
