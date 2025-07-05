use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    #[error(".s")]
    SingleNoPrefix = 0,
    #[error(".d")]
    DoubleNoPrefix = 1,
    #[error(".w")]
    WordNoPrefix = 4,
    // LongNoPrefix = 5,
    #[error(".s")]
    Single = 16,
    #[error(".d")]
    Double = 17,
    #[error(".w")]
    Word = 20,
    // Long = 21,
}
