use super::Component;


/// A component that represents the log target (module path).
#[derive(Debug, Default, Clone, Copy)]
pub struct Target;

impl Component for Target {
  fn write_plain<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    write!(f, "{}", record.target()).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    write!(f, "\x1b[90m{}\x1b[0m", record.target()).unwrap();
  }
}
