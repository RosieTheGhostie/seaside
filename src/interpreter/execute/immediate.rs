use super::super::{
    instruction::{fields, Instruction},
    memory::regions::Region,
    Exception, Interpreter,
};
use crate::{config::Endian, constants::opcodes::Opcode, sign_extend::SignExtend};
use num_traits::FromPrimitive;

impl Interpreter {
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
            StoreDoubleCoprocessor1 => self.swc1(rt, rs_value, imm),
            _ => Err(Exception::InterpreterFailure),
        }
    }

    fn beq(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value == rt_value {
            self.branch(imm);
        }
        Ok(())
    }

    fn bne(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value != rt_value {
            self.branch(imm);
        }
        Ok(())
    }

    fn bgtz(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value as i32 > 0 {
            self.branch(imm);
        }
        Ok(())
    }

    fn addi(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        match u32::checked_add_signed(rs_value, imm) {
            Some(sum) => self.registers.write_u32_to_cpu(rt, sum),
            None => Err(Exception::IntegerOverflowOrUnderflow),
        }
    }

    fn addiu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        self.registers
            .write_u32_to_cpu(rt, u32::wrapping_add_signed(rs_value, imm))
    }

    fn slti(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let imm: i32 = imm.sign_extend();
        self.registers
            .write_u32_to_cpu(rt, if rs_value < imm { 1 } else { 0 })
    }

    fn sltiu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm = <u16 as SignExtend<i32>>::sign_extend(&imm) as u32;
        self.registers
            .write_u32_to_cpu(rt, if rs_value < imm { 1 } else { 0 })
    }

    fn andi(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value & imm as u32)
    }

    fn ori(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value | imm as u32)
    }

    fn xori(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, rs_value ^ imm as u32)
    }

    fn lui(&mut self, rt: u8, imm: u16) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rt, (imm as u32) << 16)
    }

    fn lb(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u8(address)?.sign_extend();
        self.registers.write_i32_to_cpu(rt, value)
    }

    fn lh(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: i32 = self.memory.read_u16(address, true)?.sign_extend();
        self.registers.write_i32_to_cpu(rt, value)
    }

    fn lwl(&mut self, rt: u8, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xFFFFFFFC;
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

    fn lw(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value: u32 = self.memory.read_u32(address, true)?;
        self.registers.write_u32_to_cpu(rt, value)
    }

    fn lbu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u8(address)? as u32;
        self.registers.write_u32_to_cpu(rt, value)
    }

    fn lhu(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u16(address, true)? as u32;
        self.registers.write_u32_to_cpu(rt, value)
    }

    fn lwr(&mut self, rt: u8, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xFFFFFFFC;
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

    fn sb(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let byte = (rt_value & u8::MAX as u32) as u8;
        self.memory.write_u8(address, byte)
    }

    fn sh(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let half = (rt_value & u16::MAX as u32) as u16;
        self.memory.write_u16(address, half, true)
    }

    fn swl(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xFFFFFFFC;
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

    fn sw(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        self.memory.write_u32(address, rt_value, true)
    }

    fn sc(&mut self, rt: u8, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        self.sw(rs_value, rt_value, imm)?;
        // always succeeds because seaside doesn't simulate multiple processors
        self.registers.write_u32_to_cpu(rt, 1)
    }

    fn swr(&mut self, rs_value: u32, rt_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let word_address = address & 0xFFFFFFFC;
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

    fn ll(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        // identical to lw in current version of seaside
        self.lw(rt, rs_value, imm)
    }

    fn lwc1(&mut self, ft: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u32(address, true)?;
        self.registers.write_u32_to_fpu(ft, value)
    }

    fn ldc1(&mut self, ft: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let value = self.memory.read_u64(address, true)?;
        self.registers.write_u64_to_fpu(ft, value)
    }

    fn swc1(&mut self, ft: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read_u32_from_fpu(ft)?;
        self.memory.write_u32(address, ft_value, true)
    }

    fn sdc1(&mut self, ft: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read_u64_from_fpu(ft)?;
        self.memory.write_u64(address, ft_value, true)
    }

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

    fn bltz(&mut self, rs_value: u32, imm: u16, link: bool) -> Result<(), Exception> {
        if (rs_value as i32) < 0 {
            if link {
                self.link()?;
            }
            self.branch(imm);
        }
        Ok(())
    }

    fn bgez(&mut self, rs_value: u32, imm: u16, link: bool) -> Result<(), Exception> {
        if (rs_value as i32) >= 0 {
            if link {
                self.link()?;
            }
            self.branch(imm);
        }
        Ok(())
    }

    fn tgei(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) >= imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tgeiu(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value >= (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tlti(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let imm: i32 = imm.sign_extend();
        if (rs_value as i32) < imm {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tltiu(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value < (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn teqi(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value == (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tnei(&mut self, rs_value: u32, imm: u16) -> Result<(), Exception> {
        if rs_value != (imm as u32) {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }
}
