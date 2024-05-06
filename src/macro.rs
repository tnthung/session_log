//! Macros under this module are shortcuts and recommended way to log the messages with corresponding
//! levels. With them, the message will be formatted with the file name and line number where the macro
//! is called. You can use them like how you use `writeln!` from the `std` crate, except for the first
//! argument you need to pass a `Loggable` object. Here is an example:
//!
//! ```
//! use session_log::{Logger, log_info};
//!
//! fn main() {
//!   let logger = Logger::new("main");
//!
//!   for i in 0..10 {
//!     log_info!(logger, "message-{}", i);
//!
//!     // or like this
//!     log_info!(logger, "message-{i}");
//!   }
//! }
//! ```
//!
//! Also, because the logging methods are implemented for the `Logger` struct, you have to import the
//! `Loggable` trait to use those methods. But with these macros, it's already included by the macro,
//! so there's no need to import it manually.
//!
//! ```
//! use session_log::Logger;
//!
//! fn main() {
//!   let logger = Logger::new("main");
//!
//!   // logger.info("message");    // this won't work without importing Loggable
//!   log_info!(logger, "message"); // this will work regardless of importing Loggable or not
//! }
//! ```


/// Log a message with the debug level.
#[macro_export]
macro_rules! log_debug {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.debug(message.as_str());
  }};
}


/// Log a message with the verbose level.
#[macro_export]
macro_rules! log_verbose {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.verbose(message.as_str());
  }};
}


/// Log a message with the info level.
#[macro_export]
macro_rules! log_info {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.info(message.as_str());
  }};
}


/// Log a message with the warning level.
#[macro_export]
macro_rules! log_warning {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.warning(message.as_str());
  }};
}


/// Log a message with the critical level.
#[macro_export]
macro_rules! log_critical {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.critical(message.as_str());
  }};
}


/// Log a message with the error level.
#[macro_export]
macro_rules! log_error {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.error(message.as_str());
  }};
}


/// Log a message with the fatal level.
#[macro_export]
macro_rules! log_fatal {
  ($loggable:expr, $($arg:tt)*) => {{
    use session_log::Loggable;
    let base = format!("{} {} - ", file!(), line!());
    let message = base + format!($($arg)*).as_str();
    $loggable.fatal(message.as_str());
  }};
}


pub use log_debug;
pub use log_verbose;
pub use log_info;
pub use log_warning;
pub use log_critical;
pub use log_error;
pub use log_fatal;
