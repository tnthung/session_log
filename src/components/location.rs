use super::Component;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Default)]
pub struct Location(String);


impl Location {
  #[track_caller]
  pub fn new() -> Self {
    let loc = std::panic::Location::caller();

    let line = loc.line();
    let path = loc.file();

    // remove the UNC prefix
    let path = path.strip_prefix(r"\\?\").unwrap_or(path);

    // remove the project root prefix
    let path = path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap_or(path);

    // remove the prefix slash
    let path = path.strip_prefix(r"\").unwrap_or(path);
    let path = path.strip_prefix(r"/").unwrap_or(path);

    Self(format!("{path}:{line}"))
  }
}


impl Component for Location {
  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0).unwrap();
  }
}
