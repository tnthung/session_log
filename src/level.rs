

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
  Debug,
  Verbose,
  Info,
  Warning,
  Critical,
  Error,
  Fatal,
}


impl std::fmt::Display for Level {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match (*self, f.alternate()) {
      (Level::Debug   , false) => write!(f, "[D]"),
      (Level::Verbose , false) => write!(f, "[V]"),
      (Level::Info    , false) => write!(f, "[I]"),
      (Level::Warning , false) => write!(f, "[W]"),
      (Level::Critical, false) => write!(f, "[C]"),
      (Level::Error   , false) => write!(f, "[E]"),
      (Level::Fatal   , false) => write!(f, "[F]"),

      (Level::Debug   , true ) => write!(f, "\x1b[90m[D]\x1b[0m"),  // Gray
      (Level::Verbose , true ) => write!(f, "\x1b[90m[V]\x1b[0m"),  // Gray
      (Level::Info    , true ) => write!(f, "\x1b[32m[I]\x1b[0m"),  // Green
      (Level::Warning , true ) => write!(f, "\x1b[33m[W]\x1b[0m"),  // Yellow
      (Level::Critical, true ) => write!(f, "\x1b[33m[C]\x1b[0m"),  // Yellow
      (Level::Error   , true ) => write!(f, "\x1b[31m[E]\x1b[0m"),  // Red
      (Level::Fatal   , true ) => write!(f, "\x1b[31m[F]\x1b[0m"),  // Red
    }
  }
}
