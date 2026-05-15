use super::Component;


/// A component for ` ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SingleSpace;

impl Component for SingleSpace {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, " ").unwrap();
  }
}


/// A component for ` - ` as spacer.
#[derive(Debug, Default, Clone, Copy)]
pub struct SpacedHyphen;

impl Component for SpacedHyphen {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, " - ").unwrap();
  }
}
