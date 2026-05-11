pub mod level;
pub mod location;
pub mod message;
pub mod spacing;
pub mod target;
pub mod time;


use std::fmt::Arguments;
use std::io::Write;
use log::Record;


/// Components are building block of the log message.
pub trait Component: Default + Send + Sync {
  /// Used for non-color output, such as writing to a file.
  fn write_plain<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>);

  /// Used for color output, such as printing to the console.
  #[cfg(feature = "color")]
  fn write_color<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
    self.write_plain(f, message, record);
  }
}


pub(crate) trait ComponentEx: Component {
  fn write<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
    self.write_plain(f, message, record);
  }

  #[cfg(not(feature = "color"))]
  fn print<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
    self.write_plain(f, message, record);
  }

  #[cfg(feature = "color")]
  fn print<'a>(&self, f: &mut dyn Write, message: &Arguments<'a>, record: &Record<'a>) {
    self.write_color(f, message, record);
  }
}


impl<T: Component> ComponentEx for T {}
