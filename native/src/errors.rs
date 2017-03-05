use std::io::{Error as IoError};
use std::fmt::{self, Display};
use std::error::Error;
use std::convert::From;

use glob::PatternError;

#[derive(Debug)]
pub enum RefactorError {
    Glob(PatternError),
    Io(IoError),
}

impl Display for RefactorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefactorError::Glob(ref err) => write!(f, "{}", err),
            RefactorError::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for RefactorError {
    fn description(&self) -> &str {
        match *self {
            RefactorError::Glob(ref err) => err.description(),
            RefactorError::Io(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RefactorError::Glob(ref err) => Some(err),
            RefactorError::Io(ref err) => Some(err),
        }
    }
}

impl From<PatternError> for RefactorError {
    fn from(err: PatternError) -> RefactorError {
        RefactorError::Glob(err)
    }
}

impl From<IoError> for RefactorError {
    fn from(err: IoError) -> RefactorError {
        RefactorError::Io(err)
    }
}
