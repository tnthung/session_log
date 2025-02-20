

/// Output trait is used to define a common interface for all types that can both be written to a file
/// and printed to the console. This is the basic traits for each components of the log.
pub trait Output {
  /// Writes a string representation that is suitable for writing to a file.
  fn for_write(&self, f: &mut impl std::fmt::Write);

  /// Writes a string representation that is suitable for printing to the console. By default, this method
  /// is the same as `for_write`.
  fn for_print(&self, f: &mut impl std::fmt::Write) {
    self.for_write(f);
  }

  /// Writes a string representation that is suitable for printing to the console when the color feature
  /// is enabled. By default, this method is the same as `for_print`.
  fn for_print_colored(&self, f: &mut impl std::fmt::Write) {
    self.for_print(f);
  }
}


/// Allow &Output to be used as Output.
impl<T: Output> Output for &T {
  fn for_write(&self, f: &mut impl std::fmt::Write) {
    (*self).for_write(f);
  }

  fn for_print(&self, f: &mut impl std::fmt::Write) {
    (*self).for_print(f);
  }

  fn for_print_colored(&self, f: &mut impl std::fmt::Write) {
    (*self).for_print_colored(f);
  }
}


pub(crate) trait OutputInner {
  fn write(&self, f: &mut impl std::fmt::Write);
  fn print(&self, f: &mut impl std::fmt::Write);
}


impl<T: Output> OutputInner for T {
  fn write(&self, f: &mut impl std::fmt::Write) {
    self.for_write(f);
  }

  fn print(&self, f: &mut impl std::fmt::Write) {
    #[cfg(not(feature = "color"))]
    self.for_print(f);

    #[cfg(feature = "color")]
    self.for_print_colored(f);
  }
}
