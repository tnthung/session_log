use super::Component;
use std::sync::OnceLock;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Clone, Default)]
pub struct Location(pub(crate) &'static str, pub(crate) u32, OnceLock<String>);


impl Location {
  #[track_caller]
  pub fn new() -> Self {
    let loc = std::panic::Location::caller();
    Self(loc.file(), loc.line(), OnceLock::new())
  }

  /// Returns the raw file path. (May not be consistent)
  pub fn raw_file(&self) -> &'static str {
    self.0
  }

  /// Returns the line number.
  pub fn line(&self) -> u32 {
    self.1
  }

  /// Returns the file path that is relative to the project root.
  pub fn file(&self) -> String {
    let path = self.0;

    // remove the UNC prefix
    let path = path.strip_prefix(r"\\?\").unwrap_or(path);

    // remove the project root prefix
    let path = path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap_or(path);

    // remove the prefix slash
    let path = path.strip_prefix(r"\").unwrap_or(path);
    let path = path.strip_prefix(r"/").unwrap_or(path);

    path.to_string()
  }

  /// Get the formatted location.
  pub fn as_formatted(&self) -> &String {
    self.2.get_or_init(|| format!("{}:{}", self.file(), self.line()))
  }
}


impl Component for Location {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.as_formatted()).unwrap();
  }

  fn color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.as_formatted()).unwrap();
  }
}
