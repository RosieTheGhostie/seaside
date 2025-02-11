use super::{component::Component, fields, operation::Operation, DestructuredInstruction};
use seaside_constants::opcodes::Opcode;
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
