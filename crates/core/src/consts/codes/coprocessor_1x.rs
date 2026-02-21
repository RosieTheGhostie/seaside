//! Coprocessor 1X is an interesting class of instructions that was added in MIPS IV. It's fairly
//! small, and since MARS doesn't support any of these, I probably won't bother either.

use num_derive::FromPrimitive;
use strum::Display;

/// **!! UNIMPLEMENTED !!**
#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor1XFn {
    #[strum(to_string = "lwxc1")]
    LoadWordIndexed = 0x00,

    #[strum(to_string = "ldxc1")]
    LoadDoubleIndexed = 0x01,

    #[strum(to_string = "swxc1")]
    StoreWordIndexed = 0x08,

    #[strum(to_string = "sdxc1")]
    StoreDoubleIndexed = 0x09,

    #[strum(to_string = "prefx")]
    PrefetchIndexed = 0x0f,

    // If I were actually going to implement these into seaside, I would probably take the time to
    // factor out the `fmt` field.
    #[strum(to_string = "madd.s")]
    MultiplyAddSingle = 0x20,

    #[strum(to_string = "madd.d")]
    MultiplyAddDouble = 0x21,

    #[strum(to_string = "msub.s")]
    MultiplySubtractSingle = 0x28,

    #[strum(to_string = "msub.d")]
    MultiplySubtractDouble = 0x29,

    #[strum(to_string = "nmadd.s")]
    NegativeMultiplyAddSingle = 0x30,

    #[strum(to_string = "nmadd.d")]
    NegativeMultiplyAddDouble = 0x31,

    #[strum(to_string = "nmsub.s")]
    NegativeMultiplySubtractSingle = 0x38,

    #[strum(to_string = "nmsub.d")]
    NegativeMultiplySubtractDouble = 0x39,
}
