use super::{
    component::{Component, FprDisplayer, GprDisplayer},
    operation::Operation,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Default)]
pub struct DestructuredInstruction {
    pub operation: Operation,
    pub components: [Component; 5],
}

impl Display for DestructuredInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Operation::*;
        match self.operation {
            Opcode(opcode) => write!(f, "{opcode}"),
            SpecialFn(r#fn) => write!(f, "{fn}"),
            RegisterImmediateFn(r#fn) => write!(f, "{fn}"),
            Coprocessor0Fn(r#fn) => write!(f, "{fn}"),
            Coprocessor1Fn(r#fn) => write!(f, "{fn}"),
            BranchCoprocessor1 => write!(f, "bc1"),
            Special2Fn(r#fn) => write!(f, "{fn}"),
        }?;
        let mut needs_comma: bool = false;
        for component in self.components {
            if needs_comma && component.should_precede_with_comma() {
                write!(f, ",")?;
            }
            needs_comma = !matches!(component, Component::Fmt(_) | Component::Condition(_));
            match component {
                Component::Empty => break,
                Component::Fmt(fmt) => write!(f, "{fmt}"),
                Component::Gpr(index) => write!(f, " {}", GprDisplayer(index)),
                Component::WrappedGpr(index) => write!(f, "({})", GprDisplayer(index)),
                Component::Fpr(index) => write!(f, " {}", FprDisplayer(index)),
                Component::Cc(cc) => write!(f, " {cc}"),
                Component::Condition(c) => write!(f, "{}", if c { 't' } else { 'f' }),
                Component::Shamt(shamt) => write!(f, " {shamt}"),
                Component::Immediate(imm) => write!(f, " {}", imm as i16),
                Component::Offset(offset) => write!(f, " 0x{offset:04x}"),
                Component::Code(code) => write!(f, " {code}"),
                Component::Index(index) => write!(f, " 0x{index:08x}"),
            }?;
        }
        Ok(())
    }
}
