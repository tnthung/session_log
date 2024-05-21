//! Shortcut macros for logging with formatted strings.


/// Log a debug message.
#[macro_export]
macro_rules! log_debug {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.debug(&format!($($t)*));
  }};
}


/// Log a verbose message.
#[macro_export]
macro_rules! log_verbose {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.verbose(&format!($($t)*));
  }};
}


/// Log an info message.
#[macro_export]
macro_rules! log_info {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.info(&format!($($t)*));
  }};
}


/// Log a warning message.
#[macro_export]
macro_rules! log_warning {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.warning(&format!($($t)*));
  }};
}


/// Log an critical message.
#[macro_export]
macro_rules! log_critical {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.critical(&format!($($t)*));
  }};
}


/// Log a error message.
#[macro_export]
macro_rules! log_error {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.error(&format!($($t)*));
  }};
}


/// Log a fatal message.
#[macro_export]
macro_rules! log_fatal {
  ($logger:expr, $($t:tt)*) => {{
    use $crate::prelude::*;
    $logger.fatal(&format!($($t)*));
  }};
}
