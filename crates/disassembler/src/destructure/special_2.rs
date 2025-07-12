use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use num_traits::FromPrimitive;
use seaside_constants::fn_codes::Special2Fn;
use seaside_type_aliases::Instruction;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use Special2Fn::*;
    let rs = fields::rs(instruction);
    let rt = fields::rt(instruction);
    let rd = fields::rd(instruction);
    let r#fn = Special2Fn::from_u8(fields::r#fn(instruction))?;
    let mut components = [Component::default(); 5];
    match r#fn {
        MultiplyAdd | MultiplyAddUnsigned | MultiplySubtract | MultiplySubtractUnsigned => {
            components[0] = Component::CpuRegister(rs);
            components[1] = Component::CpuRegister(rt);
        }
        Multiply => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rs);
            components[2] = Component::CpuRegister(rt);
        }
        CountLeadingZeroes | CountLeadingOnes => {
            components[0] = Component::CpuRegister(rd);
            components[1] = Component::CpuRegister(rs);
        }
    }
    Some(DestructuredInstruction::new(
        Operation::Special2Fn(r#fn),
        components,
    ))
}
