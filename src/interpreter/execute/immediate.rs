use super::super::{memory::regions::Region, Exception, Interpreter};
use crate::{
    config::Endian, constants::opcodes::Opcode, disassembler::fields, sign_extend::SignExtend,
    type_aliases::Instruction,
};
use num_traits::FromPrimitive;

impl Interpreter {
    /// Executes `instruction`, which must follow the "immediate" instruction format:
    ///
    /// ```text
    /// xxxxxx xxxxx xxxxx xxxxxxxxxxxxxxxx
    /// opcode  $rs   $rt     offset/imm
    /// ```
    pub fn execute_immediate_format(
        &mut self,
        opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        use Opcode::*;
        let rs = fields::rs(instruction);
        let rt = fields::rt(instruction);
        let imm = fields::imm(instruction);
        let rs_value = self.registers.read_u32_from_cpu(rs)?;
        let rt_value = self.registers.read_u32_from_cpu(rt)?;
        match opcode {
            RegisterImmediate => self.execute_regimm(rt, rs_value, imm),
            BranchEqual => self.beq(rs_value, rt_value, imm),
            BranchNotEqual => self.bne(rs_value, rt_value, imm),
            BranchLessEqualZero => self.blez(rs_value, imm),
            BranchGreaterThanZero => self.bgtz(rs_value, imm),
            AddImmediate => self.addi(rt, rs_value, imm),
            AddImmediateUnsigned => self.addiu(rt, rs_value, imm),
            SetLessThanImmediate => self.slti(rt, rs_value, imm),
            SetLessThanImmediateUnsigned => self.sltiu(rt, rs_value, imm),
            AndImmediate => self.andi(rt, rs_value, imm),
            OrImmediate => self.ori(rt, rs_value, imm),
            XorImmediate => self.xori(rt, rs_value, imm),
            LoadUpperImmediate => self.lui(rt, imm),
            LoadByte => self.lb(rt, rs_value, imm),
            LoadHalf => self.lh(rt, rs_value, imm),
            LoadWordLeft => self.lwl(rt, rs_value, rt_value, imm),
            LoadWord => self.lw(rt, rs_value, imm),
            LoadByteUnsigned => self.lbu(rt, rs_value, imm),
            LoadHalfUnsigned => self.lhu(rt, rs_value, imm),
            LoadWordRight => self.lwr(rt, rs_value, rt_value, imm),
            StoreByte => self.sb(rs_value, rt_value, imm),
            StoreHalf => self.sh(rs_value, rt_value, imm),
            StoreWordLeft => self.swl(rs_value, rt_value, imm),
            StoreWord => self.sw(rs_value, rt_value, imm),
            StoreConditional => self.sc(rt, rs_value, rt_value, imm),
            StoreWordRight => self.swr(rs_value, rt_value, imm),
            LoadLinked => self.ll(rt, rs_value, imm),
            LoadWordCoprocessor1 => self.lwc1(rt, rs_value, imm),
            LoadDoubleCoprocessor1 => self.ldc1(rt, rs_value, imm),
            StoreWordCoprocessor1 => self.swc1(rt, rs_value, imm),
            StoreDoubleCoprocessor1 => self.sdc1(rt, rs_value, imm),
            _ => Err(Exception::InterpreterFailure),
        }
    }

    /// If `rs_value` is equal to `rt_value`, branches `offset` instructions ahead.
    fn beq(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        if rs_value == rt_value {
            self.branch(offset);
        }
        Ok(())
    }

    /// If `rs_value` is not equal to `rt_value`, branches `offset` instructions ahead.
    fn bne(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        if rs_value != rt_value {
            self.branch(offset);
        }
        Ok(())
    }

    /// If `rs_value` is non-positive, branches `offset` instructions ahead.
    fn blez(&mut self, rs_value: u32, offset: u16) -> Result<(), Exception> {
        if rs_value as i32 <= 0 {
            self.branch(offset);
        }
        Ok(())
    }

    /// If `rs_value` is strictly positive, branches `offset` instructions ahead.
    fn bgtz(&mut self, rs_value: u32, offset: u16) -> Result<(), Exception> {
        if rs_value as i32 > 0 {
            self.branch(offset);
        }
        Ok(())
    }

    /// Adds `rs_value` to the sign-extended `imm`, storing the sum in CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [integer overflow/underflow][Exception::IntegerOverflowOrUnderflow] exception if
    /// the sum cannot be represented as an unsigned 32-bit integer.
    fn addi(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        match u32::checked_add_signed(rs_value, imm) {
            Some(sum) => self.registers.write_u32_to_cpu(rt, sum),
            None => Err(Exception::IntegerOverflowOrUnderflow),
        }
    }

