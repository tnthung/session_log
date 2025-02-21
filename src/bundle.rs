use crate::components::*;


/// A bundle is a fixed set of components that are used to create single record in the log. `write`
/// and `print` methods are used to convert the bundle into a desired format.
pub trait Bundle {
  /// Format the bundle to the writer for writing to a file.
  fn write(&self, f: &mut impl std::fmt::Write);

  /// Format the bundle to the writer for printing to the console. By default, it calls `write`.
  fn print(&self, f: &mut impl std::fmt::Write) {
    self.write(f);
  }
}


/// Convert the type into a bundle.
pub trait ToBundle<'a> {
  type B: Bundle;

  fn to_bundle(
    self,
    time    : Time,
    level   : Level,
    source  : Source<'a>,
    location: Location,
  ) -> Self::B;
}


/// The default bundle that can directly be used if no custom formatting is needed.
pub struct DefaultBundle<'a>(Time, Level, Source<'a>, Location, Message);


impl<'a> Bundle for DefaultBundle<'a> {
  fn write(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " - ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " - ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, ": ").unwrap();
    ComponentInner::write(&self.4, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " - ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " - ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, ": ").unwrap();
    ComponentInner::print(&self.4, f);
  }
}


impl<'a, T: Into<String>> ToBundle<'a> for T {
  type B = DefaultBundle<'a>;

  fn to_bundle(self, time: Time, level: Level, source: Source<'a>, location: Location) -> Self::B {
    DefaultBundle(time, level, source, location, Message(self.into()))
  }
}
