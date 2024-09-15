use crate::*;


/// Location type is used to represent the location where the logging is happened. It cannot be created
/// outside of the library.
#[derive(Debug, Clone, Copy)]
pub struct Location(&'static str, u32);


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
    let path = path.strip_prefix(r"\\?\").unwrap_or(path);
    let path = path.strip_prefix(env!("CARGO_MANIFEST_DIR")).unwrap_or(path);
    let path = path.strip_prefix(r"\").unwrap_or(path);

    path.to_string()
  }
}


impl Output for Location {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}:{}", self.file(), self.line()).unwrap();
  }

  fn for_print(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}:{}", self.file(), self.line()).unwrap();
  }
}
