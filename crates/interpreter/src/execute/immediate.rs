use crate::{
    Exception, Interpreter, InterpreterState,
    memory::Region,
    register_file::{IndexByRegister, TryIndexByRegister},
};
use num_traits::FromPrimitive;
use seaside_constants::{
    Opcode,
    register::{CpuRegister, FpuRegister},
};
use seaside_disassembler::fields;
use seaside_int_utils::{Endian, SignExtend};
use seaside_type_aliases::Instruction;

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
        let rs_value: u32 = self.state.registers.read(rs);
        let rt_value: u32 = self.state.registers.read(rt);
        match opcode {
            RegisterImmediate => self.execute_regimm(rt, rs_value, imm),
            BranchEqual => self.state.beq(rs_value, rt_value, imm),
            BranchNotEqual => self.state.bne(rs_value, rt_value, imm),
            BranchLessEqualZero => self.state.blez(rs_value, imm),
            BranchGreaterThanZero => self.state.bgtz(rs_value, imm),
            AddImmediate => self.state.addi(rt, rs_value, imm),
            AddImmediateUnsigned => self.state.addiu(rt, rs_value, imm),
            SetLessThanImmediate => self.state.slti(rt, rs_value, imm),
            SetLessThanImmediateUnsigned => self.state.sltiu(rt, rs_value, imm),
            AndImmediate => self.state.andi(rt, rs_value, imm),
            OrImmediate => self.state.ori(rt, rs_value, imm),
            XorImmediate => self.state.xori(rt, rs_value, imm),
            LoadUpperImmediate => self.state.lui(rt, imm),
            LoadByte => self.state.lb(rt, rs_value, imm),
            LoadHalf => self.state.lh(rt, rs_value, imm),
            LoadWordLeft => self.state.lwl(rt, rs_value, rt_value, imm),
            LoadWord => self.state.lw(rt, rs_value, imm),
            LoadByteUnsigned => self.state.lbu(rt, rs_value, imm),
            LoadHalfUnsigned => self.state.lhu(rt, rs_value, imm),
            LoadWordRight => self.state.lwr(rt, rs_value, rt_value, imm),
            StoreByte => self.state.sb(rs_value, rt_value, imm),
            StoreHalf => self.state.sh(rs_value, rt_value, imm),
            StoreWordLeft => self.state.swl(rs_value, rt_value, imm),
            StoreWord => self.state.sw(rs_value, rt_value, imm),
            StoreConditional => self.state.sc(rt, rs_value, rt_value, imm),
            StoreWordRight => self.state.swr(rs_value, rt_value, imm),
            LoadLinked => self.state.ll(rt, rs_value, imm),
            LoadWordCoprocessor1 => self.state.lwc1(rt.to_fpu(), rs_value, imm),
            LoadDoubleCoprocessor1 => self.state.ldc1(rt.to_fpu(), rs_value, imm),
            StoreWordCoprocessor1 => self.state.swc1(rt.to_fpu(), rs_value, imm),
            StoreDoubleCoprocessor1 => self.state.sdc1(rt.to_fpu(), rs_value, imm),
            _ => Err(Exception::InterpreterFailure),
        }
    }

    /// Executes an instruction following the "register immediate" instruction format:
    ///
    /// ```text
    /// 000001 xxxxx xxxxx xxxxxxxxxxxxxxxx
    /// opcode  $rs   fn         imm
    /// ```
    pub fn execute_regimm(
        &mut self,
        rt: CpuRegister,
        rs_value: u32,
        imm: u16,
    ) -> Result<(), Exception> {
        use seaside_constants::fn_codes::RegisterImmediateFn::{self, *};
        let r#fn = RegisterImmediateFn::from_u8(rt as u8).ok_or(Exception::ReservedInstruction)?;
        match r#fn {
            BranchLessThanZero => self.state.bltz(rs_value, imm, false),
            BranchGreaterEqualZero => self.state.bgez(rs_value, imm, false),
            TrapGreaterEqualImmediate => self.state.tgei(rs_value, imm),
            TrapGreaterEqualImmediateUnsigned => self.state.tgeiu(rs_value, imm),
            TrapLessThanImmediate => self.state.tlti(rs_value, imm),
            TrapLessThanImmediateUnsigned => self.state.tltiu(rs_value, imm),
            TrapEqualImmediate => self.state.teqi(rs_value, imm),
            TrapNotEqualImmediate => self.state.tnei(rs_value, imm),
            BranchLessThanZeroAndLink => self.state.bltz(rs_value, imm, true),
            BranchGreaterEqualZeroAndLink => self.state.bgez(rs_value, imm, true),
        }
    }
}

impl InterpreterState {
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
    fn addi(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if let Some(sum) = u32::checked_add_signed(rs_value, imm) {
            self.registers.write(rt, sum);
            Ok(())
        } else {
            Err(Exception::IntegerOverflowOrUnderflow)
        }
    }

    /// Adds `rs_value` to the sign-extended `imm`, storing the sum in CPU register `rt`.
    fn addiu(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        self.registers
            .write(rt, u32::wrapping_add_signed(rs_value, imm));
        Ok(())
    }

