use super::Component;
use chrono::{Local, Utc, SecondsFormat};


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]+[ZZ:ZZ]`.
#[derive(Debug, Default, Clone, Copy)]
pub struct FullTime;

impl Component for FullTime {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, "{}", Local::now().to_rfc3339_opts(SecondsFormat::Micros, true)).unwrap();
  }
}


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalTime;

impl Component for LocalTime {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, "{}", Local::now().format("%Y-%m-%dT%H:%M:%S%.6f")).unwrap();
  }
}


/// Time component formatted as `[YYYY]-[MM]-[DD]T[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Default, Clone, Copy)]
pub struct UtcTime;

impl Component for UtcTime {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, "{}", Utc::now().format("%Y-%m-%dT%H:%M:%S%.6f")).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS.ssssss]`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ShortTime;

impl Component for ShortTime {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, "{}", Local::now().format("%H:%M:%S%.6f")).unwrap();
  }
}


/// Time component formatted as `[HH]:[mm]:[SS]`.
#[derive(Debug, Default, Clone, Copy)]
pub struct SimpleTime;

impl Component for SimpleTime {
  fn write_plain<'a, W: std::io::Write + ?Sized>(f: &mut W, _: &log::Record<'a>) {
    write!(f, "{}", Local::now().format("%H:%M:%S")).unwrap();
  }
}
