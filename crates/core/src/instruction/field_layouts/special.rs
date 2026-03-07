use crate::{consts::codes::SpecialFn, instruction::Packable, prelude::*, u5};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fields {
    pub rs: RegisterIndex,
    pub rt: RegisterIndex,
    pub rd: RegisterIndex,
    pub shamt: u5,
    pub r#fn: SpecialFn,
}

impl Fields {
    pub const fn new(
        rs: RegisterIndex,
        rt: RegisterIndex,
        rd: RegisterIndex,
        shamt: u5,
        r#fn: SpecialFn,
    ) -> Self {
        Self {
            rs,
            rt,
            rd,
            shamt,
            r#fn,
        }
    }

    pub const fn new_template(r#fn: SpecialFn) -> Self {
        Self::new(
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            u5::ZERO,
            r#fn,
        )
    }

    pub const fn code(&self) -> u32 {
        (self.rs.0.as_u32() << 15)
            | (self.rt.0.as_u32() << 10)
            | (self.rd.0.as_u32() << 5)
            | self.shamt.as_u32()
    }

    pub const fn with_cc(mut self, cc: ConditionCode) -> Self {
        self.set_cc(cc);
        self
    }

    pub const fn set_cc(&mut self, cc: ConditionCode) {
        self.rt.0 = self.rt.0.and_u8(0b00011).or(cc.0.as_upper_bits_of_u5());
    }

    pub const fn set_code(&mut self, code: u32) {
        self.rs.0 = u5::MAX.and_u32(code >> 15);
        self.rt.0 = u5::MAX.and_u32(code >> 10);
        self.rd.0 = u5::MAX.and_u32(code >> 5);
        self.shamt = u5::MAX.and_u32(code);
    }
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        ((self.rs.as_u8() as Instruction) << 21)
            | ((self.rt.as_u8() as Instruction) << 16)
            | ((self.rd.as_u8() as Instruction) << 11)
            | ((self.shamt.as_u8() as Instruction) << 6)
            | self.r#fn as Instruction
    }
}
