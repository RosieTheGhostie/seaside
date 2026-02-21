use seaside_core::prelude::*;

use super::{Component, DestructuredInstruction, Operation};
use crate::fields;

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
