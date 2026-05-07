use super::Component;


/// A component that represents the log target (module path).
#[derive(Debug)]
pub struct Target<'a>(&'a str);


impl<'a> Component<'a> for Target<'a> {
  fn construct(_: std::fmt::Arguments<'a>, record: log::Record<'a>) -> Self {
    Target(record.target())
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0).unwrap();
  }
}
