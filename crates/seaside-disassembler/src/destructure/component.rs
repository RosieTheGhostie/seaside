use seaside_constants::{
    NumberFormat,
    register::{CpuRegister, FpuRegister},
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Component {
    #[default]
    Empty,
    Fmt(NumberFormat),
    Gpr(CpuRegister),
    WrappedGpr(CpuRegister),
    Fpr(FpuRegister),
    Cc(u8),
    Condition(bool),
    Shamt(u8),
    Immediate(u16),
    HexImmediate(u16),
    Offset(u16),
    Code(u32),
    Index(u32),
}

pub struct GprDisplayer(pub CpuRegister);
pub struct FprDisplayer(pub FpuRegister);

impl Component {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    pub fn should_precede_with_comma(&self) -> bool {
        !matches!(
            *self,
            Self::Empty | Self::Fmt(_) | Self::WrappedGpr(_) | Self::Condition(_)
        )
    }
}

impl Display for GprDisplayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            write!(f, "${}", self.0 as u8)
        } else {
            write!(f, "{:#}", self.0)
        }
    }
}

impl Display for FprDisplayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            write!(f, "${}", self.0 as u8)
        } else {
            write!(f, "{:#}", self.0)
        }
    }
}
