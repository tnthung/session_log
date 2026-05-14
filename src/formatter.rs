#![allow(private_bounds)]
use super::bundle::Bundle;
use super::output::{Output, Mode};
use super::session::Metadata;
use std::collections::HashMap;
use std::sync::OnceLock;
use log::Record;


#[derive(Debug)]
pub struct Formatter<B: Bundle> {
  outputs:  Vec<Output>,
  sessions: HashMap<u128, (Metadata, Vec<String>)>,
  _marker:  std::marker::PhantomData<B>,
}

impl<B: Bundle+'static> Formatter<B> {
  #[allow(private_interfaces)]
  pub fn new() -> Self {
    Self {
      outputs:  Vec::with_capacity(5),
      sessions: HashMap::new(),
      _marker:  std::marker::PhantomData,
    }
  }

  pub fn add_output(&mut self, output: Output) {
    self.outputs.push(output);
  }

  fn output_record(&mut self, record: &Record, for_write: &OnceLock<String>) -> bool /* has session */ {
    let for_print = OnceLock::new();

    let mut has_session = false;
    for output in &mut self.outputs {
      let message = match output.mode() {
        Mode::Print => for_print.get_or_init(|| {
          let mut buffer = Vec::new();
          B::print(&mut buffer, record);
          String::from_utf8(buffer).unwrap()
        }),

        Mode::Write => for_write.get_or_init(|| {
          let mut buffer = Vec::new();
          B::write(&mut buffer, record);
          String::from_utf8(buffer).unwrap()
        }),

        Mode::Session => {
          has_session = true;
          continue;
        },
      };

      let writer = output.writer();
      writer.write_all(message.as_bytes()).unwrap();
      writer.write_all(b"\n").unwrap();
    }

    has_session
  }
}


pub(crate) trait FormatterTrait {
  fn flush(&mut self);
  fn add_log(&mut self, record: &Record);
  fn drop_session(&mut self, sid: u128, elapsed: std::time::Duration);
}

impl<B: Bundle+'static> FormatterTrait for Formatter<B> {
  fn flush(&mut self) {
    for output in &mut self.outputs {
      output.writer().flush().unwrap();
    }
  }

  fn add_log(&mut self, record: &Record) {
    let for_write = OnceLock::new();
    if !self.output_record(record, &for_write) { return; }

    let Some(sid) = record.key_values()
      .get("__session_log_session_id".into())
      .map(|v| v.to_u128()).flatten()
    else { return; };

    let Some(meta) = Metadata::get(sid)
      else { return; };

    let message = for_write.get_or_init(|| {
      let mut buffer = Vec::new();
      B::write(&mut buffer, record);
      String::from_utf8(buffer).unwrap()
    });

    let indent = "| ".repeat(meta.nest_level);
    let bucket = &mut self.sessions.entry(sid.into())
      .or_insert_with(|| (meta, Vec::new())).1;
    for line in message.lines() {
      bucket.push(format!("{indent}{line}"));
    }
  }

  fn drop_session(&mut self, sid: u128, elapsed: std::time::Duration) {
    let Some((meta, messages)) = self.sessions.remove(&sid)
      else { return; };

    let title = match meta.name {
      Some(name) => &format!(" {name} "),
      None       => "",
    };

    let indent = "| ".repeat(meta.nest_level - 1);
    let header = format!("{indent}┌─{title}-------\n");
    let footer = format!("{indent}└─ elapsed: {elapsed:.2?} -------\n");

    for output in &mut self.outputs {
      if output.mode() != Mode::Session { continue; }

      let writer = output.writer();
      writer.write_all(header.as_bytes()).unwrap();

      for message in &messages {
        writer.write_all(message.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
      }

      writer.write_all(footer.as_bytes()).unwrap();
    }
  }
}
