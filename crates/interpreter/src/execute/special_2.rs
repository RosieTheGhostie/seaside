use num_traits::FromPrimitive;
use seaside_constants::{fn_codes::Special2Fn, register::CpuRegister};
use seaside_disassembler::fields;
use seaside_type_aliases::Instruction;

use crate::{Exception, Interpreter, InterpreterState, math, register_file::IndexByRegister};

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
        let r#fn =
            Special2Fn::from_u8(fields::r#fn(instruction)).ok_or(Exception::ReservedInstruction)?;
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
        let product = math::i32::mul(rs_value as _, rt_value as _);
        self.registers.hi = math::u32::add(self.registers.hi, product.upper_half as _);
        self.registers.lo = math::u32::add(self.registers.lo, product.lower_half as _);

        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, adding the most significant word
    /// of the product to register `hi` and the least significant word to register `lo`.
    fn maddu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let product = math::u32::mul(rs_value, rt_value);
        self.registers.hi = math::u32::add(self.registers.hi, product.upper_half);
        self.registers.lo = math::u32::add(self.registers.lo, product.lower_half);

        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, storing the least significant word
    /// of the product in CPU register `rd` and discarding the most significant word.
    fn mul(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(
            rd,
            math::i32::mul_with_truncation(rs_value as _, rt_value as _),
        );
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, subtracting the most significant
    /// word of the product from register `hi` and the least significant word from register `lo`.
    fn msub(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let product = math::i32::mul(rs_value as _, rt_value as _);
        self.registers.hi = math::u32::sub(self.registers.hi, product.upper_half as _);
        self.registers.lo = math::u32::sub(self.registers.lo, product.lower_half as _);

        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, subtracting the most significant
    /// word of the product from register `hi` and the least significant word from register `lo`.
    fn msubu(&mut self, rt_value: u32, rs_value: u32) -> Result<(), Exception> {
        let product = math::u32::mul(rs_value, rt_value);
        self.registers.hi = math::u32::sub(self.registers.hi, product.upper_half);
        self.registers.lo = math::u32::sub(self.registers.lo, product.lower_half);

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
