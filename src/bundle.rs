use crate::components::*;


/// A bundle is a fixed set of components that are used to create single record in the log. `write`
/// and `print` methods are used to convert the bundle into a desired format. Default implementations
/// for tuple that only contains `Component` are provided (up to 12 elements).
///
/// For custom formatting or tuple that contains more than 12 elements, you need to implement the
/// `Bundle` trait manually.
pub trait Bundle {
  /// Format the bundle to the writer for writing to a file.
  fn write(&self, f: &mut impl std::fmt::Write);

  /// Format the bundle to the writer for printing to the console. By default, it calls `write`.
  fn print(&self, f: &mut impl std::fmt::Write) {
    self.write(f);
  }
}


/// Convert the type into a bundle.
pub trait ToBundle {
  type B: Bundle;

  fn to_bundle(
    self,
    time    : Time,
    level   : Level,
    source  : Source,
    location: Location,
  ) -> Self::B;
}


/// The default bundle that can directly be used if no custom formatting is needed.
pub type DefaultBundle = (Time, Level, Source, Location, Message);


impl<T: Into<String>> ToBundle for T {
  type B = DefaultBundle;

  fn to_bundle(self, time: Time, level: Level, source: Source, location: Location) -> Self::B {
    (time, level, source, location, Message(self.into()))
  }
}


// ---- Implementations for tuples up to 12 elements. ----


impl Bundle for () {
  fn write(&self, _: &mut impl std::fmt::Write) {}
}


impl<A> Bundle for (A,)
where
  A: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f);
  }
}


impl<A, B> Bundle for (A, B)
where
  A: Component,
  B: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f);
  }
}


impl<A, B, C> Bundle for (A, B, C)
where
  A: Component,
  B: Component,
  C: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f);
  }
}


impl<A, B, C, D> Bundle for (A, B, C, D)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f);
  }
}


impl<A, B, C, D, E> Bundle for (A, B, C, D, E)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f);
  }
}


impl<A, B, C, D, E, F> Bundle for (A, B, C, D, E, F)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f); write!(f, " ").unwrap();
    self.5.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f); write!(f, " ").unwrap();
    self.5.print(f);
  }
}


impl<A, B, C, D, E, F, G> Bundle for (A, B, C, D, E, F, G)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f); write!(f, " ").unwrap();
    self.5.write(f); write!(f, " ").unwrap();
    self.6.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f); write!(f, " ").unwrap();
    self.5.print(f); write!(f, " ").unwrap();
    self.6.print(f);
  }
}


impl<A, B, C, D, E, F, G, H> Bundle for (A, B, C, D, E, F, G, H)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
  H: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f); write!(f, " ").unwrap();
    self.5.write(f); write!(f, " ").unwrap();
    self.6.write(f); write!(f, " ").unwrap();
    self.7.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f); write!(f, " ").unwrap();
    self.5.print(f); write!(f, " ").unwrap();
    self.6.print(f); write!(f, " ").unwrap();
    self.7.print(f);
  }
}


impl<A, B, C, D, E, F, G, H, I> Bundle for (A, B, C, D, E, F, G, H, I)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
  H: Component,
  I: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f); write!(f, " ").unwrap();
    self.5.write(f); write!(f, " ").unwrap();
    self.6.write(f); write!(f, " ").unwrap();
    self.7.write(f); write!(f, " ").unwrap();
    self.8.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f); write!(f, " ").unwrap();
    self.5.print(f); write!(f, " ").unwrap();
    self.6.print(f); write!(f, " ").unwrap();
    self.7.print(f); write!(f, " ").unwrap();
    self.8.print(f);
  }
}


