use crate::output::Output;


/// Level enum is representing the level of message & which level the logger accepts. There are 7 that
/// are available: Verbose, Debug, Info, Warning, Critical, Error, and Fatal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  Verbose,
  Debug,
  Info,
  Warning,
  Critical,
  Error,
  /// `Fatal` level is used for unrecoverable errors. When logging with this level, the program will
  /// be `TERMINATED` immediately after the message is logged.
  Fatal,
}


impl Output for Level {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    match self {
      Level::Verbose  => write!(f, "[V]"),
      Level::Debug    => write!(f, "[D]"),
      Level::Info     => write!(f, "[I]"),
      Level::Warning  => write!(f, "[W]"),
      Level::Critical => write!(f, "[C]"),
      Level::Error    => write!(f, "[E]"),
      Level::Fatal    => write!(f, "[F]"),
    }.unwrap();
  }

  fn for_print_colored(&self, f: &mut impl std::fmt::Write) {
    match self {
      Level::Verbose  => write!(f, "\x1b[90m[V]\x1b[0m"),        // gray
      Level::Debug    => write!(f, "\x1b[37m[D]\x1b[0m"),        // white
      Level::Info     => write!(f, "\x1b[32m[I]\x1b[0m"),        // green
      Level::Warning  => write!(f, "\x1b[33m[W]\x1b[0m"),        // yellow
      Level::Critical => write!(f, "\x1b[38;5;208m[C]\x1b[0m"),  // orange
      Level::Error    => write!(f, "\x1b[31m[E]\x1b[0m"),        // red
      Level::Fatal    => write!(f, "\x1b[31m[F]\x1b[0m"),        // red
    }.unwrap();
  }
}
