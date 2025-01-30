#![allow(dead_code)]
pub mod component;
pub mod destructured_instruction;

mod coprocessor_0;
mod coprocessor_1;
mod immediate;
mod jump;
mod operation;
mod special;
mod special_2;

pub use destructured_instruction::DestructuredInstruction;

use super::fields;
use crate::{
    constants::{instruction_format::InstructionFormat, opcodes::Opcode},
    type_aliases::{Address, Instruction},
};
use num_traits::FromPrimitive;

pub fn destructure(instruction: Instruction, address: Address) -> Option<DestructuredInstruction> {
    use InstructionFormat::*;
    let opcode = match Opcode::from_u8(fields::opcode(instruction)) {
        Some(opcode) => opcode,
        None => return None,
    };
    Some(
        match InstructionFormat::from(opcode) {
            Special => special::destructure(instruction),
            Immediate => immediate::destructure(opcode, instruction),
            Jump => jump::destructure(opcode, instruction),
            Coprocessor0 => coprocessor_0::destructure(instruction),
            Coprocessor1 => coprocessor_1::destructure(instruction),
            Special2 => special_2::destructure(instruction),
        }?
        .with_address(address),
    )
}
