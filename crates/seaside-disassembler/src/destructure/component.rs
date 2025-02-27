use seaside_constants::NumberFormat;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Component {
    #[default]
    Empty,
    Fmt(NumberFormat),
    Gpr(u8),
    WrappedGpr(u8),
    Fpr(u8),
    Cc(u8),
    Condition(bool),
    Shamt(u8),
    Immediate(u16),
    HexImmediate(u16),
    Offset(u16),
    Code(u32),
    Index(u32),
}

pub struct GprDisplayer(pub u8);
pub struct FprDisplayer(pub u8);

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
        use seaside_constants::register::*;
        if f.alternate() {
            write!(f, "${}", self.0)
        } else {
            write!(
                f,
                "${}",
                match self.0 {
                    ZERO => "zero",
                    AT => "at",
                    V0 => "v0",
                    V1 => "v1",
                    A0 => "a0",
                    A1 => "a1",
                    A2 => "a2",
                    A3 => "a3",
                    T0 => "t0",
                    T1 => "t1",
                    T2 => "t2",
                    T3 => "t3",
                    T4 => "t4",
                    T5 => "t5",
                    T6 => "t6",
                    T7 => "t7",
                    S0 => "s0",
                    S1 => "s1",
                    S2 => "s2",
                    S3 => "s3",
                    S4 => "s4",
                    S5 => "s5",
                    S6 => "s6",
                    S7 => "s7",
                    T8 => "t8",
                    T9 => "t9",
                    K0 => "k0",
                    K1 => "k1",
                    GP => "gp",
                    SP => "sp",
                    FP => "fp",
                    RA => "ra",
                    _ => "??",
                }
            )
        }
    }
}

impl Display for FprDisplayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            write!(f, "${}", self.0)
        } else {
            write!(f, "$f{}", self.0)
        }
    }
}
