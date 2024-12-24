mod coprocessor_1;
mod immediate;
mod jump;
mod special;
mod syscall;

use super::{
    instruction::{fields, Instruction, InstructionFormat},
    Exception, Interpreter,
};
use crate::constants::opcodes::Opcode;
use num_traits::FromPrimitive;

impl Interpreter {
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use InstructionFormat::*;
        let opcode = match Opcode::from_u8(fields::opcode(instruction)) {
            Some(opcode) => opcode,
            None => return Err(Exception::ReservedInstruction),
        };
        match InstructionFormat::from(opcode) {
            Special => self.execute_special(instruction),
            Immediate => self.execute_immediate_format(opcode, instruction),
            Jump => self.execute_jump_format(opcode, instruction),
            Coprocessor1 => self.execute_coprocessor_1(instruction),
        }
    }
}
