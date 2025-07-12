mod coprocessor_0;
mod coprocessor_1;
mod immediate;
mod jump;
mod special;
mod special_2;
mod syscall;

use crate::{Exception, Interpreter};
use seaside_constants::InstructionFormat;
use seaside_disassembler::fields;
use seaside_type_aliases::Instruction;

impl Interpreter {
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use InstructionFormat::*;
        let opcode = fields::opcode(instruction).ok_or(Exception::ReservedInstruction)?;
        match InstructionFormat::from(opcode) {
            Special => self.execute_special(instruction),
            Immediate => self.execute_immediate_format(opcode, instruction),
            Jump => self.execute_jump_format(opcode, instruction),
            Coprocessor0 => self.execute_coprocessor_0(instruction),
            Coprocessor1 => self.execute_coprocessor_1(instruction),
            Special2 => self.execute_special_2(instruction),
        }
    }
}
