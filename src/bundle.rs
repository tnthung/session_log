use crate::components::*;


/// A bundle is a fixed set of components that are used to create single record in the log. `write`
/// and `print` methods are used to convert the bundle into a desired format. Default implementations
/// for tuple that only contains `Component` are provided (up to 12 elements), which are simply space
/// concatenated. For custom formatting or tuple that contains more elements, you need to implement
/// the `Bundle` trait manually.
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
    ComponentInner::write(&self.0, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f);
  }
}


impl<A, B> Bundle for (A, B)
where
  A: Component,
  B: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f);
  }
}


impl<A, B, C> Bundle for (A, B, C)
where
  A: Component,
  B: Component,
  C: Component,
{
  fn write(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.7, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.7, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.7, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.8, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.7, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.8, f);
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
    ComponentInner::write(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.7, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.8, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.9, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.7, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.8, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.9, f);
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
    ComponentInner::write(&self.0 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.7 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.8 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.9 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.10, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.7 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.8 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.9 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.10, f);
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
    ComponentInner::write(&self.0 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.1 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.2 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.3 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.4 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.5 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.6 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.7 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.8 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.9 , f); write!(f, " ").unwrap();
    ComponentInner::write(&self.10, f); write!(f, " ").unwrap();
    ComponentInner::write(&self.11, f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    ComponentInner::print(&self.0 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.1 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.2 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.3 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.4 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.5 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.6 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.7 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.8 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.9 , f); write!(f, " ").unwrap();
    ComponentInner::print(&self.10, f); write!(f, " ").unwrap();
    ComponentInner::print(&self.11, f);
  }
}
