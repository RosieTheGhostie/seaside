use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use crate::{constants::fn_codes::Coprocessor0Fn, type_aliases::instruction::Instruction};
use num_traits::FromPrimitive;

pub fn destructure(instruction: Instruction) -> Option<DestructuredInstruction> {
    use Coprocessor0Fn::*;
    let rt = fields::rt(instruction);
    let rd = fields::rd(instruction);
    let r#fn = Coprocessor0Fn::from_u8(fields::r#fn(instruction))?;
    let mut components = [Component::default(); 5];
    match r#fn {
        MoveFromCoprocessor0 | MoveToCoprocessor0 => {
            components[0] = Component::Gpr(rt);
            components[1] = Component::Gpr(rd);
        }
        ErrorReturn => {}
    }
    Some(DestructuredInstruction {
        operation: Operation::Coprocessor0Fn(r#fn),
        components,
    })
}