    /// If `rs_value` (interpreted as a signed integer) is less than the sign-extended `imm`, stores
    /// the value 1 in CPU register `rt`. Otherwise, stores the value 0 in `rt`.
    fn slti(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let imm: i32 = imm.sign_extend();
        self.registers
            .write(rt, if rs_value < imm { 1u32 } else { 0u32 });
        Ok(())
    }

    /// If `rs_value` (interpreted as an unsigned integer) is less than `imm`, stores the value 1 in
    /// CPU register `rt`. Otherwise, stores the value 0 in `rt`.
    fn sltiu(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm = <u16 as SignExtend<i32>>::sign_extend(&imm) as u32;
        self.registers
            .write(rt, if rs_value < imm { 1u32 } else { 0u32 });
        Ok(())
    }

    /// Computes the bitwise AND of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn andi(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write(rt, rs_value & imm as u32);
        Ok(())
    }

    /// Computes the bitwise OR of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn ori(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write(rt, rs_value | imm as u32);
        Ok(())
    }

    /// Computes the bitwise XOR of `rs_value` and `imm`, storing the result in CPU register `rt`.
    fn xori(&mut self, rt: CpuRegister, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write(rt, rs_value ^ imm as u32);
        Ok(())
    }

    /// Shifts `imm` left by 16 bits, storing the result in CPU register `rt`.
    fn lui(&mut self, rt: CpuRegister, imm: u16) -> Result<(), Exception> {
        self.registers.write(rt, (imm as u32) << 16);
        Ok(())
    }

    /// Loads the sign-extended byte stored at address `rs_value + offset.sign_extend()` into CPU
    /// register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory.
    fn lb(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u8(address)?.sign_extend();
        self.registers.write(rt, value);
        Ok(())
    }

    /// Loads the sign-extended halfword stored at address `rs_value + offset.sign_extend()` into
    /// CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 2 bytes.
    fn lh(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u16(address, true)?.sign_extend();
        self.registers.write(rt, value);
        Ok(())
    }

    fn lwl(
        &mut self,
        rt: CpuRegister,
        rs_value: u32,
        rt_value: u32,
        offset: u16,
    ) -> Result<(), Exception> {
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
        self.registers.write(rt, (rt_value & mask) | loaded);
        Ok(())
    }

    /// Loads the word stored at address `rs_value + offset.sign_extend()` into CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn lw(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: u32 = self.memory.read_u32(address, true)?;
        self.registers.write(rt, value);
        Ok(())
    }

    /// Loads the byte stored at address `rs_value + offset.sign_extend()` into CPU register `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory.
    fn lbu(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u8(address)? as u32;
        self.registers.write(rt, value);
        Ok(())
    }

    /// Loads the halfword stored at address `rs_value + offset.sign_extend()` into CPU register
    /// `rt`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 2 bytes.
    fn lhu(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u16(address, true)? as u32;
        self.registers.write(rt, value);
        Ok(())
    }

    fn lwr(
        &mut self,
        rt: CpuRegister,
        rs_value: u32,
        rt_value: u32,
        offset: u16,
    ) -> Result<(), Exception> {
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
        self.registers.write(rt, (rt_value & mask) | loaded);
        Ok(())
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
    fn sc(
        &mut self,
        rt: CpuRegister,
        rs_value: u32,
        rt_value: u32,
        offset: u16,
    ) -> Result<(), Exception> {
        self.sw(rs_value, rt_value, offset)?;
        // always succeeds because seaside doesn't simulate multiple processors
        self.registers.write(rt, 1u32);
        Ok(())
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
    fn ll(&mut self, rt: CpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        // identical to lw in current version of seaside
        self.lw(rt, rs_value, offset)
    }

    /// Loads the word stored at address `rs_value + offset.sign_extend()` into FPU register `ft`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn lwc1(&mut self, ft: FpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u32(address, true)?;
        self.registers.write(ft, value);
        Ok(())
    }

    /// Loads the double stored at address `rs_value + offset.sign_extend()` into FPU register `ft`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid load][Exception::InvalidLoad] exception if the computed address points
    /// to currently inaccessible memory, the address is not aligned to 8 bytes, or `ft` is not
    /// divisible by two.
    fn ldc1(&mut self, ft: FpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u64(address, true)?;
        self.registers.try_write(ft, value)
    }

    /// Stores the value of FPU register `ft` at address `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory or if the address is not aligned to 4 bytes.
    fn swc1(&mut self, ft: FpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read(ft);
        self.memory.write_u32(address, ft_value, true)
    }

    /// Stores the value of FPU register `ft` at address `rs_value + offset.sign_extend()`.
    ///
    /// # Exceptions
    ///
    /// Raises an [invalid store][Exception::InvalidStore] exception if the computed address points
    /// to currently inaccessible memory, the address is not aligned to 8 bytes, or `ft` is not
    /// divisible by two.
    fn sdc1(&mut self, ft: FpuRegister, rs_value: u32, offset: u16) -> Result<(), Exception> {
        let offset: i32 = offset.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.try_read(ft)?;
        self.memory.write_u64(address, ft_value, true)
    }

    /// If `rs_value` is negative, branches `offset` instructions ahead. Also performs a link if
    /// `link` is set to `true`.
    fn bltz(&mut self, rs_value: u32, offset: u16, link: bool) -> Result<(), Exception> {
        if (rs_value as i32) < 0 {
            if link {
                self.link();
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
                self.link();
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
