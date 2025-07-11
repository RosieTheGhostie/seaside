use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use num_traits::FromPrimitive;
use seaside_constants::fn_codes::Coprocessor0Fn;
use seaside_type_aliases::Instruction;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use Coprocessor0Fn::*;
    let rt = fields::rt(instruction);
    let rd = fields::rd(instruction).to_indexed();
    let r#fn = Coprocessor0Fn::from_u8(fields::rs_raw(instruction))?;
    let mut components = [Component::default(); 5];
    match r#fn {
        MoveFromCoprocessor0 | MoveToCoprocessor0 => {
            components[0] = Component::CpuRegister(rt);
            components[1] = Component::Coprocessor0Register(rd.try_into_coprocessor_0()?);
        }
        ErrorReturn => {}
    }
    Some(DestructuredInstruction::new(
        Operation::Coprocessor0Fn(r#fn),
        components,
    ))
}
