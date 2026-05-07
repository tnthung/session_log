use super::Component;
use chrono::{Local, SecondsFormat};


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`.
#[derive(Debug, Clone)]
pub struct FullTime(String);

impl<'a> Component<'a> for FullTime {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    FullTime(Local::now().to_rfc3339_opts(SecondsFormat::Micros, true))
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Clone)]
pub struct LocalTime(String);

impl<'a> Component<'a> for LocalTime {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    LocalTime(Local::now().to_rfc3339_opts(SecondsFormat::Micros, false))
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Clone)]
pub struct ShortTime(String);

impl<'a> Component<'a> for ShortTime {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    ShortTime(Local::now().format("%H:%M:%S%.6f").to_string())
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS]`.
#[derive(Debug, Clone)]
pub struct SimpleTime(String);

impl<'a> Component<'a> for SimpleTime {
  fn construct(_: std::fmt::Arguments<'a>, _: log::Record<'a>) -> Self where Self: Sized {
    SimpleTime(Local::now().format("%H:%M:%S").to_string())
  }

  fn write_plain(&self, f: &mut impl std::fmt::Write) {
    write!(f, "{}", self.0).unwrap();
  }
}
