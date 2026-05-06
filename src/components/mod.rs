mod string;


/// Components are building block of the log message.
pub trait Component {
  /// Used for non-color output, such as writing to a file.
  fn write_plain(&self, f: &mut impl std::fmt::Write);

  /// Used for color output, such as printing to the console.
  #[cfg(feature = "color")]
  fn write_color(&self, f: &mut impl std::fmt::Write) { self.plain(f); }
}
