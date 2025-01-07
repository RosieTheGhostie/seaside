use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Special2Fn {
    MultiplyAdd = 0x00,              // madd
    MultiplyAddUnsigned = 0x01,      // maddu
    Multiply = 0x02,                 // mul
    MultiplySubtract = 0x04,         // msub
    MultiplySubtractUnsigned = 0x05, // msubu
    CountLeadingZeroes = 0x20,       // clz
    CountLeadingOnes = 0x21,         // clo
}
