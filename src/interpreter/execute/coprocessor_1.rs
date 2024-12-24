use super::super::{
    instruction::{fields, Instruction},
    Exception, Interpreter,
};
use crate::constants::{fn_codes::Coprocessor1Fn, number_fmt::NumberFormat};
use num_traits::{FromPrimitive, Zero};

impl Interpreter {
    pub fn execute_coprocessor_1(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use NumberFormat::*;
        let ft = fields::ft(instruction);
        let fs = fields::fs(instruction);
        let fd = fields::fd(instruction);
        let r#fn = match Coprocessor1Fn::from_u8(fields::r#fn(instruction)) {
            Some(r#fn) => r#fn,
            None => return Err(Exception::ReservedInstruction),
        };
        match NumberFormat::from_u8(fields::fmt(instruction)) {
            Some(Single) => self.execute_coprocessor_1_single(ft, fs, fd, r#fn),
            Some(Double) => self.execute_coprocessor_1_double(ft, fs, fd, r#fn),
            Some(Word) => self.execute_coprocessor_1_word(fs, fd, r#fn),
            None => Err(Exception::ReservedInstruction),
        }
    }

    fn execute_coprocessor_1_single(
        &mut self,
        ft: u8,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value = self.registers.read_f32_from_fpu(ft)?;
        let fs_value = self.registers.read_f32_from_fpu(fs)?;
        match r#fn {
            Add => self.add_s(fd, fs_value, ft_value),
            Subtract => self.sub_s(fd, fs_value, ft_value),
            Multiply => self.mul_s(fd, fs_value, ft_value),
            Divide => self.div_s(fd, fs_value, ft_value),
            SquareRoot => self.sqrt_s(fd, fs_value),
            AbsoluteValue => self.abs_s(fd, fs_value),
            Move => self.mov_s(fd, fs_value),
            Negate => self.neg_s(fd, fs_value),
            RoundWord => self.round_w_s(fd, fs_value),
            TruncateWord => self.trunc_w_s(fd, fs_value),
            CeilingWord => self.ceil_w_s(fd, fs_value),
            FloorWord => self.floor_w_s(fd, fs_value),
            MoveConditional => self.movc_s(fd, ft, fs_value),
            MoveZero => self.movz_s(fd, ft, fs_value),
            MoveNotZero => self.movn_s(fd, ft, fs_value),
            ConvertToSingle => Err(Exception::ReservedInstruction),
            ConvertToDouble => self.cvt_d_s(fd, fs_value),
            ConvertToWord => self.cvt_w_s(fd, fs_value),
            CompareEqual => self.c_eq_s(fd, fs_value, ft_value),
            CompareLessThan => self.c_lt_s(fd, fs_value, ft_value),
            CompareLessEqual => self.c_le_s(fd, fs_value, ft_value),
        }
    }

    fn execute_coprocessor_1_double(
        &mut self,
        ft: u8,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let ft_value = self.registers.read_f64_from_fpu(ft)?;
        let fs_value = self.registers.read_f64_from_fpu(fs)?;
        match r#fn {
            Add => self.add_d(fd, fs_value, ft_value),
            Subtract => self.sub_d(fd, fs_value, ft_value),
            Multiply => self.mul_d(fd, fs_value, ft_value),
            Divide => self.div_d(fd, fs_value, ft_value),
            SquareRoot => self.sqrt_d(fd, fs_value),
            AbsoluteValue => self.abs_d(fd, fs_value),
            Move => self.mov_d(fd, fs_value),
            Negate => self.neg_d(fd, fs_value),
            RoundWord => self.round_w_d(fd, fs_value),
            TruncateWord => self.trunc_w_d(fd, fs_value),
            CeilingWord => self.ceil_w_d(fd, fs_value),
            FloorWord => self.floor_w_d(fd, fs_value),
            MoveConditional => self.movc_d(fd, ft, fs_value),
            MoveZero => self.movz_d(fd, ft, fs_value),
            MoveNotZero => self.movn_d(fd, ft, fs_value),
            ConvertToSingle => self.cvt_s_d(fd, fs_value),
            ConvertToDouble => Err(Exception::ReservedInstruction),
            ConvertToWord => self.cvt_w_d(fd, fs_value),
            CompareEqual => self.c_eq_d(fd, fs_value, ft_value),
            CompareLessThan => self.c_lt_d(fd, fs_value, ft_value),
            CompareLessEqual => self.c_le_d(fd, fs_value, ft_value),
        }
    }

    fn execute_coprocessor_1_word(
        &mut self,
        fs: u8,
        fd: u8,
        r#fn: Coprocessor1Fn,
    ) -> Result<(), Exception> {
        use Coprocessor1Fn::*;
        let fs_value = self.registers.read_i32_from_fpu(fs)?;
        match r#fn {
            ConvertToSingle => self.cvt_s_w(fd, fs_value),
            ConvertToDouble => self.cvt_d_w(fd, fs_value),
            _ => Err(Exception::ReservedInstruction),
        }
    }

