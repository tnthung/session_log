use super::Component;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Default)]
pub struct Location(String);


impl<'a> Component<'a> for Location {
  fn construct(_: std::fmt::Arguments<'a>, record: log::Record<'a>) -> Self where Self: Sized {
    match (record.file(), record.line()) {
      (Some(file), Some(line)) => Self(format!("{file}:{line}")),
      _ => Self("unknown".to_string()),
    }
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0).unwrap();
  }
}
