use super::Component;


/// A component for ` ` as spacer.
#[derive(Debug, Clone)]
pub struct SingleSpace;

impl<'a> Component<'a> for SingleSpace {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    SingleSpace
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, " ").unwrap();
  }
}


/// A component for ` - ` as spacer.
#[derive(Debug, Clone)]
pub struct SpacedHyphen;

impl<'a> Component<'a> for SpacedHyphen {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    SpacedHyphen
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, " - ").unwrap();
  }
}