    fn add_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value + ft_value)
    }

    fn add_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value + ft_value)
    }

    fn sub_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value - ft_value)
    }

    fn sub_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value - ft_value)
    }

    fn mul_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value * ft_value)
    }

    fn mul_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value * ft_value)
    }

    fn div_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        if !ft_value.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value / ft_value)
        } else {
            Err(Exception::DivideByZero)
        }
    }

    fn div_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        if !ft_value.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value / ft_value)
        } else {
            Err(Exception::DivideByZero)
        }
    }

    fn sqrt_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value.sqrt())
    }

    fn sqrt_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value.sqrt())
    }

    fn abs_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value.abs())
    }

    fn abs_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value.abs())
    }

    fn mov_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value)
    }

    fn mov_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value)
    }

    fn neg_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, -fs_value)
    }

    fn neg_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, -fs_value)
    }

    fn round_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.round() as i32)
    }

    fn round_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.round() as i64)
    }

    fn trunc_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.trunc() as i32)
    }

    fn trunc_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.trunc() as i64)
    }

    fn ceil_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.ceil() as i32)
    }

    fn ceil_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.ceil() as i64)
    }

    fn floor_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_i32_to_fpu(fd, fs_value.floor() as i32)
    }

    fn floor_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_i64_to_fpu(fd, fs_value.floor() as i64)
    }

    fn movc_s(&mut self, fd: u8, ft: u8, fs_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(ft);
        let condition = fields::condition_from_index(ft);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn movc_d(&mut self, fd: u8, ft: u8, fs_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(ft);
        let condition = fields::condition_from_index(ft);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn movz_s(&mut self, fd: u8, rt: u8, fs_value: f32) -> Result<(), Exception> {
        if self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn movz_d(&mut self, fd: u8, rt: u8, fs_value: f64) -> Result<(), Exception> {
        if self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn movn_s(&mut self, fd: u8, rt: u8, fs_value: f32) -> Result<(), Exception> {
        if !self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f32_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn movn_d(&mut self, fd: u8, rt: u8, fs_value: f64) -> Result<(), Exception> {
        if !self.registers.read_u32_from_cpu(rt)?.is_zero() {
            self.registers.write_f64_to_fpu(fd, fs_value)
        } else {
            Ok(())
        }
    }

    fn cvt_d_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value as f64)
    }

    fn cvt_w_s(&mut self, fd: u8, fs_value: f32) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_s(fd, fs_value)
    }

    fn cvt_s_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value as f32)
    }

    fn cvt_w_d(&mut self, fd: u8, fs_value: f64) -> Result<(), Exception> {
        // The manual I'm referencing mentions something calling FCSR (which supposedly would
        // influence what kind of rounding is used here), but I have no idea what they're talking
        // about.
        self.round_w_d(fd, fs_value)
    }

    fn cvt_s_w(&mut self, fd: u8, fs_value: i32) -> Result<(), Exception> {
        self.registers.write_f32_to_fpu(fd, fs_value as f32)
    }

    fn cvt_d_w(&mut self, fd: u8, fs_value: i32) -> Result<(), Exception> {
        self.registers.write_f64_to_fpu(fd, fs_value as f64)
    }

    fn c_eq_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value == ft_value)
    }

    fn c_eq_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value == ft_value)
    }

    fn c_lt_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value < ft_value)
    }

    fn c_lt_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value < ft_value)
    }

    fn c_le_s(&mut self, fd: u8, fs_value: f32, ft_value: f32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value <= ft_value)
    }

    fn c_le_d(&mut self, fd: u8, fs_value: f64, ft_value: f64) -> Result<(), Exception> {
        let cc = fields::cc_from_index(fd);
        self.registers.write_flag_to_fpu(cc, fs_value <= ft_value)
    }
}
