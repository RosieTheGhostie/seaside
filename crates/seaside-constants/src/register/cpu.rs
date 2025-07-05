use super::ParseError;
use core::{
    fmt::{Display, Formatter, Result as FmtResult, Write},
    str::FromStr,
};
use num_traits::FromPrimitive;
use strum_macros::EnumIter;
use take_exact::TakeExact;

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumIter, Eq, Ord, PartialEq, PartialOrd)]
pub enum CpuRegister {
    Zero,
    AsmTemp,
    Val0,
    Val1,
    Arg0,
    Arg1,
    Arg2,
    Arg3,
    Temp0,
    Temp1,
    Temp2,
    Temp3,
    Temp4,
    Temp5,
    Temp6,
    Temp7,
    Saved0,
    Saved1,
    Saved2,
    Saved3,
    Saved4,
    Saved5,
    Saved6,
    Saved7,
    Temp8,
    Temp9,
    Kernel0,
    Kernel1,
    GlobalPtr,
    StackPtr,
    FramePtr,
    ReturnAddr,
}

impl Display for CpuRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            f.write_char('$')?;
        }
        f.write_str(self.name())
    }
}

impl FromStr for CpuRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('$').unwrap_or(s);
        if s.is_empty() {
            return Err(ParseError::Empty);
        }
        if s == "zero" {
            return Ok(CpuRegister::Zero);
        }
        let [prefix, suffix] = s
            .bytes()
            .take_exact(ParseError::BadValue, ParseError::BadValue)?;
        match [prefix, suffix] {
            [b'a', b't'] => Ok(Self::AsmTemp),
            [b'a', index @ b'0'..=b'3'] => Ok(Self::arg(index - b'0')),
            [b'f', b'p'] => Ok(Self::FramePtr),
            [b'g', b'p'] => Ok(Self::GlobalPtr),
            [b'k', b'0'] => Ok(Self::Kernel0),
            [b'k', b'1'] => Ok(Self::Kernel1),
            [b'r', b'a'] => Ok(Self::ReturnAddr),
            [b's', b'p'] => Ok(Self::StackPtr),
            [b's', index @ b'0'..=b'7'] => Ok(Self::saved(index - b'0')),
            [b't', index @ b'0'..=b'9'] => Ok(Self::temp(index - b'0')),
            [b'v', b'0'] => Ok(Self::Kernel0),
            [b'v', b'1'] => Ok(Self::Kernel1),
            _ => Err(ParseError::BadValue),
        }
    }
}

impl FromPrimitive for CpuRegister {
    fn from_u8(n: u8) -> Option<Self> {
        (n < 32).then(|| unsafe { core::mem::transmute(n) })
    }

    fn from_u64(n: u64) -> Option<Self> {
        if let Ok(n) = n.try_into() {
            Self::from_u8(n)
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
}

impl CpuRegister {
    pub const N_REGISTERS: usize = 32;
    pub const NAMES: [&str; Self::N_REGISTERS] = [
        "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
        "t7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
        "fp", "ra",
    ];

    pub const fn name(&self) -> &'static str {
        Self::NAMES[*self as usize]
    }

    const fn arg(index: u8) -> Self {
        assert!(index <= 3, "invalid index for argument register");
        unsafe { core::mem::transmute::<u8, Self>(Self::Arg0 as u8 + index) }
    }

    const fn temp(index: u8) -> Self {
        let discriminant = match index {
            0..=7 => Self::Temp0 as u8 + index,
            8 => Self::Temp8 as u8,
            9 => Self::Temp9 as u8,
            _ => panic!("invalid index for temporary register"),
        };
        unsafe { core::mem::transmute::<u8, Self>(discriminant) }
    }

    const fn saved(index: u8) -> Self {
        assert!(index <= 7, "invalid index for saved register");
        unsafe { core::mem::transmute::<u8, Self>(Self::Saved0 as u8 + index) }
    }
}
