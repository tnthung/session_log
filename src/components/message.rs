use super::Component;
use std::sync::OnceLock;


/// A component that represents the log message itself.
#[derive(Debug, Default, Clone)]
pub struct Message(OnceLock<String>);

impl Message {
  fn value<'a>(&self, message: &std::fmt::Arguments<'a>) -> &str {
    self.0.get_or_init(|| message.to_string())
  }
}

impl Component for Message {
  fn write_plain<'a>(&self, f: &mut dyn std::io::Write, message: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, "{}", self.value(message)).unwrap();
  }
}
