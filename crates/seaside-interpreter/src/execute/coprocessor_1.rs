use crate::{Exception, Interpreter, InterpreterState};
use num_traits::{FromPrimitive, Zero};
use seaside_constants::{fn_codes::Coprocessor1Fn, NumberFormat};
use seaside_disassembler::fields;
use seaside_type_aliases::Instruction;

impl Interpreter {
    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format:
    ///
    /// ```text
    /// 010001 x0x0x xxxxx xxxxx xxxxx xxxxxx
    /// opcode  fmt   $ft   $fs   $fd    fn
    /// ```
    ///
    /// Some instructions specify a condition flag (`cc`). If the instruction only writes to the
    /// flag, `cc` is found in the field `$fs` as shown:
    ///
    /// ```text
    /// 010001 10x0x xxxxx xxx 00 xxxxx 0xx00x
    /// opcode  fmt   $ft  cc      $fd    fn
    /// ```
    ///
    /// If the instruction needs a boolean condition to compare with the condition flag, `cc` and
    /// the condition can be found in the field `$ft` instead:
    ///
    /// ```text
    /// 010001 10x0x xxx 0 x xxxxx xxxxx 11xxx0
    /// opcode  fmt  cc    c  $fs   $fd    fn
    /// ```
    pub fn execute_coprocessor_1(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use NumberFormat::*;
        let ft = fields::ft(instruction);
        let fs = fields::fs(instruction);
        let fd = fields::fd(instruction);
        let fmt = fields::fmt(instruction);
        if fmt == 8 {
            return self.state.bc1c(ft, instruction);
        }
        let r#fn = match Coprocessor1Fn::from_u8(fields::r#fn(instruction)) {
            Some(r#fn) => r#fn,
            None => return Err(Exception::ReservedInstruction),
        };
        match NumberFormat::from_u8(fmt) {
            Some(Single) => self.execute_coprocessor_1_single(ft, fs, fd, r#fn),
            Some(Double) => self.execute_coprocessor_1_double(ft, fs, fd, r#fn),
            Some(Word) => self.execute_coprocessor_1_word(fs, fd, r#fn),
            Some(SingleNoPrefix) if r#fn == Coprocessor1Fn::Add => self
                .state
                .mfc1(ft, self.state.registers.read_f32_from_fpu(fd)?),
            Some(WordNoPrefix) if r#fn == Coprocessor1Fn::Add => self
                .state
                .mtc1(fd, self.state.registers.read_u32_from_cpu(ft)?),
            _ => Err(Exception::ReservedInstruction),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00000`][`NumberFormat::Single`].
    fn execute_coprocessor_1_single(
        &mut self,
        ft: u8,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value = self.state.registers.read_f32_from_fpu(ft)?;
        let fs_value = self.state.registers.read_f32_from_fpu(fs)?;
        match r#fn {
            Add => self.state.add_s(fd, fs_value, ft_value),
            Subtract => self.state.sub_s(fd, fs_value, ft_value),
            Multiply => self.state.mul_s(fd, fs_value, ft_value),
            Divide => self.state.div_s(fd, fs_value, ft_value),
            SquareRoot => self.state.sqrt_s(fd, fs_value),
            AbsoluteValue => self.state.abs_s(fd, fs_value),
            Move => self.state.mov_s(fd, fs_value),
            Negate => self.state.neg_s(fd, fs_value),
            RoundWord => self.state.round_w_s(fd, fs_value),
            TruncateWord => self.state.trunc_w_s(fd, fs_value),
            CeilingWord => self.state.ceil_w_s(fd, fs_value),
            FloorWord => self.state.floor_w_s(fd, fs_value),
            MoveConditional => self.state.movc_s(fd, ft, fs_value),
            MoveZero => self.state.movz_s(fd, ft, fs_value),
            MoveNotZero => self.state.movn_s(fd, ft, fs_value),
            ConvertToSingle => Err(Exception::ReservedInstruction),
            ConvertToDouble => self.state.cvt_d_s(fd, fs_value),
            ConvertToWord => self.state.cvt_w_s(fd, fs_value),
            CompareEqual => self.state.c_eq_s(fd, fs_value, ft_value),
            CompareLessThan => self.state.c_lt_s(fd, fs_value, ft_value),
            CompareLessEqual => self.state.c_le_s(fd, fs_value, ft_value),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00001`][`NumberFormat::Double`].
    fn execute_coprocessor_1_double(
        &mut self,
        ft: u8,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value = self.state.registers.read_f64_from_fpu(ft)?;
        let fs_value = self.state.registers.read_f64_from_fpu(fs)?;
        match r#fn {
            Add => self.state.add_d(fd, fs_value, ft_value),
            Subtract => self.state.sub_d(fd, fs_value, ft_value),
            Multiply => self.state.mul_d(fd, fs_value, ft_value),
            Divide => self.state.div_d(fd, fs_value, ft_value),
            SquareRoot => self.state.sqrt_d(fd, fs_value),
            AbsoluteValue => self.state.abs_d(fd, fs_value),
            Move => self.state.mov_d(fd, fs_value),
            Negate => self.state.neg_d(fd, fs_value),
            RoundWord => self.state.round_w_d(fd, fs_value),
            TruncateWord => self.state.trunc_w_d(fd, fs_value),
            CeilingWord => self.state.ceil_w_d(fd, fs_value),
            FloorWord => self.state.floor_w_d(fd, fs_value),
            MoveConditional => self.state.movc_d(fd, ft, fs_value),
            MoveZero => self.state.movz_d(fd, ft, fs_value),
            MoveNotZero => self.state.movn_d(fd, ft, fs_value),
            ConvertToSingle => self.state.cvt_s_d(fd, fs_value),
            ConvertToDouble => Err(Exception::ReservedInstruction),
            ConvertToWord => self.state.cvt_w_d(fd, fs_value),
            CompareEqual => self.state.c_eq_d(fd, fs_value, ft_value),
            CompareLessThan => self.state.c_lt_d(fd, fs_value, ft_value),
            CompareLessEqual => self.state.c_le_d(fd, fs_value, ft_value),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00100`][`NumberFormat::Word`].
    fn execute_coprocessor_1_word(
        &mut self,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let fs_value = self.state.registers.read_i32_from_fpu(fs)?;
        match r#fn {
            ConvertToSingle => self.state.cvt_s_w(fd, fs_value),
            ConvertToDouble => self.state.cvt_d_w(fd, fs_value),
            _ => Err(Exception::ReservedInstruction),
        }
    }
}

impl InterpreterState {
    /// Adds `fs_value` and `ft_value`, storing the sum in FPU register `fd`.
    fn add_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value + ft_value)
    }

    /// Adds `fs_value` and `ft_value`, storing the sum in FPU register `fd`.
    fn add_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value + ft_value)
    }

    /// Subtracts `ft_value` from `fs_value`, storing the difference in FPU register `fd`.
    fn sub_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value - ft_value)
    }

    /// Subtracts `ft_value` from `fs_value`, storing the difference in FPU register `fd`.
    fn sub_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value - ft_value)
    }

    /// Multiplies `fs_value` and `ft_value`, storing the product in FPU register `fd`.
    fn mul_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value * ft_value)
    }

    /// Multiplies `fs_value` and `ft_value`, storing the product in FPU register `fd`.
    fn mul_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value * ft_value)
    }

    /// Divides `fs_value` by `ft_value`, storing the quotient in FPU register `fd`.
    fn div_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        if !ft_value.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value / ft_value)
        } else {
            Err(Exception::DivideByZero)
        }
    }

    /// Divides `fs_value` by `ft_value`, storing the quotient in FPU register `fd`.
    fn div_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        if !ft_value.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value / ft_value)
        } else {
            Err(Exception::DivideByZero)
        }
    }

    /// Computes the square root of `fs_value`, storing the result in FPU register `fd`.
    fn sqrt_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value.sqrt())
    }

    /// Computes the square root of `fs_value`, storing the result in FPU register `fd`.
    fn sqrt_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value.sqrt())
    }

    /// Computes the absolute value of `fs_value`, storing the result in FPU register `fd`.
    fn abs_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value.abs())
    }

    /// Computes the absolute value of `fs_value`, storing the result in FPU register `fd`.
    fn abs_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value.abs())
    }

    /// Stores `fs_value` in FPU register `fd`.
    fn mov_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value)
    }

    /// Stores `fs_value` in FPU register `fd`.
    fn mov_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value)
    }

    /// Negates `fs_value`, storing the result in FPU register `fd`.
    fn neg_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, -fs_value)
    }

    /// Negates `fs_value`, storing the result in FPU register `fd`.
    fn neg_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, -fs_value)
    }

    /// If the condition flag specified by `ft` matches the condition, branches `offset`
    /// instructions ahead (where `offset` is the lower half-word of `instruction`).
    fn bc1c(&mut self, ft: u8, instruction: Instruction) -> Result<(), Exception> {
        let cc = fields::cc_from_index(ft);
        let condition = fields::condition_from_index(ft);
        let offset = (instruction & 0xffff) as u16;
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.branch(offset);
        }
        Ok(())
    }

    /// Rounds `fs_value` to the nearest integer, storing the result in FPU register `fd`.
    ///
    /// If `fs_value` is exactly halfway between two integers, rounds away from 0.0.
    fn round_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.round() as i32)
    }

    /// Rounds `fs_value` to the nearest integer, storing the result in FPU register `fd`.
    ///
    /// If `fs_value` is exactly halfway between two integers, rounds away from 0.0.
    fn round_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.round() as i64)
    }

    /// Converts `fs_value` to an integer by discarding the fractional component, storing the result
    /// in FPU register `fd`.
    fn trunc_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.trunc() as i32)
    }

    /// Converts `fs_value` to an integer by discarding the fractional component, storing the result
    /// in FPU register `fd`.
    fn trunc_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.trunc() as i64)
    }

    /// Finds the smallest integer greater than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn ceil_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.ceil() as i32)
    }

    /// Finds the smallest integer greater than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn ceil_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.ceil() as i64)
    }

    /// Finds the largest integer less than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn floor_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.floor() as i32)
    }

    /// Finds the largest integer less than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn floor_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.floor() as i64)
    }

    /// If the condition flag specified by `ft` matches the condition, stores `fs_value` in FPU
    /// register `fd`.
    fn movc_s(&mut self, fd: u8, ft: u8, fs_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(ft);
        let condition = fields::condition_from_index(ft);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the condition flag specified by `ft` matches the condition, stores `fs_value` in FPU
    /// register `fd`.
    fn movc_d(&mut self, fd: u8, ft: u8, fs_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(ft);
        let condition = fields::condition_from_index(ft);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is zero, stores `fs_value` in FPU register `fd`.
    fn movz_s(&mut self, fd: u8, rt: u8, fs_value: f32) -> Result<(), Exception> {
        if self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is zero, stores `fs_value` in FPU register `fd`.
    fn movz_d(&mut self, fd: u8, rt: u8, fs_value: f64) -> Result<(), Exception> {
        if self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is non-zero, stores `fs_value` in FPU register `fd`.
    fn movn_s(&mut self, fd: u8, rt: u8, fs_value: f32) -> Result<(), Exception> {
        if !self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is non-zero, stores `fs_value` in FPU register `fd`.
    fn movn_d(&mut self, fd: u8, rt: u8, fs_value: f64) -> Result<(), Exception> {
        if !self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// Converts `fs_value` to a double, storing the result in FPU register `fd`.
    fn cvt_d_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value as f64)
    }

    /// Converts `fs_value` to a signed 32-bit integer, storing the result in FPU register `fd`.
    fn cvt_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_s(fd, fs_value)
    }

    /// Converts `fs_value` to a float, storing the result in FPU register `fd`.
    fn cvt_s_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value as f32)
    }

    /// Converts `fs_value` to a signed 32-bit integer, storing the result in FPU register `fd`.
    fn cvt_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_d(fd, fs_value)
    }

    /// Converts `fs_value` to a float, storing the result in FPU register `fd`.
    fn cvt_s_w(&mut self, fd: u8, fs_value: i32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value as f32)
    }

    /// Converts `fs_value` to a double, storing the result in FPU register `fd`.
    fn cvt_d_w(&mut self, fd: u8, fs_value: i32) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value as f64)
    }

    /// Checks if `fs_value` is equal to `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_eq_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value == ft_value)
    }

    /// Checks if `fs_value` is equal to `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_eq_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value == ft_value)
    }

    /// Checks if `fs_value` is less than `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_lt_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value < ft_value)
    }

    /// Checks if `fs_value` is less than `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_lt_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value < ft_value)
    }

    /// Checks if `fs_value` is less than or equal to `ft_value`, setting the FPU condition flag
    /// specified by `fd` accordingly.
    fn c_le_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value <= ft_value)
    }

    /// Checks if `fs_value` is less than or equal to `ft_value`, setting the FPU condition flag
    /// specified by `fd` accordingly.
    fn c_le_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value <= ft_value)
    }

    /// Stores `fd_value` in CPU register `rt`.
    fn mfc1(&mut self, rt: u8, fd_value: f32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, fd_value.to_bits())
    }

    /// Stores `rt_value` in FPU register `fd`.
    fn mtc1(&mut self, fd: u8, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_fpu(fd, rt_value)
    }
}
