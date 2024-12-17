use super::{
    instruction::{fields, Instruction, InstructionFormat},
    Exception, Interpreter,
};
use crate::constants::{fn_codes::SpecialFn, opcodes::Opcode};
use num_traits::FromPrimitive;
use std::mem::transmute;

impl Interpreter {
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use InstructionFormat::*;
        let opcode = match Opcode::from_u8(fields::opcode(instruction)) {
            Some(opcode) => opcode,
            None => return Err(Exception::ReservedInstruction),
        };
        match InstructionFormat::from(opcode) {
            Register => self.execute_register_format(opcode, instruction),
            Immediate => self.execute_immediate_format(opcode, instruction),
            Jump => self.execute_jump_format(opcode, instruction),
        }
    }

    fn execute_register_format(
        &mut self,
        _opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        use SpecialFn::*;
        let rs = fields::rs(instruction);
        let rt = fields::rt(instruction);
        let rd = fields::rd(instruction);
        let rs_value = self.registers.read_u32_from_cpu(rs)?;
        let rt_value = self.registers.read_u32_from_cpu(rt)?;
        let shamt = fields::shamt(instruction);
        let r#fn = match SpecialFn::from_u8(fields::r#fn(instruction)) {
            Some(fn_code) => fn_code,
            None => return Err(Exception::ReservedInstruction),
        };
        match r#fn {
            ShiftLeftLogical => self.sll(rd, rt_value, shamt),
            MoveConditional => todo!(),
            ShiftRightLogical => self.srl(rd, rt_value, shamt),
            ShiftRightArithmetic => self.sra(rd, rt_value, shamt),
            ShiftLeftLogicalVariable => self.sllv(rd, rs_value, rt_value),
            ShiftRightLogicalVariable => self.srlv(rd, rs_value, rt_value),
            ShiftRightArithmeticVariable => self.srav(rd, rs_value, rt_value),
            JumpRegister => todo!(),
            JumpAndLinkRegister => todo!(),
            MoveZero => self.movz(rd, rs_value, rt_value),
            MoveNotZero => self.movn(rd, rs_value, rt_value),
            SystemCall => self.syscall(),
            Break => todo!(),
            MoveFromHigh => self.mfhi(rd),
            MoveToHigh => self.mthi(rs_value),
            MoveFromLow => self.mflo(rd),
            MoveToLow => self.mtlo(rs_value),
            Multiply => self.mult(rs_value, rt_value),
            MultiplyUnsigned => self.multu(rs_value, rt_value),
            Divide => self.div(rs_value, rt_value),
            DivideUnsigned => self.divu(rs_value, rt_value),
            Add => self.add(rd, rs_value, rt_value),
            AddUnsigned => self.addu(rd, rs_value, rt_value),
            Subtract => self.sub(rd, rs_value, rt_value),
            SubtractUnsigned => self.subu(rd, rs_value, rt_value),
            And => self.and(rd, rs_value, rt_value),
            Or => self.or(rd, rs_value, rt_value),
            Xor => self.xor(rd, rs_value, rt_value),
            Nor => self.nor(rd, rs_value, rt_value),
            SetLessThan => self.slt(rd, rs_value, rt_value),
            SetLessThanUnsigned => self.sltu(rd, rs_value, rt_value),
            TrapGreaterEqual => todo!(),
            TrapGreaterEqualUnsigned => todo!(),
            TrapLessThan => todo!(),
            TrapLessThanUnsigned => todo!(),
            TrapEqual => todo!(),
            TrapNotEqual => todo!(),
        }
    }

    fn execute_immediate_format(
        &mut self,
        _opcode: Opcode,
        _instruction: Instruction,
    ) -> Result<(), Exception> {
        todo!()
    }

    fn execute_jump_format(
        &mut self,
        _opcode: Opcode,
        _instruction: Instruction,
    ) -> Result<(), Exception> {
        todo!()
    }
}

impl Interpreter {
    fn sll(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << shamt)
    }

    fn srl(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> shamt)
    }

    fn sra(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        self.registers.write_i32_to_cpu(rd, rt_value >> shamt)
    }

    fn sllv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << rs_value)
    }

    fn srlv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> rs_value)
    }

    fn srav(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        self.registers.write_i32_to_cpu(rd, rt_value >> rs_value)
    }

    fn movz(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value == 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    fn movn(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    fn mfhi(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.hi)
    }

    fn mthi(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.hi = rs_value;
        Ok(())
    }

    fn mflo(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.lo)
    }

    fn mtlo(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.lo = rs_value;
        Ok(())
    }

    fn mult(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i64;
        let rt_value = rt_value as i64;
        let product = i64::wrapping_mul(rs_value, rt_value);
        let product = unsafe { transmute::<i64, u64>(product) };
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xFFFFFFFF) as u32;
        Ok(())
    }

    fn multu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xFFFFFFFF) as u32;
        Ok(())
    }

    fn div(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = unsafe { transmute::<u32, i32>(rs_value) };
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        if rt_value != 0 {
            let quotient = i32::wrapping_div(rs_value, rt_value);
            let remainder = i32::wrapping_rem(rs_value, rt_value);
            self.registers.hi = unsafe { transmute::<i32, u32>(remainder) };
            self.registers.lo = unsafe { transmute::<i32, u32>(quotient) };
        }
        Ok(())
    }

    fn divu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.hi = u32::wrapping_rem(rs_value, rt_value);
            self.registers.lo = u32::wrapping_div(rs_value, rt_value);
        }
        Ok(())
    }

    fn add(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = unsafe { transmute::<u32, i32>(rs_value) };
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        match i32::checked_add(rs_value, rt_value) {
            Some(sum) => self.registers.write_i32_to_cpu(rd, sum),
            None => Err(Exception::IntegerOverflowOrUndeflow),
        }
    }

    fn addu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_add(rs_value, rt_value))
    }

    fn sub(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = unsafe { transmute::<u32, i32>(rs_value) };
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        match i32::checked_sub(rs_value, rt_value) {
            Some(difference) => self.registers.write_i32_to_cpu(rd, difference),
            None => Err(Exception::IntegerOverflowOrUndeflow),
        }
    }

    fn subu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_sub(rs_value, rt_value))
    }

    fn and(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value & rt_value)
    }

    fn or(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value | rt_value)
    }

    fn xor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value ^ rt_value)
    }

    fn nor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, !(rs_value | rt_value))
    }

    fn slt(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = unsafe { transmute::<u32, i32>(rs_value) };
        let rt_value = unsafe { transmute::<u32, i32>(rt_value) };
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }

    fn sltu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }
}
