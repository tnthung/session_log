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

  /// Writes to the writer for printing to the console. Only used when the color feature is disabled.
  fn print(&self, f: &mut impl std::fmt::Write) { self.write(f); }

  /// Write to the writer for printing to the console with color. Only used when the color feature is
  /// enabled.
  fn color(&self, f: &mut impl std::fmt::Write) { self.print(f); }
}


impl Component for &str {
  fn write(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{self}").unwrap();
  }
}


pub trait ComponentFormat: Component {
  /// Writes to the writer for writing to a file.
  fn format_write(&self, f: &mut impl std::fmt::Write) {
    self.write(f);
  }

  /// Writes to the writer for printing to the console.
  fn format_print(&self, f: &mut impl std::fmt::Write) {
    #[cfg(not(feature = "color"))] self.print(f);
    #[cfg(    feature = "color" )] self.color(f);
  }
}


impl<C: Component> ComponentFormat for C {}