impl<A, B, C, D, E, F, G, H, I, J> Bundle for (A, B, C, D, E, F, G, H, I, J)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
  H: Component,
  I: Component,
  J: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.write(f); write!(f, " ").unwrap();
    self.1.write(f); write!(f, " ").unwrap();
    self.2.write(f); write!(f, " ").unwrap();
    self.3.write(f); write!(f, " ").unwrap();
    self.4.write(f); write!(f, " ").unwrap();
    self.5.write(f); write!(f, " ").unwrap();
    self.6.write(f); write!(f, " ").unwrap();
    self.7.write(f); write!(f, " ").unwrap();
    self.8.write(f); write!(f, " ").unwrap();
    self.9.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.print(f); write!(f, " ").unwrap();
    self.1.print(f); write!(f, " ").unwrap();
    self.2.print(f); write!(f, " ").unwrap();
    self.3.print(f); write!(f, " ").unwrap();
    self.4.print(f); write!(f, " ").unwrap();
    self.5.print(f); write!(f, " ").unwrap();
    self.6.print(f); write!(f, " ").unwrap();
    self.7.print(f); write!(f, " ").unwrap();
    self.8.print(f); write!(f, " ").unwrap();
    self.9.print(f);
  }
}


impl<A, B, C, D, E, F, G, H, I, J, K> Bundle for (A, B, C, D, E, F, G, H, I, J, K)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
  H: Component,
  I: Component,
  J: Component,
  K: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .write(f); write!(f, " ").unwrap();
    self.1 .write(f); write!(f, " ").unwrap();
    self.2 .write(f); write!(f, " ").unwrap();
    self.3 .write(f); write!(f, " ").unwrap();
    self.4 .write(f); write!(f, " ").unwrap();
    self.5 .write(f); write!(f, " ").unwrap();
    self.6 .write(f); write!(f, " ").unwrap();
    self.7 .write(f); write!(f, " ").unwrap();
    self.8 .write(f); write!(f, " ").unwrap();
    self.9 .write(f); write!(f, " ").unwrap();
    self.10.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .print(f); write!(f, " ").unwrap();
    self.1 .print(f); write!(f, " ").unwrap();
    self.2 .print(f); write!(f, " ").unwrap();
    self.3 .print(f); write!(f, " ").unwrap();
    self.4 .print(f); write!(f, " ").unwrap();
    self.5 .print(f); write!(f, " ").unwrap();
    self.6 .print(f); write!(f, " ").unwrap();
    self.7 .print(f); write!(f, " ").unwrap();
    self.8 .print(f); write!(f, " ").unwrap();
    self.9 .print(f); write!(f, " ").unwrap();
    self.10.print(f);
  }
}


impl<A, B, C, D, E, F, G, H, I, J, K, L> Bundle for (A, B, C, D, E, F, G, H, I, J, K, L)
where
  A: Component,
  B: Component,
  C: Component,
  D: Component,
  E: Component,
  F: Component,
  G: Component,
  H: Component,
  I: Component,
  J: Component,
  K: Component,
  L: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .write(f); write!(f, " ").unwrap();
    self.1 .write(f); write!(f, " ").unwrap();
    self.2 .write(f); write!(f, " ").unwrap();
    self.3 .write(f); write!(f, " ").unwrap();
    self.4 .write(f); write!(f, " ").unwrap();
    self.5 .write(f); write!(f, " ").unwrap();
    self.6 .write(f); write!(f, " ").unwrap();
    self.7 .write(f); write!(f, " ").unwrap();
    self.8 .write(f); write!(f, " ").unwrap();
    self.9 .write(f); write!(f, " ").unwrap();
    self.10.write(f); write!(f, " ").unwrap();
    self.11.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .print(f); write!(f, " ").unwrap();
    self.1 .print(f); write!(f, " ").unwrap();
    self.2 .print(f); write!(f, " ").unwrap();
    self.3 .print(f); write!(f, " ").unwrap();
    self.4 .print(f); write!(f, " ").unwrap();
    self.5 .print(f); write!(f, " ").unwrap();
    self.6 .print(f); write!(f, " ").unwrap();
    self.7 .print(f); write!(f, " ").unwrap();
    self.8 .print(f); write!(f, " ").unwrap();
    self.9 .print(f); write!(f, " ").unwrap();
    self.10.print(f); write!(f, " ").unwrap();
    self.11.print(f);
  }
}
