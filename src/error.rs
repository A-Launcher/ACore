use core::fmt;

/// Result type shared by ACore APIs.
pub type CoreResult<T> = Result<T, CoreError>;

/// Errors that can be returned by core contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    InvalidArgument(&'static str),
    MissingValue(&'static str),
    Unsupported(&'static str),
    InvalidState(&'static str),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(value) => write!(f, "invalid argument: {value}"),
            Self::MissingValue(value) => write!(f, "missing value: {value}"),
            Self::Unsupported(value) => write!(f, "unsupported: {value}"),
            Self::InvalidState(value) => write!(f, "invalid state: {value}"),
        }
    }
}

impl std::error::Error for CoreError {}
