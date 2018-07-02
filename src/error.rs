use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    LogicError,
    DbError
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub fn db(msg: &str) -> Error {
        Error {
            kind: ErrorKind::DbError,
            message: String::from(msg)
        }
    }
    pub fn logic(msg: &str) -> Error {
        Error {
            kind: ErrorKind::LogicError,
            message: String::from(msg)
        }
    }
}

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