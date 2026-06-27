use std::fmt;

#[derive(Debug)]
pub enum OsintError {
    NotFound,
    Status(u16),
    Transport(String),
    Decode(String),
    Invalid(String),
}

impl fmt::Display for OsintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsintError::NotFound => write!(f, "not found"),
            OsintError::Status(code) => write!(f, "http status {code}"),
            OsintError::Transport(e) => write!(f, "transport error: {e}"),
            OsintError::Decode(e) => write!(f, "decode error: {e}"),
            OsintError::Invalid(e) => write!(f, "invalid input: {e}"),
        }
    }
}

impl std::error::Error for OsintError {}
