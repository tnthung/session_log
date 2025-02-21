use crate::components::Level;


/// Configuration for the logger.
#[derive(Debug, Clone, Copy)]
pub struct Config {
  pub write_level: Level,
  pub print_level: Level,
}


impl Default for Config {
  fn default() -> Self {
    Self {
      write_level: Level::Info,
      print_level: Level::Info,
    }
  }
}
