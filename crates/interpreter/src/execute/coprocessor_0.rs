use num_traits::FromPrimitive;
use seaside_core::{consts::codes::Coprocessor0Fn, prelude::*};
use seaside_disassembler::fields;

use crate::{Exception, Interpreter, InterpreterState, register_file::IndexByRegister};

impl Interpreter {
    /// Executes `instruction`, which must follow the "coprocessor 0" instruction format:
    ///
    /// ```text
    /// 010000 x0x00 xxxxx xxxxx 000000xx000
    /// opcode  fn    $rt   $rd     ???
    /// ```
    pub fn execute_coprocessor_0(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use Coprocessor0Fn::*;
        let r#fn = Coprocessor0Fn::from_u8(fields::rs_raw(instruction))
            .ok_or(Exception::ReservedInstruction)?;
        let rt = fields::rt(instruction);
        let rd = fields::rd_raw(instruction);
        match r#fn {
            MoveFromCoprocessor0 => self.state.mfc0(rt, rd),
            MoveToCoprocessor0 => self.state.mtc0(rd, rt),
            ErrorReturn => self.state.eret(instruction),
        }
    }
}

impl InterpreterState {
    /// Stores the value of coprocessor 0 register `rd` in CPU register `rt`.
    fn mfc0(&mut self, rt: CpuRegister, rd: u8) -> Result<(), Exception> {
        let rd_value = match rd {
            Coprocessor0Register::VADDR => self.registers.vaddr,
            Coprocessor0Register::STATUS => self.registers.status.into_bits(),
            Coprocessor0Register::CAUSE => self.registers.cause,
            Coprocessor0Register::EPC => self.registers.epc,
            _ => return Err(Exception::MalformedInstruction),
        };

        self.registers.write(rt, rd_value);
        Ok(())
    }

    /// Stores the value of CPU register `rt` in coprocessor 0 register `rd`.
    fn mtc0(&mut self, rd: u8, rt: CpuRegister) -> Result<(), Exception> {
        let rt_value: u32 = self.registers.read(rt);
        match rd {
            Coprocessor0Register::VADDR => self.registers.vaddr = rt_value,
            Coprocessor0Register::STATUS => self.registers.status.set_bits(rt_value),
            Coprocessor0Register::CAUSE => self.registers.cause = rt_value,
            Coprocessor0Register::EPC => self.registers.epc = rt_value,
            _ => return Err(Exception::MalformedInstruction),
        }

        Ok(())
    }

    /// Sets the program counter (PC) to the value of register `epc`, then sets bit 1 of register
    /// `status` to 0.
    fn eret(&mut self, instruction: Instruction) -> Result<(), Exception> {
        const ERET: Instruction = 0x4200_0018;

        if instruction == ERET {
            self.pc = self.registers.epc;
            self.registers.status.set_exception_level(false);
            Ok(())
        } else {
            Err(Exception::MalformedInstruction)
        }
    }
}
