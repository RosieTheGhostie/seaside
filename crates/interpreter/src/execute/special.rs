use num_traits::FromPrimitive;
use seaside_core::{consts::codes::SpecialFn, prelude::*};
use seaside_disassembler::fields;

use crate::{Exception, Interpreter, InterpreterState, math, register_file::IndexByRegister};

impl Interpreter {
    /// Executes `instruction`, which must follow the "special" instruction format:
    ///
    /// ```text
    /// 000000 xxxxx xxxxx xxxxx xxxxx xxxxxx
    /// opcode  $rs   $rt   $rd  shamt   fn
    /// ```
    pub fn execute_special(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use SpecialFn::*;
        let rs = fields::rs(instruction);
        let rt = fields::rt(instruction);
        let rd = fields::rd(instruction);
        let rs_value: u32 = self.state.registers.read(rs);
        let rt_value: u32 = self.state.registers.read(rt);
        let shamt = fields::shamt(instruction);
        let r#fn =
            SpecialFn::from_u8(fields::r#fn(instruction)).ok_or(Exception::ReservedInstruction)?;
        match r#fn {
            ShiftLeftLogical => self.state.sll(rd, rt_value, shamt),
            MoveConditional => self.state.movc(rt, rd, rs_value),
            ShiftRightLogical => self.state.srl(rd, rt_value, shamt),
            ShiftRightArithmetic => self.state.sra(rd, rt_value, shamt),
            ShiftLeftLogicalVariable => self.state.sllv(rd, rs_value, rt_value),
            ShiftRightLogicalVariable => self.state.srlv(rd, rs_value, rt_value),
            ShiftRightArithmeticVariable => self.state.srav(rd, rs_value, rt_value),
            JumpRegister => self.state.jr(rs_value),
            JumpAndLinkRegister => self.state.jalr(rd, rs_value),
            MoveZero => self.state.movz(rd, rs_value, rt_value),
            MoveNotZero => self.state.movn(rd, rs_value, rt_value),
            SystemCall => self.syscall(),
            Break => self.state.r#break(),
            MoveFromHigh => self.state.mfhi(rd),
            MoveToHigh => self.state.mthi(rs_value),
            MoveFromLow => self.state.mflo(rd),
            MoveToLow => self.state.mtlo(rs_value),
            Multiply => self.state.mult(rs_value, rt_value),
            MultiplyUnsigned => self.state.multu(rs_value, rt_value),
            Divide => self.state.div(rs_value, rt_value),
            DivideUnsigned => self.state.divu(rs_value, rt_value),
            Add => self.state.add(rd, rs_value, rt_value),
            AddUnsigned => self.state.addu(rd, rs_value, rt_value),
            Subtract => self.state.sub(rd, rs_value, rt_value),
            SubtractUnsigned => self.state.subu(rd, rs_value, rt_value),
            And => self.state.and(rd, rs_value, rt_value),
            Or => self.state.or(rd, rs_value, rt_value),
            Xor => self.state.xor(rd, rs_value, rt_value),
            Nor => self.state.nor(rd, rs_value, rt_value),
            SetLessThan => self.state.slt(rd, rs_value, rt_value),
            SetLessThanUnsigned => self.state.sltu(rd, rs_value, rt_value),
            TrapGreaterEqual => self.state.tge(rs_value, rt_value),
            TrapGreaterEqualUnsigned => self.state.tgeu(rs_value, rt_value),
            TrapLessThan => self.state.tlt(rs_value, rt_value),
            TrapLessThanUnsigned => self.state.tltu(rs_value, rt_value),
            TrapEqual => self.state.teq(rs_value, rt_value),
            TrapNotEqual => self.state.tne(rs_value, rt_value),
            _ => todo!(),
        }
    }
}

