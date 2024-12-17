use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor0Fn {
    MoveFromCoprocessor0 = 0x00, // MFC0
    MoveToCoprocessor0 = 0x04,   // MTC0
    ErrorReturn = 0x10,          // ERET
}
