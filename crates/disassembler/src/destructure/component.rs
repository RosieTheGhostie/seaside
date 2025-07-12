use seaside_constants::{
    ConditionCode, NumberFormat,
    register::{Coprocessor0Register, CpuRegister, FpuRegister},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Component {
    #[default]
    Empty,
    Fmt(NumberFormat),
    CpuRegister(CpuRegister),
    WrappedCpuRegister(CpuRegister),
    Coprocessor0Register(Coprocessor0Register),
    FpuRegister(FpuRegister),
    Cc(ConditionCode),
    Condition(bool),
    Shamt(u8),
    Immediate(u16),
    HexImmediate(u16),
    Offset(u16),
    Code(u32),
    Index(u32),
}

impl Component {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        *self == Self::Empty
    }

    pub fn should_precede_with_comma(&self) -> bool {
        !matches!(
            *self,
            Self::Empty | Self::Fmt(_) | Self::WrappedCpuRegister(_) | Self::Condition(_)
        )
    }
}
