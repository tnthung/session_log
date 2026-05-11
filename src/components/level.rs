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
    use colored::Colorize;
    write!(f, "{}", match record.level() {
      LogLevel::Trace => "[T]".bright_black(),
      LogLevel::Debug => "[D]".white(),
      LogLevel::Info  => "[I]".green(),
      LogLevel::Warn  => "[W]".yellow(),
      LogLevel::Error => "[E]".red(),
    }).unwrap();
  }
}
