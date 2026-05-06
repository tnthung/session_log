use super::Component;


impl Component for String {
  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{self}").unwrap();
  }
}


impl Component for &str {
  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{self}").unwrap();
  }
}
