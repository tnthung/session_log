use super::Component;


/// Location component is used to annotate which location in the source code the logging is happened.
#[derive(Debug, Default, Clone, Copy)]
pub struct Location;

impl Component for Location {
  fn write_plain<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    match (record.file(), record.line()) {
      (Some(file), Some(line)) => write!(f, "{file}:{line}"),
      (Some(file), None)       => write!(f, "{file}"),
      (None,       Some(line)) => write!(f, "unknown:{line}"),
      _ => write!(f, "unknown"),
    }.unwrap();
  }

  #[cfg(feature = "style")]
  fn write_style<'a>(f: &mut dyn std::io::Write, record: &log::Record<'a>) {
    use colored::{control, Color};

    if !control::SHOULD_COLORIZE.should_colorize() {
      return Self::write_plain(f, record);
    }

    write!(f, "\x1B[{}m", Color::BrightBlack.to_fg_str()).unwrap();
    Self::write_plain(f, record);
    write!(f, "\x1B[0m").unwrap();
  }
}
