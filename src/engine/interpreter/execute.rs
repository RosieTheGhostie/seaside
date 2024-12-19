use super::{
    instruction::{fields, Instruction, InstructionFormat},
    memory::regions::Region,
    Exception, Interpreter,
};
use crate::{
    config::Endian,
    constants::{fn_codes::SpecialFn, opcodes::Opcode, register},
    sign_extend::SignExtend,
};
use num_traits::FromPrimitive;

impl Interpreter {
    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Exception> {
        use InstructionFormat::*;
        let opcode = match Opcode::from_u8(fields::opcode(instruction)) {
            Some(opcode) => opcode,
            None => return Err(Exception::ReservedInstruction),
        };
        match InstructionFormat::from(opcode) {
            Register => self.execute_register_format(opcode, instruction),
            Immediate => self.execute_immediate_format(opcode, instruction),
            Jump => self.execute_jump_format(opcode, instruction),
        }
    }

    fn execute_register_format(
        &mut self,
        _opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
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
            JumpAndLinkRegister => todo!("jalr"),
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

    fn execute_immediate_format(
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
            StoreWordCoprocessor1 => self.swc1(rt, rs_value, imm),
            _ => Err(Exception::InterpreterFailure),
        }
    }

    fn execute_regimm(&mut self, rt: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
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

    fn execute_jump_format(
        &mut self,
        opcode: Opcode,
        instruction: Instruction,
    ) -> Result<(), Exception> {
        let jump_index = fields::jump_index(instruction);
        let address = (self.pc & 0xF0000000) | (jump_index << 2);
        if opcode == Opcode::JumpAndLink {
            self.link()?;
        }
        self.pc = address;
        Ok(())
    }
}

// register format instructions
impl Interpreter {
    fn sll(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << shamt)
    }

    fn movc(&mut self, rt: u8, rd: u8, rs_value: u32) -> Result<(), Exception> {
        let cc = fields::cc_from_rt(rt);
        let condition = fields::condition_from_rt(rt);
        if self.registers.read_flag_from_fpu(cc)? == condition {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    fn srl(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> shamt)
    }

    fn sra(&mut self, rd: u8, rt_value: u32, shamt: u8) -> Result<(), Exception> {
        let rt_value = rt_value as i32;
        self.registers.write_i32_to_cpu(rd, rt_value >> shamt)
    }

    fn sllv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value << rs_value)
    }

    fn srlv(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rt_value >> rs_value)
    }

    fn srav(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rt_value = rt_value as i32;
        self.registers.write_i32_to_cpu(rd, rt_value >> rs_value)
    }

    fn jr(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.pc = rs_value;
        Ok(())
    }

    fn movz(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value == 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    fn movn(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.write_u32_to_cpu(rd, rs_value)
        } else {
            Ok(())
        }
    }

    fn r#break(&mut self) -> Result<(), Exception> {
        Err(Exception::Breakpoint)
    }

    fn mfhi(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.hi)
    }

    fn mthi(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.hi = rs_value;
        Ok(())
    }

    fn mflo(&mut self, rd: u8) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, self.registers.lo)
    }

    fn mtlo(&mut self, rs_value: u32) -> Result<(), Exception> {
        self.registers.lo = rs_value;
        Ok(())
    }

    fn mult(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value: i64 = rs_value.sign_extend();
        let rt_value: i64 = rt_value.sign_extend();
        let product = i64::wrapping_mul(rs_value, rt_value) as u64;
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xFFFFFFFF) as u32;
        Ok(())
    }

    fn multu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as u64;
        let rt_value = rt_value as u64;
        let product = u64::wrapping_mul(rs_value, rt_value);
        self.registers.hi = (product >> 32) as u32;
        self.registers.lo = (product & 0xFFFFFFFF) as u32;
        Ok(())
    }

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

    fn divu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rt_value != 0 {
            self.registers.hi = u32::wrapping_rem(rs_value, rt_value);
            self.registers.lo = u32::wrapping_div(rs_value, rt_value);
        }
        Ok(())
    }

    fn add(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        match i32::checked_add(rs_value, rt_value) {
            Some(sum) => self.registers.write_i32_to_cpu(rd, sum),
            None => Err(Exception::IntegerOverflowOrUndeflow),
        }
    }

    fn addu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_add(rs_value, rt_value))
    }

    fn sub(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        match i32::checked_sub(rs_value, rt_value) {
            Some(difference) => self.registers.write_i32_to_cpu(rd, difference),
            None => Err(Exception::IntegerOverflowOrUndeflow),
        }
    }

    fn subu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, u32::wrapping_sub(rs_value, rt_value))
    }

    fn and(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value & rt_value)
    }

    fn or(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value | rt_value)
    }

    fn xor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, rs_value ^ rt_value)
    }

    fn nor(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(rd, !(rs_value | rt_value))
    }

    fn slt(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }

    fn sltu(&mut self, rd: u8, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        self.registers
            .write_u32_to_cpu(rd, if rs_value < rt_value { 1 } else { 0 })
    }

    fn tge(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        if rs_value >= rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tgeu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value >= rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tlt(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        let rs_value = rs_value as i32;
        let rt_value = rt_value as i32;
        if rs_value < rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tltu(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value < rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn teq(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value == rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }

    fn tne(&mut self, rs_value: u32, rt_value: u32) -> Result<(), Exception> {
        if rs_value != rt_value {
            Err(Exception::Trap)
        } else {
            Ok(())
        }
    }
}

// immediate format instructions
impl Interpreter {
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
            None => Err(Exception::IntegerOverflowOrUndeflow),
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
        let value = f32::from_bits(self.memory.read_u32(address, true)?);
        self.registers.write_f32_to_fpu(ft, value)
    }

    fn swc1(&mut self, ft: u8, rs_value: u32, imm: u16) -> Result<(), Exception> {
        let offset: i32 = imm.sign_extend();
        let address = u32::wrapping_add_signed(rs_value, offset);
        let ft_value = self.registers.read_f32_from_fpu(ft)?.to_bits();
        self.memory.write_u32(address, ft_value, true)
    }
}

// regimm instructions
impl Interpreter {
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

// branching utility methods (these all assume no branch delay slot btw)
impl Interpreter {
    fn branch(&mut self, offset: u16) {
        let offset = <u16 as SignExtend<i32>>::sign_extend(&offset) << 2;
        self.pc = u32::wrapping_add_signed(self.pc, offset);
    }

    fn link(&mut self) -> Result<(), Exception> {
        self.registers.write_u32_to_cpu(register::RA, self.pc)
    }
}
