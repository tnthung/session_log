#[cfg(feature = "style")]
pub mod color;
#[cfg(feature = "style")]
pub mod style;
pub mod level;
pub mod location;
pub mod message;
pub mod spacing;
pub mod target;
pub mod time;


use std::io::Write;
use log::Record;


/// Components are building block of the log message.
pub trait Component: Send + Sync {
  /// Used for non-color output, such as writing to a file.
  fn write_plain<'a>(f: &mut dyn Write, record: &Record<'a>);

  /// Used for color output, such as printing to the console.
  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::write_plain(f, record);
  }
}


pub(crate) trait ComponentEx: Component {
  #[inline(always)]
  fn write<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::write_plain(f, record);
  }

  #[cfg(not(feature = "style"))]
  #[inline(always)]
  fn print<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::write_plain(f, record);
  }

  #[cfg(feature = "style")]
  #[inline(always)]
  fn print<'a>(f: &mut dyn Write, record: &Record<'a>) {
    Self::write_style(f, record);
  }
}


impl<T: Component> ComponentEx for T {}
