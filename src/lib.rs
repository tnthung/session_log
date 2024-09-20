mod macros;
mod util;          use util     ::*;
mod writer;        use writer   ::*;
mod output;    pub use output   ::*;
mod time;      pub use time     ::*;
mod source;    pub use source   ::*;
mod location;  pub use location ::*;
mod level;     pub use level    ::*;
mod context;   pub use context  ::*;
mod formatter; pub use formatter::*;
mod loggable;  pub use loggable ::*;
mod config;    pub use config   ::*;
mod logger;    pub use logger   ::*;
mod global;    pub use global   ::*;
mod session;   pub use session  ::*;


pub mod prelude {
  pub use crate::{
    Loggable,
    Logger,
    log_debug,
    log_verbose,
    log_info,
    log_warning,
    log_critical,
    log_error,
    log_fatal,
  };
}


pub mod glog {
  pub use crate::{
    glog_debug,
    glog_verbose,
    glog_info,
    glog_warning,
    glog_critical,
    glog_error,
    glog_fatal,
  };
}
