use crate::*;
use chrono::prelude::*;


/// Context of each log message.
#[derive(Debug)]
pub enum Context<'a> {
  /// Regular log message. All log made by logger will have this context.
  Log {
    time   : DateTime<Local>,
    level  : Level,
    file   : &'static str,
    line   : u32,
    logger : &'a str,
    session: Option<&'a str>,
    message: &'a str,
  },

  /// Start of a session. Occurs when a logger session is constructed.
  SessionStart {
    time   : DateTime<Local>,
    file   : &'static str,
    line   : u32,
    logger : &'a str,
    session: &'a str,
  },

  /// End of a session. Occurs when a logger session is destructed.
  SessionEnd {
    time   : DateTime<Local>,
    elapsed: i64,
    file   : &'static str,
    line   : u32,
    logger : &'a str,
    session: &'a str,
  },
}


pub type ContextProcessor = fn(&Context) -> (String, String);


impl Context<'_> {
  /// Get the time of the context in Local timezone.
  pub fn get_time(&self) -> &DateTime<Local> {
    match self {
      Context::Log          { time, .. } => time,
      Context::SessionStart { time, .. } => time,
      Context::SessionEnd   { time, .. } => time,
    }
  }

  /// Get the level of the context. Only available for log messages.
  pub fn get_level(&self) -> Option<&Level> {
    match self {
      Context::Log { level, .. } => Some(level),
      _ => None,
    }
  }

  /// Get the file of the context. Not available for session end.
  pub fn get_file(&self) -> &str {
    match self {
      Context::Log          { file, .. } => file,
      Context::SessionStart { file, .. } => file,
      Context::SessionEnd   { file, .. } => file,
    }
  }

  /// Get the line of the context.
  pub fn get_line(&self) -> &u32 {
    match self {
      Context::Log          { line, .. } => line,
      Context::SessionStart { line, .. } => line,
      Context::SessionEnd   { line, .. } => line,
    }
  }

  /// Get the logger entry name of the context.
  pub fn get_logger(&self) -> &str {
    match self {
      Context::Log          { logger, .. } => logger,
      Context::SessionStart { logger, .. } => logger,
      Context::SessionEnd   { logger, .. } => logger,
    }
  }

  /// Get the session name of the context.
  pub fn get_session(&self) -> Option<&str> {
    match self {
      Context::Log          { session, .. } => *session,
      Context::SessionStart { session, .. } => Some(session),
      Context::SessionEnd   { session, .. } => Some(session),
    }
  }

  /// Get the message of the context.
  pub fn get_message(&self) -> &str {
    match self {
      Context::Log { message, .. } => message,
      Context::SessionStart { .. } => "Session start",
      Context::SessionEnd   { .. } => "Session end",
    }
  }

  /// Get the default formatted string of time.
  /// `[YYYY]-[MM]-[DD]T[HH]:[mm]:[ss.ssssss]+[ZZ:ZZ]`
  pub fn get_time_str(&self) -> String {
    self.get_time().to_rfc3339_opts(SecondsFormat::Micros, true)
  }

  /// Get the default formatted string of name.
  ///
  /// If session is none, the logger name is returned.
  /// Otherwise, the `"{logger}:{session}"` is returned.
  pub fn get_name(&self) -> String {
    let logger  = self.get_logger();
    let session = self.get_session();

    if let Some(session) = session {
      return format!("{logger}:{session}");
    }

    logger.to_string()
  }

  /// Get the default formatted string of location. `"{file}:{line}"`
  pub fn get_location_str(&self) -> String {
    format!("{}:{}", self.get_file(), self.get_line())
  }
}


/// The default processor for outputting to the console and returning the formatted string.
pub fn processor(ctx: &Context) -> (String, String) {
  let time = ctx.get_time_str();
  let name = ctx.get_name();
  let loc  = ctx.get_location_str();

  match ctx {
    Context::Log { level, message, session, .. } if session.is_none() => (
      format!("{time} {level:#} {name} - {loc} - {message}"),
      format!("{time} {level} {name} - {loc} - {message}")
    ),

    Context::Log { level, message, .. } => (
      format!("{time} {level:#} {name} - {loc} - {message}"),
      format!("{time} {level} {loc} - {message}")
    ),

    Context::SessionStart { .. } => (
      format!("{time}     {name} - {loc} - Session start"),
      format!("{time}     {loc} - Session start")
    ),

    Context::SessionEnd { elapsed, .. } => (
      format!("{time}     {name} - {loc} - Session end, Elapsed: {elapsed}us"),
      format!("{time}     {loc} - Session end")
    ),
  }
}
