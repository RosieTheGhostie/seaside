use crate::{
    Exception, Interpreter, InterpreterState,
    register_file::{IndexByRegister, TryIndexByRegister},
};
use num_traits::{FromPrimitive, Zero};
use seaside_constants::{
    NumberFormat,
    fn_codes::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn},
    register::{CpuRegister, FpuRegister},
};
use seaside_disassembler::fields;
use seaside_type_aliases::Instruction;

impl Interpreter {
    /// Executes `instruction`, which must follow one of the "coprocessor 1" instruction formats:
    ///
    /// ```text
    ///   op    fmt   $ft   $fs   $fd    fn
    /// 010001 xxxxx xxxxx xxxxx xxxxx xxxxxx
    /// cop. 1
    /// ```
    ///
    /// Some instructions specify a condition flag (`cc`). If the instruction only writes to the
    /// flag, `cc` is found in the field `$fs` as shown:
    ///
    /// ```text
    ///   op    fmt   $ft   $fs  cc       fn
    /// 010001 1000x xxxxx xxxxx xxx 00 xxxxxx
    /// cop. 1
    /// ```
    ///
    /// If the instruction needs a boolean condition to compare with the condition flag, `cc` and
    /// the condition can be found in the field `$ft` instead:
    ///
    /// ```text
    ///                    tf
    ///   op    fmt  cc    ↓  $fs   $fd   fn
    /// 010001 1000x xxx 0 x xxxxx xxxxx 010001
    /// cop. 1                             ↑
    ///                                 movt.fmt
    /// ```
    ///
    /// Finally, there are also two versions for the so-called "register immediate" subclass of
    /// instructions:
    ///
    /// ```text
    ///                 nd tf
    ///   op    fn   cc  ↓ ↓      offset
    /// 010001 01000 xxx x x xxxxxxxxxxxxxxxx
    /// cop. 1  bc1
    ///
    ///   op    fn    $rt   $fs   <unused>
    /// 010001 00xxx xxxxx xxxxx 00000000000
    /// cop. 1
    /// ```
    pub fn execute_coprocessor_1(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use NumberFormat::*;
        let ft = fields::ft(instruction);
        let fs = fields::fs(instruction);
        let fd = fields::fd(instruction);
        let fmt = fields::fmt(instruction);
        match Coprocessor1RegisterImmediateFn::from_u8(fmt) {
            Some(Coprocessor1RegisterImmediateFn::MoveFromCoprocessor1) => {
                return self.state.mfc1(ft.to_cpu(), self.state.registers.read(fs));
            }
            Some(Coprocessor1RegisterImmediateFn::MoveToCoprocessor1) => {
                return self.state.mtc1(fs, self.state.registers.read(ft.to_cpu()));
            }
            Some(Coprocessor1RegisterImmediateFn::BranchCoprocessor1Flag) => {
                return self.state.bc1c(ft, instruction);
            }
            None => {}
        }
        let r#fn = match Coprocessor1Fn::from_u8(fields::r#fn(instruction)) {
            Some(r#fn) => r#fn,
            None => return Err(Exception::ReservedInstruction),
        };
        match NumberFormat::from_u8(fmt) {
            Some(Single) => self.execute_coprocessor_1_single(ft, fs, fd, r#fn),
            Some(Double) => self.execute_coprocessor_1_double(ft, fs, fd, r#fn),
            Some(Word) => self.execute_coprocessor_1_word(fs, fd, r#fn),
            _ => Err(Exception::ReservedInstruction),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00000`](NumberFormat::Single).
    fn execute_coprocessor_1_single(
        &mut self,
        ft: FpuRegister,
        fs: FpuRegister,
        fd: FpuRegister,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value: f32 = self.state.registers.read(ft);
        let fs_value: f32 = self.state.registers.read(fs);
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
            MoveZero => self.state.movz_s(fd, ft.to_cpu(), fs_value),
            MoveNotZero => self.state.movn_s(fd, ft.to_cpu(), fs_value),
            ConvertToSingle => Err(Exception::ReservedInstruction),
            ConvertToDouble => self.state.cvt_d_s(fd, fs_value),
            ConvertToWord => self.state.cvt_w_s(fd, fs_value),
            CompareEqual => self.state.c_eq_s(fd, fs_value, ft_value),
            CompareLessThan => self.state.c_lt_s(fd, fs_value, ft_value),
            CompareLessEqual => self.state.c_le_s(fd, fs_value, ft_value),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00001`](NumberFormat::Double).
    fn execute_coprocessor_1_double(
        &mut self,
        ft: FpuRegister,
        fs: FpuRegister,
        fd: FpuRegister,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value: f64 = self.state.registers.try_read(ft)?;
        let fs_value: f64 = self.state.registers.try_read(fs)?;
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
            MoveZero => self.state.movz_d(fd, ft.to_cpu(), fs_value),
            MoveNotZero => self.state.movn_d(fd, ft.to_cpu(), fs_value),
            ConvertToSingle => self.state.cvt_s_d(fd, fs_value),
            ConvertToDouble => Err(Exception::ReservedInstruction),
            ConvertToWord => self.state.cvt_w_d(fd, fs_value),
            CompareEqual => self.state.c_eq_d(fd, fs_value, ft_value),
            CompareLessThan => self.state.c_lt_d(fd, fs_value, ft_value),
            CompareLessEqual => self.state.c_le_d(fd, fs_value, ft_value),
        }
    }

    /// Executes `instruction`, which must follow the "coprocessor 1" instruction format and have
    /// the `fmt` field set to [`0b00100`](NumberFormat::Word).
    fn execute_coprocessor_1_word(
        &mut self,
        fs: FpuRegister,
        fd: FpuRegister,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let fs_value: i32 = self.state.registers.read(fs);
        match r#fn {
            ConvertToSingle => self.state.cvt_s_w(fd, fs_value),
            ConvertToDouble => self.state.cvt_d_w(fd, fs_value),
            _ => Err(Exception::ReservedInstruction),
        }
    }
}

impl InterpreterState {
    /// Adds `fs_value` and `ft_value`, storing the sum in FPU register `fd`.
    fn add_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value + ft_value);
        Ok(())
    }

