use num_derive::FromPrimitive;
use strum::Display;

/// **!! UNIMPLEMENTED !!**
#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum Coprocessor2RegisterImmediateFn {
    #[strum(to_string = "mfc2")]
    MoveFromCoprocessor2 = 0x00,

    #[strum(to_string = "mtc2")]
    MoveToCoprocessor2 = 0x04,
}
