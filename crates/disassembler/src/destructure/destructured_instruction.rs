use super::{Component, Operation};
use core::fmt::{Display, Formatter, Result as FmtResult};
use seaside_int_utils::SignExtend;
use seaside_type_aliases::Address;

#[derive(Debug, Default)]
pub struct DestructuredInstruction {
    pub operation: Operation,
    pub components: [Component; 5],
    pub address: Address,
}

impl DestructuredInstruction {
    pub fn new(operation: Operation, components: [Component; 5]) -> Self {
        Self {
            operation,
            components,
            address: 0x00000000,
        }
    }

    pub fn with_address(mut self, address: Address) -> Self {
        self.address = address;
        self
    }
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
            Coprocessor1RegisterImmediateFn(r#fn) => write!(f, "{fn}"),
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
                Component::CpuRegister(register) => write!(f, " {register:#}"),
                Component::WrappedCpuRegister(register) => write!(f, "({register:#})"),
                Component::Coprocessor0Register(register) => write!(f, " {register:#}"),
                Component::FpuRegister(register) => write!(f, " {register:#}"),
                Component::Cc(cc) => write!(f, " {cc}"),
                Component::Condition(c) => write!(f, "{}", if c { 't' } else { 'f' }),
                Component::Shamt(shamt) => write!(f, " {shamt}"),
                Component::Immediate(imm) => write!(f, " {}", imm as i16),
                Component::HexImmediate(imm) => write!(f, " {imm:#06x}"),
                Component::Offset(offset) => {
                    let offset: i32 = <u16 as SignExtend<i32>>::sign_extend(&offset) << 2;
                    let address = (self.address + 4).wrapping_add_signed(offset);
                    write!(f, " {address:#010x}")
                }
                Component::Code(code) => write!(f, " {code}"),
                Component::Index(index) => {
                    let address = ((self.address + 4) & 0xf0000000) | (index << 2);
                    write!(f, " {address:#010x}")
                }
            }?;
        }
        Ok(())
    }
}
