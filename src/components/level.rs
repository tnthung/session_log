use super::Component;
use log::Level as LogLevel;


/// A component that represents the log level.
#[derive(Debug, Default, Clone, Copy)]
pub struct Level;

impl Component for Level {
  fn write_plain<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "[T]"),
      LogLevel::Debug => write!(f, "[D]"),
      LogLevel::Info  => write!(f, "[I]"),
      LogLevel::Warn  => write!(f, "[W]"),
      LogLevel::Error => write!(f, "[E]"),
    }.unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "\x1b[90m[T]\x1b[0m"), // gray
      LogLevel::Debug => write!(f, "\x1b[37m[D]\x1b[0m"), // white
      LogLevel::Info  => write!(f, "\x1b[32m[I]\x1b[0m"), // green
      LogLevel::Warn  => write!(f, "\x1b[33m[W]\x1b[0m"), // yellow
      LogLevel::Error => write!(f, "\x1b[31m[E]\x1b[0m"), // red
    }.unwrap();
  }
}
