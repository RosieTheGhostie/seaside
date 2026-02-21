use num_derive::FromPrimitive;
use strum::Display;

#[derive(Clone, Copy, Debug, Display, Eq, FromPrimitive, PartialEq)]
pub enum NumberFormat {
    #[strum(to_string = ".s")]
    Single = 0x10,

    #[strum(to_string = ".d")]
    Double = 0x11,

    #[strum(to_string = ".w")]
    Word = 0x14,

    /// **!! UNIMPLEMENTED !!**
    #[strum(to_string = ".l")]
    Long = 0x15,
}
