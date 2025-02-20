use crate::output::Output;


/// Message component is used to annotate the log message.
///
/// * Although it is technically possible to create a logging bundle without a message, it is not
/// recommended as it will not provide any useful information.
#[derive(Debug, Clone)]
pub struct Message(pub String);


impl Message {
  /// Creates a new message component.
  pub fn new<S: Into<String>>(msg: S) -> Self {
    Self(msg.into())
  }
}


impl Output for Message {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}
