use super::Component;


/// A component that represents the log target (module path).
#[derive(Debug, Default, Clone, Copy)]
pub struct Target;

impl Component for Target {
  fn write_plain<'a>(&self, f: &mut dyn std::io::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    write!(f, "{}", record.target()).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color<'a>(&self, f: &mut dyn std::io::Write, _: &std::fmt::Arguments<'a>, record: &log::Record<'a>) {
    use colored::Colorize;
    write!(f, "{}", record.target().bright_black()).unwrap();
  }
}
