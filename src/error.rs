//! Basic error handling mechanisms
use std::error::Error;
use std::convert::From;
use std::{io, fmt};
use term;

/// The result type for the Parsing
pub type PeelResult<'a, T> = Result<T, PeelError>;

// Error conversion
macro_rules! from_error {
    ($($p:ty,)*) => (
        $(impl From<$p> for PeelError {
            fn from(err: $p) -> PeelError {
                PeelError {
                    code: ErrorType::Other,
                    description: err.description().to_owned(),
                    cause: Some(Box::new(err)),
                }
            }
        })*
    )
}

from_error! {
    io::Error,
    term::Error,
}

// Concrete error handling

#[derive(Debug, PartialEq, Eq)]
/// Error codes as indicator what happened
pub enum ErrorType {
    /// Internal errors which should not happen at all
    Internal,

    /// You have to add nodes to the Tree before traversing it
    NoTreeRoot,

    /// Errors not directly from the library (like OS errors)
    Other,
}

/// Representation for an error of the library
pub struct PeelError {
    /// The error variant
    pub code: ErrorType,

    /// Additional description for the error
    pub description: String,

    /// The cause for this error
    pub cause: Option<Box<Error>>,
}

impl fmt::Display for PeelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Code: {:?}, Description: {}",
               self.code,
               self.description)
    }
}

impl fmt::Debug for PeelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for PeelError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Throw an internal error
pub fn bail(code: ErrorType, description: &fmt::Display) -> PeelError {
    PeelError {
        code: code,
        description: description.to_string(),
        cause: None,
    }
}

macro_rules! bail {
    ($code:expr, $($fmt:tt)*) => (
        return Err(::error::bail($code, &format_args!($($fmt)*)))
    )
}
