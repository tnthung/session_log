use super::Component;
use std::sync::OnceLock;
use chrono::{DateTime, Local, SecondsFormat};


/// Time component is used to annotate the time when the logging is happened.
#[derive(Debug, Clone)]
pub struct Time(pub DateTime<Local>, OnceLock<String>);


impl Time {
  /// Returns the raw `DateTime<Local>` object.
  pub fn as_raw(&self) -> &DateTime<Local> {
    &self.0
  }

  /// Returns the formatted time string.
  /// Format: `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`
  pub fn as_formatted(&self) -> &String {
    self.1.get_or_init(|| self.0.to_rfc3339_opts(SecondsFormat::Micros, true))
  }
}


impl Default for Time {
  fn default() -> Self {
    Self(Local::now(), OnceLock::new())
  }
}


impl Component for Time {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.as_formatted()).unwrap();
  }

  fn color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.as_formatted()).unwrap();
  }
}
