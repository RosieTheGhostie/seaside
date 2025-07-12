//! Coprocessor 1X is an interesting class of instructions that was added in MIPS IV. It's fairly
//! small, and since MARS doesn't support any of these, I probably won't bother either.

use num_derive::FromPrimitive;
use thiserror::Error; // these aren't errors, but i want to convert them to strings, soooo

#[derive(Clone, Copy, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum Coprocessor1XFn {
    #[error("lwxc1")]
    LoadWordIndexed = 0x00,
    #[error("ldxc1")]
    LoadDoubleIndexed = 0x01,
    #[error("swxc1")]
    StoreWordIndexed = 0x08,
    #[error("sdxc1")]
    StoreDoubleIndexed = 0x09,
    #[error("prefx")]
    PrefetchIndexed = 0x0f,
    // If I were actually going to implement these into seaside, I would probably take the time to
    // factor out the `fmt` field.
    #[error("madd.s")]
    MultiplyAddSingle = 0x20,
    #[error("madd.d")]
    MultiplyAddDouble = 0x21,
    #[error("msub.s")]
    MultiplySubtractSingle = 0x28,
    #[error("msub.d")]
    MultiplySubtractDouble = 0x29,
    #[error("nmadd.s")]
    NegativeMultiplyAddSingle = 0x30,
    #[error("nmadd.d")]
    NegativeMultiplyAddDouble = 0x31,
    #[error("nmsub.s")]
    NegativeMultiplySubtractSingle = 0x38,
    #[error("nmsub.d")]
    NegativeMultiplySubtractDouble = 0x39,
}
