use core::ops::Range;

// TODO: use seaside_constants::register::{Coprocessor0Register, CpuRegister, FpuRegister};
use seaside_type_aliases::{Address, Offset, Size};
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
    Cpu(/* TODO: CpuRegister */),
    Coprocessor0(/* TODO: Coprocessor0Register */),
    Fpu(/* TODO: FpuRegister */),
    Stack(Offset),
}
