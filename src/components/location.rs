use super::Component;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Default, Clone, Copy)]
pub struct Location;

impl Location {
  fn value<'a>(record: &log::Record<'a>) -> String {
    match (record.file(), record.line()) {
      (Some(file), Some(line)) => format!("{file}:{line}"),
      _ => "unknown".to_string(),
    }
  }
}

impl Component for Location {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    write!(f, "{}", Self::value(record)).unwrap();
  }

  #[cfg(feature = "color")]
  fn write_color<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    use colored::Colorize;
    write!(f, "{}", Self::value(record).bright_black()).unwrap();
  }
}
