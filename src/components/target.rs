use super::Component;
use log::Metadata;


#[derive(Debug)]
pub struct Target<'a>(&'a str);


impl<'a> From<Metadata<'a>> for Target<'a> {
  fn from(metadata: Metadata<'a>) -> Self {
    Target(metadata.target())
  }
}


impl<'a> Component for Target<'a> {
  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) {
    write!(f, "\x1b[90m{}\x1b[0m", self.0).unwrap();
  }
}
