use super::Component;
use log::Level;


impl<'a> Component<'a> for Level {
  fn construct(_: std::fmt::Arguments<'a>, record: log::Record<'a>) -> Self {
    record.level()
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    match self {
      Level::Trace => write!(f, "[T]"),
      Level::Debug => write!(f, "[D]"),
      Level::Info  => write!(f, "[I]"),
      Level::Warn  => write!(f, "[W]"),
      Level::Error => write!(f, "[E]"),
    }.unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) {
    match self {
      Level::Trace => write!(f, "\x1b[90m[T]\x1b[0m"), // gray
      Level::Debug => write!(f, "\x1b[37m[D]\x1b[0m"), // white
      Level::Info  => write!(f, "\x1b[32m[I]\x1b[0m"), // green
      Level::Warn  => write!(f, "\x1b[33m[W]\x1b[0m"), // yellow
      Level::Error => write!(f, "\x1b[31m[E]\x1b[0m"), // red
    }.unwrap();
  }
}
