use crate::output::Output;
use chrono::{DateTime, Local, SecondsFormat};


/// Time component is used to annotate the time when the logging is happened.
#[derive(Debug, Clone, Copy)]
pub struct Time(pub(crate) DateTime<Local>);


impl Time {
  /// Returns the raw `DateTime<Local>` object.
  pub fn as_raw(&self) -> &DateTime<Local> {
    &self.0
  }

  /// Returns the formatted time string.
  /// Format: `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`
  pub fn as_formatted(&self) -> String {
    self.0.to_rfc3339_opts(SecondsFormat::Micros, true)
  }
}


impl Default for Time {
  fn default() -> Self {
    Self(Local::now())
  }
}


impl Output for Time {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.as_formatted()).unwrap();
  }
}
