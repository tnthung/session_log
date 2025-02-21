use super::Component;


/// Source component is used to annotate which logger the message is coming from. For example, if a
/// top level logger is named `App`, then a successive logger named `IO`, the logging from the `IO`
/// logger will be annotated as `App::IO`.
#[derive(Debug, Clone, Default)]
pub struct Source<'a>(pub &'a [String]);


impl<'a> Source<'a> {
  /// Get the root of the source.
  pub fn root(&self) -> Option<&str> {
    self.0.first().map(|s| s.as_ref())
  }
}


impl<'a> Component for Source<'a> {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0.join("::")).unwrap();
  }

  fn color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0.join("::")).unwrap();
  }
}
