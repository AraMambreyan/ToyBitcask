use std::fmt::{Debug, Display, Formatter};
use std::io;

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug)]
pub enum DatabaseError {
    KeyNotFound,
    SystemError,
}

impl From<io::Error> for DatabaseError {
    fn from(err: io::Error) -> DatabaseError {
        DatabaseError::SystemError
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> DatabaseError {
        DatabaseError::SystemError
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &DatabaseError::KeyNotFound => write!(f, "Key not found"),
            &DatabaseError::SystemError => write!(f, "System error"),
        }
    }
}
