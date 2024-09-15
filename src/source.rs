use crate::*;
use std::rc::Rc;


/// Source type is used to store the absolute path of the source logger. A source must be started with
/// logger, and can have any number layers of session. This type cannot be constructed outside of the
/// library.
#[derive(Debug, Clone)]
pub struct Source(Vec<Rc<str>>);


impl Source {
  pub(crate) fn new(logger: impl Into<String>) -> Self {
    Self(vec![logger.into().into()])
  }

  pub(crate) fn session(&self, session: impl Into<String>) -> Self {
    let mut src = self.0.iter().cloned().collect::<Vec<_>>();
    src.push(session.into().into());

    Self(src)
  }

  /// Returns the raw `Rc<str>` slice.
  pub fn raw(&self) -> &[Rc<str>] {
    &self.0
  }

  /// Returns the number of layers of the source.
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Checks if the source is from the logger.
  pub fn is_from_logger(&self) -> bool {
    self.0.len() == 1
  }

  /// Checks if the source is from the session.
  pub fn is_from_session(&self) -> bool {
    self.0.len() > 1
  }
}


impl Output for Source {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0.first().unwrap()).unwrap();

    if self.0.len() > 1 {
      write!(f, ":{}", self.0.last ().unwrap()).unwrap();
    }
  }

  fn for_print(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0.join(":")).unwrap();
  }
}
