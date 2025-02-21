use super::Component;
use std::sync::OnceLock;


/// Source component is used to annotate which logger the message is coming from. For example, if a
/// top level logger is named `App`, then a successive logger named `IO`, the logging from the `IO`
/// logger will be annotated as `App::IO`.
#[derive(Debug, Default)]
pub struct Source<'a>(pub &'a [String], OnceLock<String>);


impl<'a> Source<'a> {
  pub(crate) fn new(names: &'a [String]) -> Self {
    Source(names, OnceLock::new())
  }

  /// Get the root of the source.
  pub fn root(&self) -> Option<&str> {
    self.0.first().map(|s| s.as_ref())
  }

  /// Get the formatted source.
  pub fn as_formatted(&self) -> &String {
    self.1.get_or_init(|| self.0.join("::"))
  }
}


impl<'a> Component for Source<'a> {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.as_formatted()).unwrap();
  }

  fn color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.as_formatted()).unwrap();
  }
}
