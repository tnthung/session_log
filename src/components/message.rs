use super::Component;


pub struct Message<'a>(std::fmt::Arguments<'a>);


impl<'a> Component<'a> for Message<'a> {
  fn construct(message: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self {
    Message(message)
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}
