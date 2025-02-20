use crate::output::Output;
use std::sync::Arc;


/// Source component is used to annotate which logger the message is coming from. For example, if a
/// top level logger is named `App`, then a successive logger named `IO`, the logging from the `IO`
/// logger will be annotated as `App::IO`.
#[derive(Debug, Clone, Default)]
pub struct Source(pub(crate) Vec<Arc<str>>);


impl Source {
  /// Push a new layer of source.
  pub fn push(&mut self, session: impl Into<String>) {
    self.0.push(session.into().into());
  }

  /// Pop the last layer of source.
  pub fn pop(&mut self) {
    self.0.pop();
  }
}


impl Output for Source {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0.join("::")).unwrap();
  }
}
