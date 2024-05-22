mod error;
mod level;
mod loggable;
mod logger;
mod session;
mod context;
mod r#macro;


pub use error   ::*;
pub use level   ::Level;
pub use loggable::Loggable;
pub use logger  ::Logger;
pub use session ::Session;
pub use context ::*;
pub use r#macro ::*;


/// Re-export the most basic requirement to use the library.
///
/// # Exported
/// - `session_log::logger::Logger`
/// - `session_log::macro::*`
pub mod prelude {
  pub use crate::r#macro ::*;
  pub use crate::logger  ::Logger;
  pub use crate::loggable::Loggable;
}
