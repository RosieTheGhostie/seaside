use super::{Component, DestructuredInstruction, Operation};
use crate::fields;
use seaside_constants::Opcode;
use seaside_type_aliases::Instruction;

pub fn destructure(opcode: Opcode, instruction: Instruction) -> Option<DestructuredInstruction> {
    Some(DestructuredInstruction::new(
        Operation::Opcode(opcode),
        [
            Component::Index(fields::jump_index(instruction)),
            Component::Empty,
            Component::Empty,
            Component::Empty,
            Component::Empty,
        ],
    ))
}
