use num_derive::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor0Fn {
    MoveFromCoprocessor0 = 0x00, // mfc0
    MoveToCoprocessor0 = 0x04,   // mtc0
    ErrorReturn = 0x10,          // eret
}
