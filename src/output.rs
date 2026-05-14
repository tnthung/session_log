use std::io::{Write, Result};


/// Defines how each message should be outputted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
  Print,
  Write,
  Session,
}


pub struct Output {
  mode:   Mode,
  writer: Box<dyn Write + Send>,
}

impl Output {
  /// Print mode is primarily used for outputting to the console and may contains colors or styles.\
  /// This is not recommended for outputting to files, as the extra decorations may make the log hard\
  /// to read. This is the only mode that supports colors and styles.
  pub fn new_print(writer: impl Write + Send + 'static) -> Self {
    Self {
      mode:   Mode::Print,
      writer: Box::new(writer),
    }
  }

  /// Write mode is used when outputting to files **PLUS** immediacy is expected. Unlike `Session` mode,\
  /// this mode will not block the output til the session is dropped, which means the output may be\
  /// interleaved with other sessions. This is suitable for environment where immediacy is expected.\
  pub fn new_write(writer: impl Write + Send + 'static) -> Self {
    Self {
      mode:   Mode::Write,
      writer: Box::new(writer),
    }
  }

  /// Session mode is used when outputting to files **PLUS** clarity is expected. It's primarily used\
  /// for debugging purposes to see how call stacks evolve. This mode will block the output til the\
  /// session is dropped, which means the output will not be interleaved with other sessions but may\
  /// loss if the program ends unexpectedly. Generally, if the `Drop` is called, then the output will\
  /// be flushed, but you never know.
  pub fn new_session(writer: impl Write + Send + 'static) -> Self {
    Self {
      mode:   Mode::Session,
      writer: Box::new(writer),
    }
  }

  #[inline(always)]
  pub(crate) fn mode(&self) -> Mode {
    self.mode
  }

  #[inline(always)]
  pub(crate) fn writer(&mut self) -> &mut dyn Write {
    &mut *self.writer
  }
}

impl Write for Output {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.writer.write(buf)
  }

  fn flush(&mut self) -> Result<()> {
    self.writer.flush()
  }
}

impl std::fmt::Debug for Output {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Output")
      .field("mode", &self.mode)
      .finish()
  }
}
