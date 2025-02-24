use super::Component;


/// Message component is used to annotate the log message.
///
/// * Although it is technically possible to create a logging bundle without a message, it is not
/// recommended as it will not provide any useful information.
#[derive(Debug, Clone)]
pub struct Message<'a>(pub &'a str);


impl<'a> Message<'a> {
  /// Creates a new message component.
  pub fn new<S: AsRef<str>>(msg: &'a S) -> Self {
    Self(msg.as_ref())
  }
}


impl<'a> Component for Message<'a> {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}
