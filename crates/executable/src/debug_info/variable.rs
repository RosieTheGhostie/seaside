use core::ops::Range;

// TODO: use seaside_constants::register::{Coprocessor0Register, CpuRegister, FpuRegister};
use seaside_type_aliases::{Address, Offset, Size};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Variable {
    pub name: String,
    pub display_mode: DisplayMode,
    pub location: Location,
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
    Cpu {
        // TODO: register: CpuRegister,
        size: u8,
    },
    Coprocessor0 {
        // TODO: register: Coprocessor0Register,
        size: u8,
    },
    Fpu {
        // TODO: register: FpuRegister,
        size: u8,
    },
    Stack {
        offset: Offset,
        size: Size,
    },
}
