use super::Component;


/// A component for ` ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SingleSpace;

impl Component for SingleSpace {
  fn write_plain<'a>(&self, f: &mut dyn std::io::Write, _: &log::Record<'a>) {
    write!(f, " ").unwrap();
  }
}


/// A component for ` - ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpacedHyphen;

impl Component for SpacedHyphen {
  fn write_plain<'a>(&self, f: &mut dyn std::io::Write, _: &log::Record<'a>) {
    write!(f, " - ").unwrap();
  }
}
