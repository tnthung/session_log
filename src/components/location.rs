use crate::output::Output;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Clone, Default)]
pub struct Location(pub(crate) &'static str, pub(crate) u32);


impl Location {
  #[track_caller]
  pub(crate) fn new() -> Self {
    let loc = std::panic::Location::caller();
    Self(loc.file(), loc.line())
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
}


impl Output for Location {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}:{}", self.file(), self.line()).unwrap();
  }
}
