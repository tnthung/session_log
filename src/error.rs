

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
  DifferentLevel,
  DifferentDirectory,
  FailedToCreateFolder,
}
