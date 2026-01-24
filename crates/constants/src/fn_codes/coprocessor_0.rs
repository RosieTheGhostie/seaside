use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor0Fn {
    #[strum(to_string = "mfc0")]
    MoveFromCoprocessor0 = 0x00,

    #[strum(to_string = "mtc0")]
    MoveToCoprocessor0 = 0x04,

    #[strum(to_string = "eret")]
    ErrorReturn = 0x10,
}
