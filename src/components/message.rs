use super::Component;


/// A component that represents the log message itself.
#[derive(Debug, Default, Clone, Copy)]
pub struct Message;

impl Component for Message {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, record: &log::Record<'a>) {
    write!(f, "{}", record.args()).unwrap();
  }
}
