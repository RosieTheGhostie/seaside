use crate::{Exception, Interpreter, InterpreterState, register_file::IndexByRegister};
use num_traits::FromPrimitive;
use seaside_constants::{fn_codes::Special2Fn, register::CpuRegister};
use seaside_disassembler::fields;
use seaside_int_utils::SignExtend;
use seaside_type_aliases::Instruction;

impl Interpreter {
    /// Executes `instruction`, which must follow the "special 2" instruction format:
    ///
    /// ```text
    /// 011100 xxxxx xxxxx xxxxx 00000 xxxxxx
    /// opcode  $rs   $rt   $rd   n/a    fn
    /// ```
    pub fn execute_special_2(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use Special2Fn::*;
        let rs = fields::rs(instruction);
        let rt = fields::rt(instruction);
        let rd = fields::rd(instruction);
        let rs_value: u32 = self.state.registers.read(rs);
        let rt_value: u32 = self.state.registers.read(rt);
        let r#fn = match Special2Fn::from_u8(fields::r#fn(instruction)) {
            Some(fn_code) => fn_code,
            None => return Err(Exception::ReservedInstruction),
        };
        match r#fn {
            MultiplyAdd => self.state.madd(rt_value, rs_value),
            MultiplyAddUnsigned => self.state.maddu(rt_value, rs_value),
            Multiply => self.state.mul(rd, rs_value, rt_value),
            MultiplySubtract => self.state.msub(rt_value, rs_value),
            MultiplySubtractUnsigned => self.state.msubu(rt_value, rs_value),
            CountLeadingZeroes => self.state.clz(rd, rs_value),
            CountLeadingOnes => self.state.clo(rd, rs_value),
        }
    }
}

impl InterpreterState {
    /// Multiplies `rs_value` and `rt_value` as signed integers, adding the most significant word
    /// of the product to register `hi` and the least significant word to register `lo`.
    fn madd(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = u32::wrapping_add(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_add(self.registers.lo, (product & 0xffffffff) as u32);
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, adding the most significant word
    /// of the product to register `hi` and the least significant word to register `lo`.
    fn maddu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = u32::wrapping_add(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_add(self.registers.lo, (product & 0xffffffff) as u32);
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, storing the least significant word
    /// of the product in CPU register `rd` and discarding the most significant word.
    fn mul(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        self.registers
            .write(rd, i32::wrapping_mul(rs_value, rt_value));
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, subtracting the most significant
    /// word of the product from register `hi` and the least significant word from register `lo`.
    fn msub(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = u32::wrapping_sub(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_sub(self.registers.lo, (product & 0xffffffff) as u32);
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, subtracting the most significant
    /// word of the product from register `hi` and the least significant word from register `lo`.
    fn msubu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = u32::wrapping_sub(self.registers.hi, (product >> 32) as u32);
        self.registers.lo = u32::wrapping_sub(self.registers.lo, (product & 0xffffffff) as u32);
        Ok(())
    }

    /// Counts the number of leading zeroes in `rs_value` and stores the result in CPU register
    /// `rd`.
    fn clz(&mut self, rd: CpuRegister, rs_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, rs_value.leading_zeros());
        Ok(())
    }

    /// Counts the number of leading ones in `rs_value` and stores the result in CPU register `rd`.
    fn clo(&mut self, rd: CpuRegister, rs_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, rs_value.leading_ones());
        Ok(())
    }
}
