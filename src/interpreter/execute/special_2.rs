use super::super::{
    instruction::{fields, Instruction},
    Exception, Interpreter,
};
use crate::{constants::fn_codes::Special2Fn, sign_extend::SignExtend};
use num_traits::FromPrimitive;

impl Interpreter {
    pub fn execute_special_2(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use Special2Fn::*;
        let rs = fields::rs(instruction);
        let rt = fields::rt(instruction);
        let rd = fields::rd(instruction);
        let rs_value = self.registers.read_u32_from_cpu(rs)?;
        let rt_value = self.registers.read_u32_from_cpu(rt)?;
        let r#fn = match Special2Fn::from_u8(fields::r#fn(instruction)) {
            Some(fn_code) => fn_code,
            None => return Err(Exception::ReservedInstruction),
        };
        match r#fn {
            MultiplyAdd => self.madd(rt_value, rs_value),
            MultiplyAddUnsigned => self.maddu(rt_value, rs_value),
            Multiply => self.mul(rd, rs_value, rt_value),
            MultiplySubtract => self.msub(rt_value, rs_value),
            MultiplySubtractUnsigned => self.msubu(rt_value, rs_value),
            CountLeadingZeroes => self.clz(rd, rs_value),
            CountLeadingOnes => self.clo(rd, rs_value),
        }
    }

    fn madd(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = u32::wrapping_add(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_add(self.registers.lo, (product & 0xFFFFFFFF) as u32);
        Ok(())
    }

    fn maddu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = u32::wrapping_add(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_add(self.registers.lo, (product & 0xFFFFFFFF) as u32);
        Ok(())
    }

    fn mul(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        self.registers
            .write_i32_to_cpu(rd, i32::wrapping_mul(rs_value, rt_value))
    }

    fn msub(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = u32::wrapping_sub(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_sub(self.registers.lo, (product & 0xFFFFFFFF) as u32);
        Ok(())
    }

    fn msubu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = u32::wrapping_sub(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_sub(self.registers.lo, (product & 0xFFFFFFFF) as u32);
        Ok(())
    }

    fn clz(&mut self, rd: u8, rs_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, rs_value.leading_zeros())
    }

    fn clo(&mut self, rd: u8, rs_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value.leading_ones())
    }
}
