pub mod level;
pub mod location;
pub mod message;
pub mod spacing;
pub mod target;
pub mod time;


use std::fmt::Arguments;
use log::Record;


/// Components are building block of the log message.
pub trait Component<'a> {
  fn construct(message: Arguments<'a>, record: Record<'a>) -> Self where Self: Sized;

  /// Used for non-color output, such as writing to a file.
  fn write_plain(&self, f: &mut impl std::fmt::Write);

  /// Used for color output, such as printing to the console.
  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) { self.write_plain(f); }
}


pub(crate) trait ComponentEx<'a>: Component<'a> {
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

impl<'a, T: Component<'a>> ComponentEx<'a> for T {}
