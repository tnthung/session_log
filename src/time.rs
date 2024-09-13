use crate::*;
use chrono::{DateTime, Local, SecondsFormat};


/// Time type is used to represent the current time when the logging is happened. It cannot be created
/// outside of the library.
pub struct Time(DateTime<Local>);


impl Time {
  pub(crate) fn new() -> Self {
    Self(Local::now())
  }

  /// Returns the raw `DateTime<Local>` object.
  pub fn raw(&self) -> &DateTime<Local> {
    &self.0
  }

  /// Returns the formatted time string.
  /// Format: `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`
  pub fn formatted(&self) -> String {
    self.0.to_rfc3339_opts(SecondsFormat::Micros, true)
  }
}


impl Output for Time {
  fn for_write(&self, f: &mut std::fmt::Formatter<'_>) {
    write!(f, "{}", self.formatted()).unwrap();
  }

  fn for_print(&self, f: &mut std::fmt::Formatter<'_>) {
    write!(f, "{}", self.formatted()).unwrap();
  }
}
