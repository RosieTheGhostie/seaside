use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Special2Fn {
    #[error("madd")]
    MultiplyAdd = 0x00,
    #[error("maddu")]
    MultiplyAddUnsigned = 0x01,
    #[error("mul")]
    Multiply = 0x02,
    #[error("msub")]
    MultiplySubtract = 0x04,
    #[error("msubu")]
    MultiplySubtractUnsigned = 0x05,
    #[error("clz")]
    CountLeadingZeroes = 0x20,
    #[error("clo")]
    CountLeadingOnes = 0x21,
}
