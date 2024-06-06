//! Shortcut macros for logging with formatted strings.


/// Log a debug message.
#[macro_export]
macro_rules! log_debug {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Debug
    || $logger.get_log_level()   <= Level::Debug {
      $logger.debug(&format!($($t)*));
    }
  }};
}


/// Log a verbose message.
#[macro_export]
macro_rules! log_verbose {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Verbose
    || $logger.get_log_level()   <= Level::Verbose {
      $logger.verbose(&format!($($t)*));
    }
  }};
}


/// Log an info message.
#[macro_export]
macro_rules! log_info {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Info
    || $logger.get_log_level() <= Level::Info {
      $logger.info(&format!($($t)*));
    }
  }};
}


/// Log a warning message.
#[macro_export]
macro_rules! log_warning {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Warning
    || $logger.get_log_level()   <= Level::Warning {
      $logger.warning(&format!($($t)*));
    }
  }};
}


/// Log an critical message.
#[macro_export]
macro_rules! log_critical {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Critical
    || $logger.get_log_level()   <= Level::Critical {
      $logger.critical(&format!($($t)*));
    }
  }};
}


/// Log a error message.
#[macro_export]
macro_rules! log_error {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Error
    || $logger.get_log_level()   <= Level::Error {
      $logger.error(&format!($($t)*));
    }
  }};
}


/// Log a fatal message.
#[macro_export]
macro_rules! log_fatal {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::{ Level, prelude::* };
    if $logger.get_write_level() <= Level::Fatal
    || $logger.get_log_level()   <= Level::Fatal {
      $logger.fatal(&format!($($t)*));
    }
  }};
}
