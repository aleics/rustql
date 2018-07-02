use std::error;
use std::fmt;

/// ErrorKind defines the type of errors that are available
#[derive(Debug)]
pub enum ErrorKind {
    LogicError,
    DbError
}

/// Error defines the error structure
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    /// Create a database error variable
    pub fn db(msg: &str) -> Error {
        Error {
            kind: ErrorKind::DbError,
            message: String::from(msg)
        }
    }
    /// Create a logic error variable
    pub fn logic(msg: &str) -> Error {
        Error {
            kind: ErrorKind::LogicError,
            message: String::from(msg)
        }
    }
}

/// Implementation of the standard `Error` trait to enable reading a description of the error
impl error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}