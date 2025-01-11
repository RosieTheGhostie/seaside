use super::super::{Exception, Interpreter};
use crate::{
    constants::{opcodes::Opcode, register},
    disassembler::fields,
    sign_extend::SignExtend,
    type_aliases::instruction::Instruction,
};

impl Interpreter {
    pub fn execute_jump_format(
        &mut self,
        opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        let jump_index = fields::jump_index(instruction);
        let address = (self.pc & 0xF0000000) | (jump_index << 2);
        if opcode == Opcode::JumpAndLink {
            self.link()?;
        }
        self.pc = address;
        Ok(())
    }

    pub fn branch(&mut self, offset: u16) {
        let offset = <u16 as SignExtend<i32>>::sign_extend(&offset) << 2;
        self.pc = u32::wrapping_add_signed(self.pc, offset);
    }

    pub fn link(&mut self) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(register::RA, self.pc)
    }
}