impl InterpreterState {
    /// Shifts `rt_value` left by `shamt` bits and stores the result in CPU register `rd`.
    fn sll(&mut self, rd: CpuRegister, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_left(rt_value, shamt as _));
        Ok(())
    }

    /// If the condition flag specified in the field `rt` matches the condition, stores `rs_value`
    /// in CPU register `rd`.
    fn movc(&mut self, rt: CpuRegister, rd: CpuRegister, rs_value: u32) -> Result<(), Exception> {
        let cc = fields::cc_from_cpu_register(rt);
        let condition = fields::condition_from_cpu_register(rt);
        if self.registers.read_fpu_flag(cc) == condition {
            self.registers.write(rd, rs_value);
        }

        Ok(())
    }

    /// Shifts `rt_value` right by `shamt` bits and stores the result in CPU register `rd`.
    fn srl(&mut self, rd: CpuRegister, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_right(rt_value, shamt as _));
        Ok(())
    }

    /// Shifts `rt_value` right by `shamt` bits (copying the most significant bit of `rt_value` to
    /// fill the space) and stores the result in CPU register `rd`.
    fn sra(&mut self, rd: CpuRegister, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_right_arithmetic(rt_value, shamt as _));
        Ok(())
    }

    /// Shifts `rt_value` left by `rs_value` bits and stores the result in CPU register `rd`.
    fn sllv(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_left(rt_value, rs_value));
        Ok(())
    }

    /// Shifts `rt_value` right by `rs_value` bits and stores the result in CPU register `rd`.
    fn srlv(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_right(rt_value, rs_value));
        Ok(())
    }

    /// Shifts `rt_value` right by `rs_value` bits (copying the most significant bit of `rt_value`
    /// to fill the space) and stores the result in CPU register `rd`.
    fn srav(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write(rd, math::u32::shift_right_arithmetic(rt_value, rs_value));
        Ok(())
    }

    /// Sets the program counter (PC) to `rs_value`.
    fn jr(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.pc = rs_value;
        Ok(())
    }

    /// Stores the current program counter (PC) in CPU register `rd`, then sets PC to `rs_value`.
    fn jalr(&mut self, rd: CpuRegister, rs_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, self.pc);
        self.pc = rs_value;

        Ok(())
    }

    /// If `rt_value` is zero, stores `rs_value` in CPU register `rd`.
    fn movz(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value == 0 {
            self.registers.write(rd, rs_value);
        }

        Ok(())
    }

    /// If `rt_value` is nonzero, stores `rs_value` in CPU register `rd`.
    fn movn(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.write(rd, rs_value);
        }

        Ok(())
    }

    /// Raises a [break][Exception::Break] exception.
    fn r#break(&mut self) -> Result<(), Exception> {
        Err(Exception::Break)
    }

    /// Stores the value of register `hi` in CPU register `rd`.
    fn mfhi(&mut self, rd: CpuRegister) -> Result<(), Exception> {
        self.registers.write(rd, self.registers.hi);
        Ok(())
    }

    /// Stores `rs_value` in register `hi`.
    fn mthi(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.hi = rs_value;
        Ok(())
    }

    /// Stores the value of register `lo` in CPU register `rd`.
    fn mflo(&mut self, rd: CpuRegister) -> Result<(), Exception> {
        self.registers.write(rd, self.registers.lo);
        Ok(())
    }

    /// Stores `rs_value` in register `lo`.
    fn mtlo(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.lo = rs_value;
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, storing the most significant word
    /// of the product in register `hi` and the least significant word in register `lo`.
    fn mult(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let product = math::i32::mul(rs_value as _, rt_value as _);
        self.registers.hi = product.upper_half as _;
        self.registers.lo = product.lower_half as _;

        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, storing the most significant word
    /// of the product in register `hi` and the least significant word in register `lo`.
    fn multu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let product = math::u32::mul(rs_value, rt_value);
        self.registers.hi = product.upper_half;
        self.registers.lo = product.lower_half;

        Ok(())
    }

    /// Divides `rs_value` by `rt_value` as signed integers, storing the quotient in register `lo`
    /// and the remainder in register `hi`.
    fn div(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if let Some(math::Division {
            quotient,
            remainder,
        }) = math::i32::divmod(rs_value as _, rt_value as _)
        {
            self.registers.hi = remainder as _;
            self.registers.lo = quotient as _;
        }

        Ok(())
    }

    /// Divides `rs_value` by `rt_value` as unsigned integers, storing the quotient in register `lo`
    /// and the remainder in register `hi`.
    fn divu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if let Some(math::Division {
            quotient,
            remainder,
        }) = math::u32::divmod(rs_value, rt_value)
        {
            self.registers.hi = remainder;
            self.registers.lo = quotient;
        }

        Ok(())
    }

    /// Adds `rs_value` and `rt_value` together, storing the sum in CPU register `rd`.
    ///
    /// # Exceptions
    ///
    /// Raises an [integer overflow/underflow][Exception::IntegerOverflowOrUnderflow] exception if
    /// the sum cannot be represented as a signed 32-bit integer.
    fn add(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(
            rd,
            math::i32::add_with_overflow(rs_value as _, rt_value as _)?,
        );
        Ok(())
    }

    /// Adds `rs_value` and `rt_value` together, storing the sum in CPU register `rd`.
    fn addu(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::add(rs_value, rt_value));
        Ok(())
    }

    /// Subtracts `rt_value` from `rs_value`, storing the difference in CPU register `rd`.
    ///
    /// # Exceptions
    ///
    /// Raises an [integer overflow/underflow][Exception::IntegerOverflowOrUnderflow] exception if
    /// the sum cannot be represented as a signed 32-bit integer.
    fn sub(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(
            rd,
            math::i32::sub_with_underflow(rs_value as _, rt_value as _)?,
        );
        Ok(())
    }

    /// Subtracts `rt_value` from `rs_value`, storing the difference in CPU register `rd`.
    fn subu(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::sub(rs_value, rt_value));
        Ok(())
    }

    /// Computes the bitwise AND of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn and(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::and(rs_value, rt_value));
        Ok(())
    }

    /// Computes the bitwise OR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn or(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::or(rs_value, rt_value));
        Ok(())
    }

    /// Computes the bitwise XOR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn xor(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::xor(rs_value, rt_value));
        Ok(())
    }

    /// Computes the bitwise NOR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn nor(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, math::u32::nor(rs_value, rt_value));
        Ok(())
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as signed integers), stores the
    /// value 1 in CPU register `rd`. Otherwise, stores the value 0 in `rd`.
    fn slt(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write(rd, ((rs_value as i32) < rt_value as i32) as u32);
        Ok(())
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as unsigned integers), stores the
    /// value 1 in CPU register `rd`. Otherwise, stores the value 0 in `rd`.
    fn sltu(&mut self, rd: CpuRegister, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write(rd, (rs_value < rt_value) as u32);
        Ok(())
    }

    /// If `rs_value` is greater than or equal to `rt_value` (both interpreted as signed integers),
    /// raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tge(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value as i32 >= rt_value as i32 {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is greater than or equal to `rt_value` (both interpreted as unsigned
    /// integers), raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tgeu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value >= rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as signed integers), raises a
    /// [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tlt(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if (rs_value as i32) < rt_value as i32 {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as unsigned integers), raises a
    /// [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tltu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value < rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is equal to `rt_value`, raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn teq(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value == rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is not equal to `rt_value`, raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tne(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value != rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }
}
