

#[macro_export]
macro_rules! glog_verbose {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .verbose(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_debug {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .debug(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_info {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .info(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_warning {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .warning(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_critical {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .critical(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_error {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .error(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! glog_fatal {
  ($name:expr, $($t:tt)*) => {
    {
      use $crate::*;
      Global::new($name)
        .fatal(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_verbose {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.verbose(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_debug {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.debug(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_info {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.info(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_warning {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.warning(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_critical {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.critical(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_error {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.error(&format!($($t)*))
    }
  };
}


#[macro_export]
macro_rules! log_fatal {
  ($loggable:expr, $($t:tt)*) => {
    {
      use $crate::*;
      $loggable.fatal(&format!($($t)*))
    }
  };
}
