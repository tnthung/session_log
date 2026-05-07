mod string;

pub mod level;
pub mod location;
pub mod target;


/// Components are building block of the log message.
pub trait Component {
  /// Used for non-color output, such as writing to a file.
  fn write_plain(&self, f: &mut impl std::fmt::Write);

  /// Used for color output, such as printing to the console.
  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) { self.plain(f); }
}


pub(crate) trait ComponentEx: Component {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.write_plain(f);
  }

  #[cfg(not(feature = "color"))]
  fn print(&self, f: &mut impl std::fmt::Write) {
    self.write_plain(f);
  }

  #[cfg(feature = "color")]
  fn print(&self, f: &mut impl std::fmt::Write) {
    self.write_color(f);
  }
}

impl<T: Component> ComponentEx for T {}
