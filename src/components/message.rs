use super::Component;


/// A component that represents the log message itself.
#[derive(Debug, Default, Clone, Copy)]
pub struct Message;

impl Component for Message {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    write!(f, "{}", record.args()).unwrap();
  }
}
