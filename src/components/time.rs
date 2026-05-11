use super::Component;
use std::sync::OnceLock;
use chrono::{Local, SecondsFormat};


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`.
#[derive(Debug, Default, Clone)]
pub struct FullTime(OnceLock<String>);

impl FullTime {
  fn value(&self) -> &str {
    self.0.get_or_init(|| Local::now().to_rfc3339_opts(SecondsFormat::Micros, true))
  }
}

impl Component for FullTime {
  fn write_plain<'a>(&self, f: &mut dyn std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, "{}", self.value()).unwrap();
  }
}


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Default, Clone)]
pub struct LocalTime(OnceLock<String>);

impl LocalTime {
  fn value(&self) -> &str {
    self.0.get_or_init(|| Local::now().to_rfc3339_opts(SecondsFormat::Micros, false))
  }
}

impl Component for LocalTime {
  fn write_plain<'a>(&self, f: &mut dyn std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, "{}", self.value()).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Default, Clone)]
pub struct ShortTime(OnceLock<String>);

impl ShortTime {
  fn value(&self) -> &str {
    self.0.get_or_init(|| Local::now().format("%H:%M:%S%.6f").to_string())
  }
}

impl Component for ShortTime {
  fn write_plain<'a>(&self, f: &mut dyn std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, "{}", self.value()).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS]`.
#[derive(Debug, Default, Clone)]
pub struct SimpleTime(OnceLock<String>);

impl SimpleTime {
  fn value(&self) -> &str {
    self.0.get_or_init(|| Local::now().format("%H:%M:%S").to_string())
  }
}

impl Component for SimpleTime {
  fn write_plain<'a>(&self, f: &mut dyn std::fmt::Write, _: &std::fmt::Arguments<'a>, _: &log::Record<'a>) {
    write!(f, "{}", self.value()).unwrap();
  }
}
