use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    #[error(".s")]
    Single = 0x10,
    #[error(".d")]
    Double = 0x11,
    #[error(".w")]
    Word = 0x14,
    // #[error(".l")]
    // Long = 0x15,
}
