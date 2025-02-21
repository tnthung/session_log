use super::Component;
use std::sync::Arc;


/// Source component is used to annotate which logger the message is coming from. For example, if a
/// top level logger is named `App`, then a successive logger named `IO`, the logging from the `IO`
/// logger will be annotated as `App::IO`.
#[derive(Debug, Clone, Default)]
pub struct Source(pub(crate) Vec<Arc<str>>);


impl Source {
  /// Creates a new source component.
  pub fn new(sources: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
    Self(sources.into_iter().map(|s| s.as_ref().to_string().into()).collect())
  }

  /// Get the root of the source.
  pub fn root(&self) -> Option<&str> {
    self.0.first().map(|s| s.as_ref())
  }

  /// Push a new layer of source.
  pub fn push(&mut self, session: impl Into<String>) {
    self.0.push(session.into().into());
  }

  /// Pop the last layer of source.
  pub fn pop(&mut self) {
    self.0.pop();
  }
}


impl Component for Source {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0.join("::")).unwrap();
  }

  fn color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0.join("::")).unwrap();
  }
}
