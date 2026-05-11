use super::Component;
use std::sync::OnceLock;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Default, Clone)]
pub struct Location(OnceLock<String>);

impl Location {
  fn value<'a>(&self, record: &log::Record<'a>) -> &str {
    self.0.get_or_init(|| {
      match (record.file(), record.line()) {
        (Some(file), Some(line)) => format!("{file}:{line}"),
        _ => "unknown".to_string(),
      }
    })
  }
}

impl Component for Location {
  fn write_plain<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    write!(f, "{}", self.value(record)).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    write!(f, "\x1b[90m{}\x1b[0m", self.value(record)).unwrap();
  }
}
