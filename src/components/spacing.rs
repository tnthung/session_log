use super::Component;


/// A component for ` ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SingleSpace;

impl Component for SingleSpace {
  fn write_plain<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, " ").unwrap();
  }
}


/// A component for ` - ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpacedHyphen;

impl Component for SpacedHyphen {
  fn write_plain<'a>(&self, f: &mut impl std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, " - ").unwrap();
  }
}
