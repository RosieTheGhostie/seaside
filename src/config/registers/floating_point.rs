use super::{
    register_set::{
        make_register_set_deserialize_fn, make_register_set_serialize_fn, make_register_set_visitor,
    },
    RegisterSet,
};
use num_traits::{FromPrimitive, ToPrimitive};
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub struct FloatingPointRegister(u8);

impl FromStr for FloatingPointRegister {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, n) = s.split_at(1);
        match (prefix, n.parse::<u8>()) {
            ("f", Ok(n)) if n < 32 => Ok(Self(n)),
            _ => Err(Error::from(ErrorKind::InvalidInput)),
        }
    }
}

impl FromPrimitive for FloatingPointRegister {
    fn from_u8(n: u8) -> Option<Self> {
        if n < 32 {
            Some(Self(n))
        } else {
            None
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
        } else {
            None
        }
    }
}

impl ToPrimitive for FloatingPointRegister {
    fn to_u8(&self) -> Option<u8> {
        if self.0 < 32 {
            Some(self.0)
        } else {
            None
        }
    }

    fn to_i64(&self) -> Option<i64> {
        self.to_u8().map(|n| n as i64)
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u8().map(|n| n as u64)
    }
}

impl RegisterSet for FloatingPointRegister {
    const NUM_REGISTERS: usize = 32;
    const REGISTER_NAMES: &'static [&'static str] = &[
        "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12", "f13",
        "f14", "f15", "f16", "f17", "f18", "f19", "f20", "f21", "f22", "f23", "f24", "f25", "f26",
        "f27", "f28", "f29", "f30", "f31",
    ];
}

make_register_set_visitor!(FloatingPointRegister, FloatingPointRegisterSetVisitor);
make_register_set_serialize_fn!(FloatingPointRegister, serialize_floating_point_registers);
make_register_set_deserialize_fn!(
    FloatingPointRegister,
    FloatingPointRegisterSetVisitor,
    deserialize_floating_point_registers
);
