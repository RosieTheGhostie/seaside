use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use num_traits::FromPrimitive;
use seaside_constants::{Opcode, fn_codes::RegisterImmediateFn, register::CpuRegister};
use seaside_type_aliases::Instruction;

pub fn destructure(opcode: Opcode, instruction: Instruction) -> Option<DestructuredInstruction> {
    use Opcode::*;
    let rs = fields::rs(instruction);
    let rt = fields::rt(instruction);
    let imm = fields::imm(instruction);
    let mut components = [Component::default(); 5];
    match opcode {
        RegisterImmediate => return destructure_regimm(rs, rt, imm),
        BranchEqual | BranchNotEqual => {
            components[0] = Component::CpuRegister(rs);
            components[1] = Component::CpuRegister(rt);
            components[2] = Component::Offset(imm);
        }
        BranchLessEqualZero | BranchGreaterThanZero => {
            components[0] = Component::CpuRegister(rs);
            components[1] = Component::Offset(imm);
        }
        AddImmediate
        | AddImmediateUnsigned
        | SetLessThanImmediate
        | SetLessThanImmediateUnsigned => {
            components[0] = Component::CpuRegister(rt);
            components[1] = Component::CpuRegister(rs);
            components[2] = Component::Immediate(imm);
        }
        AndImmediate | OrImmediate | XorImmediate => {
            components[0] = Component::CpuRegister(rt);
            components[1] = Component::CpuRegister(rs);
            components[2] = Component::HexImmediate(imm);
        }
        LoadUpperImmediate => {
            components[0] = Component::CpuRegister(rt);
            components[1] = Component::HexImmediate(imm);
        }
        LoadByte | LoadHalf | LoadWordLeft | LoadWord | LoadByteUnsigned | LoadHalfUnsigned
        | LoadWordRight | StoreByte | StoreHalf | StoreWordLeft | StoreWord | StoreConditional
        | StoreWordRight | LoadLinked => {
            components[0] = Component::CpuRegister(rt);
            components[1] = Component::Immediate(imm);
            components[2] = Component::WrappedCpuRegister(rs);
        }
        LoadWordCoprocessor1
        | LoadDoubleCoprocessor1
        | StoreWordCoprocessor1
        | StoreDoubleCoprocessor1 => {
            components[0] = Component::FpuRegister(rt.to_fpu());
            components[1] = Component::Immediate(imm);
            components[2] = Component::WrappedCpuRegister(rs);
        }
        _ => return None,
    }
    Some(DestructuredInstruction::new(
        Operation::Opcode(opcode),
        components,
    ))
}

fn destructure_regimm(
    rs: CpuRegister,
    rt: CpuRegister,
    imm: u16,
) -> Option<DestructuredInstruction> {
    use RegisterImmediateFn::*;
    let mut components = [
        Component::CpuRegister(rs),
        Component::default(),
        Component::default(),
        Component::default(),
        Component::default(),
    ];
    let r#fn = RegisterImmediateFn::from_u8(rt as u8)?;
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
    Some(DestructuredInstruction::new(
        Operation::RegisterImmediateFn(r#fn),
        components,
    ))
}
