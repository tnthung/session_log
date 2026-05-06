use super::Component;


impl<S: AsRef<str>> Component for S {
  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.as_ref()).unwrap();
  }
}
