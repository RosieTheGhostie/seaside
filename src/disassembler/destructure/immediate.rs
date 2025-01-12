use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use crate::{
    constants::{fn_codes::RegisterImmediateFn, opcodes::Opcode},
    type_aliases::instruction::Instruction,
};
use num_traits::FromPrimitive;

pub fn destructure(opcode: Opcode, instruction: Instruction) -> Option<DestructuredInstruction> {
    use Opcode::*;
    let rs = fields::rs(instruction);
    let rt = fields::rt(instruction);
    let imm = fields::imm(instruction);
    let mut components = [Component::default(); 5];
    match opcode {
        RegisterImmediate => return destructure_regimm(rs, rt, imm),
        BranchEqual | BranchNotEqual => {
            components[0] = Component::Gpr(rs);
            components[1] = Component::Gpr(rt);
            components[2] = Component::Offset(imm);
        }
        BranchLessEqualZero | BranchGreaterThanZero => {
            components[0] = Component::Gpr(rs);
            components[1] = Component::Offset(imm);
        }
        AddImmediate
        | AddImmediateUnsigned
        | SetLessThanImmediate
        | SetLessThanImmediateUnsigned
        | AndImmediate
        | OrImmediate
        | XorImmediate => {
            components[0] = Component::Gpr(rt);
            components[1] = Component::Gpr(rs);
            components[2] = Component::Immediate(imm);
        }
        LoadUpperImmediate => {
            components[0] = Component::Gpr(rt);
            // It's not an offset, but treating it as one makes the disassembler display it as hex.
            components[1] = Component::Offset(imm);
        }
        LoadByte
        | LoadHalf
        | LoadWordLeft
        | LoadWord
        | LoadByteUnsigned
        | LoadHalfUnsigned
        | LoadWordRight
        | StoreByte
        | StoreHalf
        | StoreWordLeft
        | StoreWord
        | StoreConditional
        | StoreWordRight
        | LoadLinked
        | LoadWordCoprocessor1
        | LoadDoubleCoprocessor1
        | StoreWordCoprocessor1
        | StoreDoubleCoprocessor1 => {
            components[0] = Component::Gpr(rt);
            components[1] = Component::Immediate(imm);
            components[2] = Component::WrappedGpr(rs);
        }
        _ => return None,
    }
    Some(DestructuredInstruction {
        operation: Operation::Opcode(opcode),
        components,
    })
}

fn destructure_regimm(rs: u8, rt: u8, imm: u16) -> Option<DestructuredInstruction> {
    use RegisterImmediateFn::*;
    let mut components = [
        Component::Gpr(rs),
        Component::default(),
        Component::default(),
        Component::default(),
        Component::default(),
    ];
    let r#fn = RegisterImmediateFn::from_u8(rt)?;
    components[1] = match r#fn {
        BranchLessThanZero
        | BranchGreaterEqualZero
        | BranchLessThanZeroAndLink
        | BranchGreaterEqualZeroAndLink => Component::Offset(imm),
        TrapGreaterEqualImmediate
        | TrapGreaterEqualImmediateUnsigned
        | TrapLessThanImmediate
        | TrapLessThanImmediateUnsigned
        | TrapEqualImmediate
        | TrapNotEqualImmediate => Component::Immediate(imm),
    };
    Some(DestructuredInstruction {
        operation: Operation::RegisterImmediateFn(r#fn),
        components,
    })
}
