use crate::{
    consts::{
        codes::{Coprocessor1Fn, Coprocessor1RegisterImmediateFn},
        formats::NumberFormat,
    },
    instruction::Packable,
    prelude::*,
    u5,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fields {
    Normal(NormalFields),
    RegisterImmediate(RegisterImmediateFields),
}

impl Packable for Fields {
    fn pack(self) -> Instruction {
        match self {
            Self::Normal(fields) => fields.pack(),
            Self::RegisterImmediate(fields) => fields.pack(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NormalFields {
    pub fmt: NumberFormat,
    pub ft: RegisterIndex,
    pub fs: RegisterIndex,
    pub fd: RegisterIndex,
    pub r#fn: Coprocessor1Fn,
}

impl NormalFields {
    pub const fn new(
        fmt: NumberFormat,
        ft: RegisterIndex,
        fs: RegisterIndex,
        fd: RegisterIndex,
        r#fn: Coprocessor1Fn,
    ) -> Self {
        Self {
            fmt,
            ft,
            fs,
            fd,
            r#fn,
        }
    }

    pub const fn new_template(r#fn: Coprocessor1Fn, fmt: NumberFormat) -> Self {
        Self::new(
            fmt,
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            RegisterIndex(u5::ZERO),
            r#fn,
        )
    }

    pub const fn new_with_cc(
        fmt: NumberFormat,
        ft: RegisterIndex,
        cc: ConditionCode,
        fd: RegisterIndex,
        r#fn: Coprocessor1Fn,
    ) -> Self {
        Self::new(fmt, ft, register_index_from_cc(cc), fd, r#fn)
    }

    pub const fn new_with_cc_and_condition(
        fmt: NumberFormat,
        cc: ConditionCode,
        condition: bool,
        fs: RegisterIndex,
        fd: RegisterIndex,
        r#fn: Coprocessor1Fn,
    ) -> Self {
        Self::new(
            fmt,
            register_index_from_cc_and_condition(cc, condition),
            fs,
            fd,
            r#fn,
        )
    }

    pub const fn with_cc_in_ft(mut self, cc: ConditionCode) -> Self {
        self.set_cc_in_ft(cc);
        self
    }

    pub const fn with_cc_in_fs(mut self, cc: ConditionCode) -> Self {
        self.set_cc_in_fs(cc);
        self
    }

    pub const fn with_cc_in_fd(mut self, cc: ConditionCode) -> Self {
        self.set_cc_in_fd(cc);
        self
    }

    pub const fn set_cc_in_ft(&mut self, cc: ConditionCode) {
        insert_cc_into_register_index(&mut self.ft, cc);
    }

    pub const fn set_cc_in_fs(&mut self, cc: ConditionCode) {
        insert_cc_into_register_index(&mut self.fs, cc);
    }

    pub const fn set_cc_in_fd(&mut self, cc: ConditionCode) {
        insert_cc_into_register_index(&mut self.fd, cc);
    }
}

impl Packable for NormalFields {
    fn pack(self) -> Instruction {
        ((self.fmt as Instruction) << 21)
            | ((self.ft.as_u8() as Instruction) << 16)
            | ((self.fs.as_u8() as Instruction) << 11)
            | ((self.fd.as_u8() as Instruction) << 6)
            | self.r#fn as Instruction
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RegisterImmediateFields {
    pub r#fn: Coprocessor1RegisterImmediateFn,
    pub rt: RegisterIndex,
    imm_or_shifted_fs: u16,
}

impl RegisterImmediateFields {
    pub const fn new(
        r#fn: Coprocessor1RegisterImmediateFn,
        rt: RegisterIndex,
        fs: RegisterIndex,
    ) -> Self {
        Self {
            r#fn,
            rt,
            imm_or_shifted_fs: 0x0000,
        }
        .with_fs(fs)
    }

    pub const fn new_template(r#fn: Coprocessor1RegisterImmediateFn) -> Self {
        Self::new(r#fn, RegisterIndex(u5::ZERO), RegisterIndex(u5::ZERO))
    }

    pub const fn with_fs(mut self, fs: RegisterIndex) -> Self {
        self.set_fs(fs);
        self
    }

    pub const fn with_imm(mut self, imm: u16) -> Self {
        self.set_imm(imm);
        self
    }

    pub const fn with_cc(mut self, cc: ConditionCode) -> Self {
        self.set_cc(cc);
        self
    }

    pub const fn fs(&self) -> RegisterIndex {
        RegisterIndex(u5::new_wrapped(
            (self.imm_or_shifted_fs >> Self::FS_OFFSET) as _,
        ))
    }

    pub const fn imm(&self) -> u16 {
        self.imm_or_shifted_fs
    }

    pub const fn cc(&self) -> ConditionCode {
        cc_from_register_index(self.rt)
    }

    pub const fn set_fs(&mut self, fs: RegisterIndex) {
        self.imm_or_shifted_fs = fs.0.as_u16() << Self::FS_OFFSET;
    }

    pub const fn set_imm(&mut self, imm: u16) {
        self.imm_or_shifted_fs = imm;
    }

    pub const fn set_cc(&mut self, cc: ConditionCode) {
        insert_cc_into_register_index(&mut self.rt, cc);
    }

    const FS_OFFSET: u32 = 11;
}

impl Packable for RegisterImmediateFields {
    fn pack(self) -> Instruction {
        ((self.r#fn as Instruction) << 21)
            | ((self.rt.as_u8() as Instruction) << 16)
            | self.imm_or_shifted_fs as Instruction
    }
}

pub const fn cc_from_register_index(index: RegisterIndex) -> ConditionCode {
    ConditionCode(index.0.upper_three_bits())
}

pub const fn condition_from_register_index(index: RegisterIndex) -> bool {
    index.0.least_significant_bit()
}

const fn insert_cc_into_register_index(index: &mut RegisterIndex, cc: ConditionCode) {
    index.0 = u5::or(index.0.lower_two_bits().as_u5(), cc.0.as_upper_bits_of_u5());
}

const fn register_index_from_cc(cc: ConditionCode) -> RegisterIndex {
    RegisterIndex(cc.0.as_upper_bits_of_u5())
}

const fn register_index_from_cc_and_condition(cc: ConditionCode, condition: bool) -> RegisterIndex {
    RegisterIndex(register_index_from_cc(cc).0.or_bool(condition))
}
