use super::Component;
use log::Level as LogLevel;


/// A component that represents the log level with a fixed width of 5 characters.
#[derive(Debug, Default, Clone, Copy)]
pub struct Level;

impl Component for Level {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "[TRACE]"),
      LogLevel::Debug => write!(f, "[DEBUG]"),
      LogLevel::Info  => write!(f, "[INFO ]"),
      LogLevel::Warn  => write!(f, "[WARN ]"),
      LogLevel::Error => write!(f, "[ERROR]"),
    }.unwrap();
  }

  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    use colored::Colorize;
    write!(f, "{}", match record.level() {
      LogLevel::Trace => "[TRACE]".bright_black(),
      LogLevel::Debug => "[DEBUG]".white(),
      LogLevel::Info  => "[INFO ]".green(),
      LogLevel::Warn  => "[WARN ]".yellow(),
      LogLevel::Error => "[ERROR]".red(),
    }).unwrap();
  }
}


/// A component that represents the log level without padding.
#[derive(Debug, Default, Clone, Copy)]
pub struct LevelNoPadding;

impl Component for LevelNoPadding {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "TRACE"),
      LogLevel::Debug => write!(f, "DEBUG"),
      LogLevel::Info  => write!(f, "INFO"),
      LogLevel::Warn  => write!(f, "WARN"),
      LogLevel::Error => write!(f, "ERROR"),
    }.unwrap();
  }

  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    use colored::Colorize;
    write!(f, "{}", match record.level() {
      LogLevel::Trace => "TRACE".bright_black(),
      LogLevel::Debug => "DEBUG".white(),
      LogLevel::Info  => "INFO" .green(),
      LogLevel::Warn  => "WARN" .yellow(),
      LogLevel::Error => "ERROR".red(),
    }).unwrap();
  }
}


/// A component that represents the log level with a single character and brackets.
#[derive(Debug, Default, Clone, Copy)]
pub struct LevelCompact;

impl Component for LevelCompact {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "[T]"),
      LogLevel::Debug => write!(f, "[D]"),
      LogLevel::Info  => write!(f, "[I]"),
      LogLevel::Warn  => write!(f, "[W]"),
      LogLevel::Error => write!(f, "[E]"),
    }.unwrap();
  }

  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
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


/// A component that represents the log level with a single character without brackets.
#[derive(Debug, Default, Clone, Copy)]
pub struct LevelMoreCompact;

impl Component for LevelMoreCompact {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    match record.level() {
      LogLevel::Trace => write!(f, "T"),
      LogLevel::Debug => write!(f, "D"),
      LogLevel::Info  => write!(f, "I"),
      LogLevel::Warn  => write!(f, "W"),
      LogLevel::Error => write!(f, "E"),
    }.unwrap();
  }

  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    use colored::Colorize;
    write!(f, "{}", match record.level() {
      LogLevel::Trace => "T".bright_black(),
      LogLevel::Debug => "D".white(),
      LogLevel::Info  => "I".green(),
      LogLevel::Warn  => "W".yellow(),
      LogLevel::Error => "E".red(),
    }).unwrap();
  }
}
