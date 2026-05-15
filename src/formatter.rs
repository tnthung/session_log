#![allow(private_bounds)]
use super::bundle::Bundle;
use super::logger::Logger;
use super::output::{Output, Mode};
use super::session::Metadata;
use std::collections::HashMap;
use log::Record;


#[derive(Debug)]
pub struct Formatter<B: Bundle> {
  write_outputs:   Vec<Output>,
  print_outputs:   Vec<Output>,
  session_outputs: Vec<Output>,
  sessions:        HashMap<u128, (Metadata, Vec<String>)>,
  write_buffer:    Vec<u8>,
  #[cfg(feature = "style")]
  print_buffer:    Vec<u8>,
  _marker:         std::marker::PhantomData<B>,
}

impl<B: Bundle+'static> Formatter<B> {
  #[allow(private_interfaces)]
  pub fn new() -> Self {
    Self {
      write_outputs:   Vec::with_capacity(5),
      print_outputs:   Vec::with_capacity(5),
      session_outputs: Vec::with_capacity(5),
      sessions:        HashMap::new(),
      write_buffer:    Vec::with_capacity(256),
      #[cfg(feature = "style")]
      print_buffer:    Vec::with_capacity(256),
      _marker:         std::marker::PhantomData,
    }
  }

  pub fn add_output(mut self, output: Output) -> Self {
    match output.mode() {
      Mode::Write   => self.write_outputs.push(output),
      Mode::Print   => self.print_outputs.push(output),
      Mode::Session => self.session_outputs.push(output),
    }

    self
  }

  pub fn attach(self) {
    if self.write_outputs.is_empty() &&
       self.print_outputs.is_empty() &&
       self.session_outputs.is_empty() {
      eprintln!("Warning: No output is added to the formatter. Logs will be discarded.");
      return;
    }

    Logger::add_formatter(Box::new(self));
  }

  fn has_session_outputs(&self) -> bool {
    !self.session_outputs.is_empty()
  }

  fn write_outputs(outputs: &mut [Output], buffer: &[u8]) {
    for output in outputs {
      let writer = output.writer();
      writer.write_all(buffer).unwrap();
      writer.write_all(b"\n").unwrap();
    }
  }

  fn render_write(&mut self, record: &Record) {
    self.write_buffer.clear();
    B::write(&mut self.write_buffer, record);
  }

  #[cfg(feature = "style")]
  fn render_print(&mut self, record: &Record) {
    self.print_buffer.clear();
    B::print(&mut self.print_buffer, record);
  }

  fn output_record(&mut self, record: &Record) -> bool /* has session */ {
    let has_session = self.has_session_outputs();

    #[cfg(not(feature = "style"))] {
      if !self.write_outputs.is_empty() || !self.print_outputs.is_empty() || has_session {
        self.render_write(record);
        Self::write_outputs(&mut self.write_outputs, &self.write_buffer);
        Self::write_outputs(&mut self.print_outputs, &self.write_buffer);
      }
    }

    #[cfg(feature = "style")] {
      if !self.write_outputs.is_empty() || has_session {
        self.render_write(record);
        Self::write_outputs(&mut self.write_outputs, &self.write_buffer);
      }

      if !self.print_outputs.is_empty() {
        self.render_print(record);
        Self::write_outputs(&mut self.print_outputs, &self.print_buffer);
      }
    }

    has_session
  }
}


pub(crate) trait FormatterTrait: Send {
  fn flush(&mut self);
  fn add_log(&mut self, record: &Record);
  fn drop_session(&mut self, sid: u128, elapsed: std::time::Duration);
}

impl<B: Bundle+'static> FormatterTrait for Formatter<B> {
  fn flush(&mut self) {
    self.write_outputs.iter_mut()
      .chain(self.print_outputs.iter_mut())
      .chain(self.session_outputs.iter_mut())
      .for_each(|o| o.writer().flush().unwrap());
  }

  fn add_log(&mut self, record: &Record) {
    if !self.output_record(record) { return; }

    let Some(sid) = record.key_values()
      .get("__session_log_session_id".into())
      .map(|v| v.to_u128()).flatten()
    else { return; };

    let Some(meta) = Metadata::get(sid)
      else { return; };

    let indent = "| ".repeat(meta.nest_level);
    let bucket = &mut self.sessions.entry(sid.into())
      .or_insert_with(|| (meta, Vec::new())).1;

    for line in std::str::from_utf8(&self.write_buffer).unwrap().lines() {
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

    for output in &mut self.session_outputs {
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
