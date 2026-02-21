use core::ops::Range;

use seaside_core::{
    prelude::*,
    types::{Offset, Size},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Variable {
    pub name: String,
    pub display_mode: DisplayMode,
    pub location: Location,
    pub size: Size,
    pub scope: Range<Address>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DisplayMode {
    #[default]
    RawBytes,

    Pointer,

    SignedInteger,
    UnsignedInteger,
    Float,

    Utf8Character,
    CString,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Location {
    Cpu(IndexedRegister),
    Coprocessor0(IndexedRegister),
    Fpu(IndexedRegister),
    Stack(Offset),
}
