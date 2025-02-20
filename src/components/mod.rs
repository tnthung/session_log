pub mod time;
pub mod level;
pub mod source;
pub mod location;
pub mod message;

pub use time::*;
pub use level::*;
pub use source::*;
pub use location::*;
pub use message::*;


pub trait Component {
  /// Writes to the writer for writing to a file.
  fn write(&self, f: &mut impl std::fmt::Write);

  /// Writes to the writer for printing to the console.
  fn print(&self, f: &mut impl std::fmt::Write) {
    self.write(f);
  }

  /// Write to the writer for printing to the console with color. Only used when the color feature is
  /// enabled.
  fn color(&self, f: &mut impl std::fmt::Write) {
    self.print(f);
  }
}


pub(crate) trait ComponentInner {
  fn write(&self, f: &mut impl std::fmt::Write);
  fn print(&self, f: &mut impl std::fmt::Write);
}


impl<T: Component> ComponentInner for T {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    #[cfg(not(feature = "color"))]
    self.write(f);

    #[cfg(feature = "color")]
    self.color(f);
  }
}
