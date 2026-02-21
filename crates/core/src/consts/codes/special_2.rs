use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Special2Fn {
    #[strum(to_string = "madd")]
    MultiplyAdd = 0x00,

    #[strum(to_string = "maddu")]
    MultiplyAddUnsigned = 0x01,

    #[strum(to_string = "mul")]
    Multiply = 0x02,

    #[strum(to_string = "msub")]
    MultiplySubtract = 0x04,

    #[strum(to_string = "msubu")]
    MultiplySubtractUnsigned = 0x05,

    #[strum(to_string = "clz")]
    CountLeadingZeroes = 0x20,

    #[strum(to_string = "clo")]
    CountLeadingOnes = 0x21,
}
