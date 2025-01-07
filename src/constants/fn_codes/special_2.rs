use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Special2Fn {
    MultiplyAdd = 0x00,              // MADD
    MultiplyAddUnsigned = 0x01,      // MADDU
    Multiply = 0x02,                 // MUL
    MultiplySubtract = 0x04,         // MSUB
    MultiplySubtractUnsigned = 0x05, // MSUBU
    CountLeadingZeroes = 0x20,       // CLZ
    CountLeadingOnes = 0x21,         // CLO
}
