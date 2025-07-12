use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Coprocessor0Fn {
    #[error("mfc0")]
    MoveFromCoprocessor0 = 0x00,
    #[error("mtc0")]
    MoveToCoprocessor0 = 0x04,
    #[error("eret")]
    ErrorReturn = 0x10,
}
