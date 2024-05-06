

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
  DifferentLevel,
  DifferentDirectory,
  FailedToCreateFolder,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionErrorKind {
  SessionDied,
}