    /// Adds `fs_value` and `ft_value`, storing the sum in FPU register `fd`.
    fn add_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value + ft_value)
    }

    /// Subtracts `ft_value` from `fs_value`, storing the difference in FPU register `fd`.
    fn sub_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value - ft_value);
        Ok(())
    }

    /// Subtracts `ft_value` from `fs_value`, storing the difference in FPU register `fd`.
    fn sub_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value - ft_value)
    }

    /// Multiplies `fs_value` and `ft_value`, storing the product in FPU register `fd`.
    fn mul_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value * ft_value);
        Ok(())
    }

    /// Multiplies `fs_value` and `ft_value`, storing the product in FPU register `fd`.
    fn mul_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value * ft_value)
    }

    /// Divides `fs_value` by `ft_value`, storing the quotient in FPU register `fd`.
    fn div_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value / ft_value);
        Ok(())
    }

    /// Divides `fs_value` by `ft_value`, storing the quotient in FPU register `fd`.
    fn div_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value / ft_value)
    }

    /// Computes the square root of `fs_value`, storing the result in FPU register `fd`.
    fn sqrt_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.sqrt());
        Ok(())
    }

    /// Computes the square root of `fs_value`, storing the result in FPU register `fd`.
    fn sqrt_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.sqrt())
    }

    /// Computes the absolute value of `fs_value`, storing the result in FPU register `fd`.
    fn abs_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.abs());
        Ok(())
    }

    /// Computes the absolute value of `fs_value`, storing the result in FPU register `fd`.
    fn abs_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.abs())
    }

    /// Stores `fs_value` in FPU register `fd`.
    fn mov_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value);
        Ok(())
    }

    /// Stores `fs_value` in FPU register `fd`.
    fn mov_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value)
    }

    /// Negates `fs_value`, storing the result in FPU register `fd`.
    fn neg_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, -fs_value);
        Ok(())
    }

    /// Negates `fs_value`, storing the result in FPU register `fd`.
    fn neg_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, -fs_value)
    }

    /// If the condition flag specified by `ft` matches the condition, branches `offset`
    /// instructions ahead (where `offset` is the lower half-word of `instruction`).
    fn bc1c(&mut self, ft: FpuRegister, instruction: Instruction) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(ft);
        let condition = fields::condition_from_fpu_register(ft);
        let offset = (instruction & 0xffff) as u16;
        if self.registers.read_fpu_flag(cc) == condition {
            self.branch(offset);
        }
        Ok(())
    }

    /// Rounds `fs_value` to the nearest integer, storing the result in FPU register `fd`.
    ///
    /// If `fs_value` is exactly halfway between two integers, rounds away from 0.0.
    fn round_w_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.round() as i32);
        Ok(())
    }

    /// Rounds `fs_value` to the nearest integer, storing the result in FPU register `fd`.
    ///
    /// If `fs_value` is exactly halfway between two integers, rounds away from 0.0.
    fn round_w_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.round() as i64)
    }

    /// Converts `fs_value` to an integer by discarding the fractional component, storing the result
    /// in FPU register `fd`.
    fn trunc_w_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.trunc() as i32);
        Ok(())
    }

    /// Converts `fs_value` to an integer by discarding the fractional component, storing the result
    /// in FPU register `fd`.
    fn trunc_w_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.trunc() as i64)
    }

    /// Finds the smallest integer greater than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn ceil_w_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.ceil() as i32);
        Ok(())
    }

    /// Finds the smallest integer greater than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn ceil_w_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.ceil() as i64)
    }

    /// Finds the largest integer less than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn floor_w_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value.floor() as i32);
        Ok(())
    }

    /// Finds the largest integer less than or equal to `fs_value`, storing the result in FPU
    /// register `fd`.
    fn floor_w_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value.floor() as i64)
    }

    /// If the condition flag specified by `ft` matches the condition, stores `fs_value` in FPU
    /// register `fd`.
    fn movc_s(&mut self, fd: FpuRegister, ft: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(ft);
        let condition = fields::condition_from_fpu_register(ft);
        if self.registers.read_fpu_flag(cc) == condition {
            self.registers.write(fd, fs_value);
        }
        Ok(())
    }

    /// If the condition flag specified by `ft` matches the condition, stores `fs_value` in FPU
    /// register `fd`.
    fn movc_d(&mut self, fd: FpuRegister, ft: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(ft);
        let condition = fields::condition_from_fpu_register(ft);
        if self.registers.read_fpu_flag(cc) == condition {
            self.registers.try_write(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is zero, stores `fs_value` in FPU register `fd`.
    fn movz_s(&mut self, fd: FpuRegister, rt: CpuRegister, fs_value: f32) -> Result<(), Exception> {
        let rt_value: u32 = self.registers.read(rt);
        if rt_value.is_zero() {
            self.registers.write(fd, fs_value);
        }
        Ok(())
    }

    /// If the value of CPU register `rt` is zero, stores `fs_value` in FPU register `fd`.
    fn movz_d(&mut self, fd: FpuRegister, rt: CpuRegister, fs_value: f64) -> Result<(), Exception> {
        let rt_value: u32 = self.registers.read(rt);
        if rt_value.is_zero() {
            self.registers.try_write(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// If the value of CPU register `rt` is non-zero, stores `fs_value` in FPU register `fd`.
    fn movn_s(&mut self, fd: FpuRegister, rt: CpuRegister, fs_value: f32) -> Result<(), Exception> {
        let rt_value: u32 = self.registers.read(rt);
        if !rt_value.is_zero() {
            self.registers.write(fd, fs_value);
        }
        Ok(())
    }

    /// If the value of CPU register `rt` is non-zero, stores `fs_value` in FPU register `fd`.
    fn movn_d(&mut self, fd: FpuRegister, rt: CpuRegister, fs_value: f64) -> Result<(), Exception> {
        let rt_value: u32 = self.registers.read(rt);
        if !rt_value.is_zero() {
            self.registers.try_write(fd, fs_value)
        } else {
            Ok(())
        }
    }

    /// Converts `fs_value` to a double, storing the result in FPU register `fd`.
    fn cvt_d_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value as f64)
    }

    /// Converts `fs_value` to a signed 32-bit integer, storing the result in FPU register `fd`.
    fn cvt_w_s(&mut self, fd: FpuRegister, fs_value: f32) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_s(fd, fs_value)
    }

    /// Converts `fs_value` to a float, storing the result in FPU register `fd`.
    fn cvt_s_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        self.registers.write(fd, fs_value as f32);
        Ok(())
    }

    /// Converts `fs_value` to a signed 32-bit integer, storing the result in FPU register `fd`.
    fn cvt_w_d(&mut self, fd: FpuRegister, fs_value: f64) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_d(fd, fs_value)
    }

    /// Converts `fs_value` to a float, storing the result in FPU register `fd`.
    fn cvt_s_w(&mut self, fd: FpuRegister, fs_value: i32) -> Result<(), Exception> {
        self.registers.write(fd, fs_value as f32);
        Ok(())
    }

    /// Converts `fs_value` to a double, storing the result in FPU register `fd`.
    fn cvt_d_w(&mut self, fd: FpuRegister, fs_value: i32) -> Result<(), Exception> {
        self.registers.try_write(fd, fs_value as f64)
    }

    /// Checks if `fs_value` is equal to `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_eq_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value == ft_value);
        Ok(())
    }

    /// Checks if `fs_value` is equal to `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_eq_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value == ft_value);
        Ok(())
    }

    /// Checks if `fs_value` is less than `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_lt_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value < ft_value);
        Ok(())
    }

    /// Checks if `fs_value` is less than `ft_value`, setting the FPU condition flag specified by
    /// `fd` accordingly.
    fn c_lt_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value < ft_value);
        Ok(())
    }

    /// Checks if `fs_value` is less than or equal to `ft_value`, setting the FPU condition flag
    /// specified by `fd` accordingly.
    fn c_le_s(&mut self, fd: FpuRegister, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value <= ft_value);
        Ok(())
    }

    /// Checks if `fs_value` is less than or equal to `ft_value`, setting the FPU condition flag
    /// specified by `fd` accordingly.
    fn c_le_d(&mut self, fd: FpuRegister, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_fpu_register(fd);
        self.registers.write_fpu_flag(cc, fs_value <= ft_value);
        Ok(())
    }

    /// Stores `fs_value` in CPU register `rt`.
    fn mfc1(&mut self, rt: CpuRegister, fs_value: f32) -> Result<(), Exception> {
        self.registers.write(rt, fs_value.to_bits());
        Ok(())
    }

    /// Stores `rt_value` in FPU register `fs`.
    fn mtc1(&mut self, fs: FpuRegister, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(fs, rt_value);
        Ok(())
    }
}
