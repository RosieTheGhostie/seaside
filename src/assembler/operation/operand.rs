#[derive(Debug, Eq, PartialEq)]
pub enum Operand {
    Register(u8),
    WrappedRegister(u8),
    Cc(u8),
    Shamt(u8),
    I16(i16),
    U16(u16),
    Code(u32),
    Label(String),
    JumpIndex(u32),
}

impl Operand {
    pub fn is_label(&self) -> bool {
        matches!(self, Self::Label(_))
    }
}
