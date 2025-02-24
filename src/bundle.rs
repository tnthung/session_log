use crate::components::*;


/// A bundle is a fixed set of components that are used to create single record in the log. `write`
/// and `print` methods are used to convert the bundle into a desired format.
///
/// For tuples that only contains components up to 16 fields, the `Bundle` trait is implemented
/// automatically. If you need more fields, or custom formatting, you can implement the trait
/// manually.
pub trait Bundle {
  /// Format the bundle to the writer for writing to a file.
  fn write(&self, f: &mut impl std::fmt::Write);

  /// Format the bundle to the writer for printing to the console. By default, it calls `write`.
  fn print(&self, f: &mut impl std::fmt::Write) { self.write(f); }
}


/// Convert the type into a bundle.
pub trait ToBundle<'a, B: Bundle> {
  fn to_bundle(
    &'a self,
    time    : Time,
    level   : Level,
    source  : Source<'a>,
    location: Location,
  ) -> B;
}


/// The default bundle that can directly be used if no custom formatting is needed.
pub type DefaultBundle<'a> = (
  Time      , &'static str,
  Level     , &'static str,
  Source<'a>, &'static str,
  Location  , &'static str,
  Message<'a>);


impl<'a, T: AsRef<str> + 'a> ToBundle<'a, DefaultBundle<'a>> for T {
  fn to_bundle(&'a self, time: Time, level: Level, source: Source<'a>, location: Location) -> DefaultBundle<'a> {
    (time, " ", level, " - ", source, " - ", location, ": ", Message(self.as_ref()))
  }
}


impl Bundle for () {
  fn write(&self, _: &mut impl std::fmt::Write) {}
}


impl<C1: Component> Bundle for C1 {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.format_print(f);
  }
}


impl<C1: Component> Bundle for (C1,) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
  }
}


impl<C1: Component, C2: Component> Bundle for (C1, C2) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
  }
}


impl<C1: Component, C2: Component, C3: Component> Bundle for (C1, C2, C3) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
  }
}


impl<C1: Component, C2: Component, C3: Component, C4: Component> Bundle for (C1, C2, C3, C4) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
  }
}


impl<
  C1: Component, C2: Component, C3: Component, C4: Component,
  C5: Component
> Bundle for (C1, C2, C3, C4, C5) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
  }
}


impl<
  C1: Component, C2: Component, C3: Component, C4: Component,
  C5: Component, C6: Component
> Bundle for (C1, C2, C3, C4, C5, C6) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
    self.5.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
    self.5.format_print(f);
  }
}


impl<
  C1: Component, C2: Component, C3: Component, C4: Component,
  C5: Component, C6: Component, C7: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
    self.5.format_write(f);
    self.6.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
    self.5.format_print(f);
    self.6.format_print(f);
  }
}


impl<
  C1: Component, C2: Component, C3: Component, C4: Component,
  C5: Component, C6: Component, C7: Component, C8: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
    self.5.format_write(f);
    self.6.format_write(f);
    self.7.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
    self.5.format_print(f);
    self.6.format_print(f);
    self.7.format_print(f);
  }
}


impl<
  C1: Component, C2: Component, C3: Component, C4: Component,
  C5: Component, C6: Component, C7: Component, C8: Component,
  C9: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
    self.5.format_write(f);
    self.6.format_write(f);
    self.7.format_write(f);
    self.8.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
    self.5.format_print(f);
    self.6.format_print(f);
    self.7.format_print(f);
    self.8.format_print(f);
  }
}


impl<
  C1: Component, C2 : Component, C3: Component, C4: Component,
  C5: Component, C6 : Component, C7: Component, C8: Component,
  C9: Component, C10: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0.format_write(f);
    self.1.format_write(f);
    self.2.format_write(f);
    self.3.format_write(f);
    self.4.format_write(f);
    self.5.format_write(f);
    self.6.format_write(f);
    self.7.format_write(f);
    self.8.format_write(f);
    self.9.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0.format_print(f);
    self.1.format_print(f);
    self.2.format_print(f);
    self.3.format_print(f);
    self.4.format_print(f);
    self.5.format_print(f);
    self.6.format_print(f);
    self.7.format_print(f);
    self.8.format_print(f);
    self.9.format_print(f);
  }
}


impl<
  C1: Component, C2 : Component, C3 : Component, C4: Component,
  C5: Component, C6 : Component, C7 : Component, C8: Component,
  C9: Component, C10: Component, C11: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
  }
}


impl<
  C1: Component, C2 : Component, C3 : Component, C4 : Component,
  C5: Component, C6 : Component, C7 : Component, C8 : Component,
  C9: Component, C10: Component, C11: Component, C12: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
    self.11.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
    self.11.format_print(f);
  }
}


impl<
  C1 : Component, C2 : Component, C3 : Component, C4 : Component,
  C5 : Component, C6 : Component, C7 : Component, C8 : Component,
  C9 : Component, C10: Component, C11: Component, C12: Component,
  C13: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
    self.11.format_write(f);
    self.12.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
    self.11.format_print(f);
    self.12.format_print(f);
  }
}


impl<
  C1 : Component, C2 : Component, C3 : Component, C4 : Component,
  C5 : Component, C6 : Component, C7 : Component, C8 : Component,
  C9 : Component, C10: Component, C11: Component, C12: Component,
  C13: Component, C14: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
    self.11.format_write(f);
    self.12.format_write(f);
    self.13.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
    self.11.format_print(f);
    self.12.format_print(f);
    self.13.format_print(f);
  }
}


impl<
  C1 : Component, C2 : Component, C3 : Component, C4 : Component,
  C5 : Component, C6 : Component, C7 : Component, C8 : Component,
  C9 : Component, C10: Component, C11: Component, C12: Component,
  C13: Component, C14: Component, C15: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
    self.11.format_write(f);
    self.12.format_write(f);
    self.13.format_write(f);
    self.14.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
    self.11.format_print(f);
    self.12.format_print(f);
    self.13.format_print(f);
    self.14.format_print(f);
  }
}


impl<
  C1 : Component, C2 : Component, C3 : Component, C4 : Component,
  C5 : Component, C6 : Component, C7 : Component, C8 : Component,
  C9 : Component, C10: Component, C11: Component, C12: Component,
  C13: Component, C14: Component, C15: Component, C16: Component
> Bundle for (C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16) {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_write(f);
    self.1 .format_write(f);
    self.2 .format_write(f);
    self.3 .format_write(f);
    self.4 .format_write(f);
    self.5 .format_write(f);
    self.6 .format_write(f);
    self.7 .format_write(f);
    self.8 .format_write(f);
    self.9 .format_write(f);
    self.10.format_write(f);
    self.11.format_write(f);
    self.12.format_write(f);
    self.13.format_write(f);
    self.14.format_write(f);
    self.15.format_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    self.0 .format_print(f);
    self.1 .format_print(f);
    self.2 .format_print(f);
    self.3 .format_print(f);
    self.4 .format_print(f);
    self.5 .format_print(f);
    self.6 .format_print(f);
    self.7 .format_print(f);
    self.8 .format_print(f);
    self.9 .format_print(f);
    self.10.format_print(f);
    self.11.format_print(f);
    self.12.format_print(f);
    self.13.format_print(f);
    self.14.format_print(f);
    self.15.format_print(f);
  }
}
