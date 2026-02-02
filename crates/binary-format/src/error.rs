use serde::{de, ser};

/// The result of serializing into or deserializing from a seaside binary document.
pub type Result<T> = core::result::Result<T, Error>;

/// An error in serialization or deserialization.
#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    /// A generic error with an explanatory message.
    #[error("{0}")]
    Other(String),
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Other(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Other(msg.to_string())
    }
}