    /// Adds `rs_value` to the sign-extended `imm`, storing the sum in CPU register `rt`.
    fn addiu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        self.registers
            .write_u32_to_cpu(rt, u32::wrapping_add_signed(rs_value, imm))
    }

    /// If `rs_value` (interpreted as a signed integer) is less than the sign-extended `imm`, stores
    /// the value 1 in CPU register `rt`. Otherwise, stores the value 0 in `rt`.
    fn slti(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let imm: i32 = imm.sign_extend();
        self.registers
            .write_u32_to_cpu(rt, if rs_value < imm { 1 } else { 0 })
    }

    /// If `rs_value` (interpreted as an unsigned integer) is less than `imm`, stores the value 1 in
    /// CPU register `rt`. Otherwise, stores the value 0 in `rt`.
    fn sltiu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm = <u16 as SignExtend<i32>>::sign_extend(&imm) as u32;
        self.registers
            .write_u32_to_cpu(rt, if rs_value < imm { 1 } else { 0 })
    }

    /// Computes the bitwise AND of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn andi(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value & imm as u32)
    }

    /// Computes the bitwise OR of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn ori(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value | imm as u32)
    }

    /// Computes the bitwise XOR of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn xori(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value ^ imm as u32)
    }

    /// Shifts `imm` left by 16 bits, storing the result in CPU register `rt`.
    fn lui(&mut self, rt: u8, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, (imm as u32) << 16)
    }

    /// Loads the sign-extended byte stored at address `rs_value + offset.sign_extend()` into CPU
    /// register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory.
    fn lb(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u8(address)?.sign_extend();
        self.registers.write_i32_to_cpu(rt, value)
    }

    /// Loads the sign-extended halfword stored at address `rs_value + offset.sign_extend()` into
    /// CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 2 bytes.
    fn lh(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u16(address, true)?.sign_extend();
        self.registers.write_i32_to_cpu(rt, value)
    }

    fn lwl(&mut self, rt: u8, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xfffffffc;
        let shift: u32 = {
            let shift = (address % 4) << 3;
            match self.memory.endian() {
                Endian::Big => shift,
                Endian::Little => 24 - shift,
            }
        };
        let mask: u32 = !(u32::MAX << shift);
        let loaded: u32 = self.memory.read_u32(word_address, false)? << shift;
        self.registers
            .write_u32_to_cpu(rt, (rt_value & mask) | loaded)
    }

    /// Loads the word stored at address `rs_value + offset.sign_extend()` into CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn lw(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: u32 = self.memory.read_u32(address, true)?;
        self.registers.write_u32_to_cpu(rt, value)
    }

    /// Loads the byte stored at address `rs_value + offset.sign_extend()` into CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory.
    fn lbu(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u8(address)? as u32;
        self.registers.write_u32_to_cpu(rt, value)
    }

    /// Loads the halfword stored at address `rs_value + offset.sign_extend()` into CPU register
    /// `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 2 bytes.
    fn lhu(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u16(address, true)? as u32;
        self.registers.write_u32_to_cpu(rt, value)
    }

    fn lwr(&mut self, rt: u8, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xfffffffc;
        let shift: u32 = {
            let shift = (address % 4) << 3;
            match self.memory.endian() {
                Endian::Big => 24 - shift,
                Endian::Little => shift,
            }
        };
        let mask: u32 = !(u32::MAX >> shift);
        let loaded: u32 = self.memory.read_u32(word_address, false)? >> shift;
        self.registers
            .write_u32_to_cpu(rt, (rt_value & mask) | loaded)
    }

    /// Stores the least significant byte of `rt_value` at the address
    /// `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory.
    fn sb(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let byte = (rt_value & u8::MAX as u32) as u8;
        self.memory.write_u8(address, byte)
    }

    /// Stores the least significant halfword of `rt_value` at the address
    /// `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 2 bytes.
    fn sh(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let half = (rt_value & u16::MAX as u32) as u16;
        self.memory.write_u16(address, half, true)
    }

    fn swl(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xfffffffc;
        let shift: u32 = {
            let shift = (address % 4) << 3;
            match self.memory.endian() {
                Endian::Big => shift,
                Endian::Little => 24 - shift,
            }
        };
        let mask: u32 = !(u32::MAX >> shift);
        let to_store: u32 = rt_value >> shift;
        let old_value: u32 = self.memory.read_u32(word_address, true)?;
        self.memory
            .write_u32(word_address, (old_value & mask) | to_store, true)
    }

    /// Stores `rt_value` at the address `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn sw(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        self.memory.write_u32(address, rt_value, true)
    }

    /// Stores `rt_value` at the address `rs_value + offset.sign_extend()`. If the store was
    /// successful, stores the value 1 in CPU register `rt`. Otherwise, stores the value 0 in `rt`.
    ///
    /// In real MIPS processors, this instruction performs an atomic store, so there is a chance for
    /// `rt` to be set to 0; however, seaside (like most MIPS simulators out there) does not
    /// simulate multiple processors. Thus, this instruction will always succeed (assuming it
    /// doesn't raise an exception), and as such, it is impossible for `rt` to be set to anything
    /// besides 1.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn sc(&mut self, rt: u8, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        self.sw(rs_value, rt_value, offset)?;
        // always succeeds because seaside doesn't simulate multiple processors
        self.registers.write_u32_to_cpu(rt, 1)
    }

    fn swr(&mut self, rs_value: u32, rt_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xfffffffc;
        let shift: u32 = {
            let shift = (address % 4) << 3;
            match self.memory.endian() {
                Endian::Big => 24 - shift,
                Endian::Little => shift,
            }
        };
        let mask: u32 = !(u32::MAX << shift);
        let to_store: u32 = rt_value << shift;
        let old_value: u32 = self.memory.read_u32(word_address, true)?;
        self.memory
            .write_u32(word_address, (old_value & mask) | to_store, true)
    }

    /// Loads the word stored at address `rs_value + offset.sign_extend()` into CPU register `rt`.
    ///
    /// In real MIPS processors, this instruction begins an atomic read-modify-write (RMW) sequence.
    /// Seeing as how seaside (like most MIPS simulators out there) does not simulate multiple
    /// processors, however, this instruction behaves identically to `lw`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn ll(&mut self, rt: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        // identical to lw in current version of seaside
        self.lw(rt, rs_value, offset)
    }

    /// Loads the word stored at address `rs_value + offset.sign_extend()` into FPU register `ft`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn lwc1(&mut self, ft: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u32(address, true)?;
        self.registers.write_u32_to_fpu(ft, value)
    }

    /// Loads the double stored at address `rs_value + offset.sign_extend()` into FPU register `ft`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory, the address is not aligned to 8 bytes, or `ft` is not
    /// divisible by two.
    fn ldc1(&mut self, ft: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u64(address, true)?;
        self.registers.write_u64_to_fpu(ft, value)
    }

    /// Stores the value of FPU register `ft` at address `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn swc1(&mut self, ft: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read_u32_from_fpu(ft)?;
        self.memory.write_u32(address, ft_value, true)
    }

    /// Stores the value of FPU register `ft` at address `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory, the address is not aligned to 8 bytes, or `ft` is not
    /// divisible by two.
    fn sdc1(&mut self, ft: u8, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read_u64_from_fpu(ft)?;
        self.memory.write_u64(address, ft_value, true)
    }

    /// Executes an instruction following the "register immediate" instruction format:
    ///
    /// ```text
    /// 000001 xxxxx xxxxx xxxxxxxxxxxxxxxx
    /// opcode  $rs   fn         imm
    /// ```
    pub fn execute_regimm(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        use crate::constants::fn_codes::RegisterImmediateFn::{self, *};
        let r#fn = match RegisterImmediateFn::from_u8(rt) {
            Some(r#fn) => r#fn,
            None => return Err(Exception::ReservedInstruction),
        };
        match r#fn {
            BranchLessThanZero => self.bltz(rs_value, imm, false),
            BranchGreaterEqualZero => self.bgez(rs_value, imm, false),
            TrapGreaterEqualImmediate => self.tgei(rs_value, imm),
            TrapGreaterEqualImmediateUnsigned => self.tgeiu(rs_value, imm),
            TrapLessThanImmediate => self.tlti(rs_value, imm),
            TrapLessThanImmediateUnsigned => self.tltiu(rs_value, imm),
            TrapEqualImmediate => self.teqi(rs_value, imm),
            TrapNotEqualImmediate => self.tnei(rs_value, imm),
            BranchLessThanZeroAndLink => self.bltz(rs_value, imm, true),
            BranchGreaterEqualZeroAndLink => self.bgez(rs_value, imm, true),
        }
    }

    /// If `rs_value` is negative, branches `offset` instructions ahead. Also performs a link if
    /// `link` is set to `true`.
    fn bltz(&mut self, rs_value: u32, offset: u16, link: bool) -> Result<(), Exception> {
        if (rs_value as i32) < 0 {
            if link {
                self.link()?;
            }
            self.branch(offset);
        }
        Ok(())
    }

    /// If `rs_value` is non-negative, branches `offset` instructions ahead. Also performs a link if
    /// `link` is set to `true`.
    fn bgez(&mut self, rs_value: u32, offset: u16, link: bool) -> Result<(), Exception> {
        if (rs_value as i32) >= 0 {
            if link {
                self.link()?;
            }
            self.branch(offset);
        }
        Ok(())
    }

    /// If `rs_value` (interpreted as a signed integer) is greater than or equal to the
    /// sign-extended `imm`, raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tgei(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) >= imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` (interpreted as an unsigned integer) is greater than or equal to `imm`, raises
    /// a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tgeiu(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value >= (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` (interpreted as a signed integer) is less than the sign-extended `imm`, raises
    /// a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tlti(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) < imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` (interpreted as an unsigned integer) is less than `imm`, raises a
    /// [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tltiu(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value < (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is equal to the sign-extended `imm`, raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn teqi(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) == imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    /// If `rs_value` is not equal to the sign-extended `imm`, raises a [trap][Exception::Trap] exception.
    ///
    /// # Exceptions
    ///
    /// Raises a [trap][Exception::Trap] exception when the condition described above passes.
    fn tnei(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) != imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }
}
