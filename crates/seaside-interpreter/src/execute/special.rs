use super::super::{Exception, Interpreter};
use num_traits::FromPrimitive;
use seaside_constants::fn_codes::SpecialFn;
use seaside_disassembler::fields;
use seaside_int_utils::SignExtend;
use seaside_type_aliases::Instruction;

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
        let rs_value = self.registers.read_u32_from_cpu(rs)?;
        let rt_value = self.registers.read_u32_from_cpu(rt)?;
        let shamt = fields::shamt(instruction);
        let r#fn = match SpecialFn::from_u8(fields::r#fn(instruction)) {
            Some(fn_code) => fn_code,
            None => return Err(Exception::ReservedInstruction),
        };
        match r#fn {
            ShiftLeftLogical => self.sll(rd, rt_value, shamt),
            MoveConditional => self.movc(rt, rd, rs_value),
            ShiftRightLogical => self.srl(rd, rt_value, shamt),
            ShiftRightArithmetic => self.sra(rd, rt_value, shamt),
            ShiftLeftLogicalVariable => self.sllv(rd, rs_value, rt_value),
            ShiftRightLogicalVariable => self.srlv(rd, rs_value, rt_value),
            ShiftRightArithmeticVariable => self.srav(rd, rs_value, rt_value),
            JumpRegister => self.jr(rs_value),
            JumpAndLinkRegister => self.jalr(rd, rs_value),
            MoveZero => self.movz(rd, rs_value, rt_value),
            MoveNotZero => self.movn(rd, rs_value, rt_value),
            SystemCall => self.syscall(),
            Break => self.r#break(),
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
            TrapGreaterEqual => self.tge(rs_value, rt_value),
            TrapGreaterEqualUnsigned => self.tgeu(rs_value, rt_value),
            TrapLessThan => self.tlt(rs_value, rt_value),
            TrapLessThanUnsigned => self.tltu(rs_value, rt_value),
            TrapEqual => self.teq(rs_value, rt_value),
            TrapNotEqual => self.tne(rs_value, rt_value),
        }
    }

    /// Shifts `rt_value` left by `shamt` bits and stores the result in CPU register `rd`.
    fn sll(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << shamt)
    }

    /// If the condition flag specified in the field `rt` matches the condition, stores `rs_value`
    /// in CPU register `rd`.
    fn movc(&mut self, rt: u8, rd: u8, rs_value: u32) -> Result<(), Exception> {
        let cc = fields::cc_from_index(rt);
        let condition = fields::condition_from_index(rt);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    /// Shifts `rt_value` right by `shamt` bits and stores the result in CPU register `rd`.
    fn srl(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> shamt)
    }

    /// Shifts `rt_value` right by `shamt` bits (copying the most significant bit of `rt_value` to
    /// fill the space) and stores the result in CPU register `rd`.
    fn sra(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        let rt_value = rt_value as i32;
        self.registers.write_i32_to_cpu(rd, rt_value >> shamt)
    }

    /// Shifts `rt_value` left by `rs_value` bits and stores the result in CPU register `rd`.
    fn sllv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << rs_value)
    }

    /// Shifts `rt_value` right by `rs_value` bits and stores the result in CPU register `rd`.
    fn srlv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> rs_value)
    }

    /// Shifts `rt_value` right by `rs_value` bits (copying the most significant bit of `rt_value`
    /// to fill the space) and stores the result in CPU register `rd`.
    fn srav(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rt_value = rt_value as i32;
        self.registers.write_i32_to_cpu(rd, rt_value >> rs_value)
    }

    /// Sets the program counter (PC) to `rs_value`.
    fn jr(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.pc = rs_value;
        Ok(())
    }

    /// Stores the current program counter (PC) in CPU register `rd`, then sets PC to `rs_value`.
    fn jalr(&mut self, rd: u8, rs_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.pc)?;
        self.pc = rs_value;
        Ok(())
    }

    /// If `rt_value` is zero, stores `rs_value` in CPU register `rd`.
    fn movz(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value == 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    /// If `rt_value` is nonzero, stores `rs_value` in CPU register `rd`.
    fn movn(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    /// Raises a [break][Exception::Break] exception.
    fn r#break(&mut self) -> Result<(), Exception> {
        Err(Exception::Break)
    }

    /// Stores the value of register `hi` in CPU register `rd`.
    fn mfhi(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.hi)
    }

    /// Stores `rs_value` in register `hi`.
    fn mthi(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.hi = rs_value;
        Ok(())
    }

    /// Stores the value of register `lo` in CPU register `rd`.
    fn mflo(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.lo)
    }

    /// Stores `rs_value` in register `lo`.
    fn mtlo(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.lo = rs_value;
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as signed integers, storing the most significant word
    /// of the product in register `hi` and the least significant word in register `lo`.
    fn mult(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xffffffff) as u32;
        Ok(())
    }

    /// Multiplies `rs_value` and `rt_value` as unsigned integers, storing the most significant word
    /// of the product in register `hi` and the least significant word in register `lo`.
    fn multu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xffffffff) as u32;
        Ok(())
    }

    /// Divides `rs_value` by `rt_value` as signed integers, storing the quotient in register `lo`
    /// and the remainder in register `hi`.
    fn div(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        if rt_value != 0 {
            let quotient = i32::wrapping_div(rs_value, rt_value) as u32;
            let remainder = i32::wrapping_rem(rs_value, rt_value) as u32;
            self.registers.hi = remainder;
            self.registers.lo = quotient;
        }
        Ok(())
    }

    /// Divides `rs_value` by `rt_value` as unsigned integers, storing the quotient in register `lo`
    /// and the remainder in register `hi`.
    fn divu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.hi = u32::wrapping_rem(rs_value, rt_value);
            self.registers.lo = u32::wrapping_div(rs_value, rt_value);
        }
        Ok(())
    }

    /// Adds `rs_value` and `rt_value` together, storing the sum in CPU register `rd`.
    ///
    /// # Exceptions
    ///
    /// Raises an [integer overflow/underflow][Exception::IntegerOverflowOrUnderflow] exception if
    /// the sum cannot be represented as a signed 32-bit integer.
    fn add(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        match i32::checked_add(rs_value, rt_value) {
            Some(sum) => self.registers.write_i32_to_cpu(rd, sum),
            None => Err(Exception::IntegerOverflowOrUnderflow),
        }
    }

    /// Adds `rs_value` and `rt_value` together, storing the sum in CPU register `rd`.
    fn addu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_add(rs_value, rt_value))
    }

    /// Subtracts `rt_value` from `rs_value`, storing the difference in CPU register `rd`.
    ///
    /// # Exceptions
    ///
    /// Raises an [integer overflow/underflow][Exception::IntegerOverflowOrUnderflow] exception if
    /// the sum cannot be represented as a signed 32-bit integer.
    fn sub(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        match i32::checked_sub(rs_value, rt_value) {
            Some(difference) => self.registers.write_i32_to_cpu(rd, difference),
            None => Err(Exception::IntegerOverflowOrUnderflow),
        }
    }

    /// Subtracts `rt_value` from `rs_value`, storing the difference in CPU register `rd`.
    fn subu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_sub(rs_value, rt_value))
    }

    /// Computes the bitwise AND of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn and(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value & rt_value)
    }

    /// Computes the bitwise OR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn or(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value | rt_value)
    }

    /// Computes the bitwise XOR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn xor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value ^ rt_value)
    }

    /// Computes the bitwise NOR of `rs_value` and `rt_value`, storing the result in CPU register
    /// `rd`.
    fn nor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, !(rs_value | rt_value))
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as signed integers), stores the
    /// value 1 in CPU register `rd`. Otherwise, stores the value 0 in `rd`.
    fn slt(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }

    /// If `rs_value` is less than `rt_value` (both interpreted as unsigned integers), stores the
    /// value 1 in CPU register `rd`. Otherwise, stores the value 0 in `rd`.
    fn sltu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }

    /// If `rs_value` is greater than or equal to `rt_value` (both interpreted as signed integers),
    /// raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tge(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        if rs_value >= rt_value {
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
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        if rs_value < rt_value {
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
