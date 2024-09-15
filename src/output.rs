

/// Output trait is used to define a common interface for all types that can both be written to a file
/// and printed to the console. This is the basic traits for each components of the log.
pub trait Output {
  /// Writes a string representation that is suitable for writing to a file.
  fn for_write(&self, f: &mut impl std::fmt::Write);

  /// Writes a string representation that is suitable for printing to the console.
  fn for_print(&self, f: &mut impl std::fmt::Write);
}
