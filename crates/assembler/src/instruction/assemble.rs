use seaside_constants::{
    ConditionCode, NumberFormat,
    fn_codes::Coprocessor1RegisterImmediateFn,
    register::{Coprocessor0Register, CpuRegister, FpuRegister},
};
use seaside_type_aliases::Instruction;

/// Inserts value(s) into field(s) with the corresponding number of bits.
macro_rules! insert {
    [$({$n_bits:literal} $field:expr),* $(,)? => $machine_code:ident] => {
        $($machine_code = ($machine_code << $n_bits) | $field as Instruction;)*
    };
    [$({$n_bits:literal} $field:expr),* $(,)? => &$machine_code:ident] => {
        $(*$machine_code = (*$machine_code << $n_bits) | $field as Instruction;)*
    };
}
pub(super) use insert;

pub const fn r_type(
    machine_code: &mut Instruction,
    rs: CpuRegister,
    rt: CpuRegister,
    rd: CpuRegister,
    shamt: u8,
    fn_code: u8,
) {
    insert![{5} rs, {5} rt, {5} rd, {5} shamt, {6} fn_code => &machine_code];
}

pub const fn movc(
    machine_code: &mut Instruction,
    rs: CpuRegister,
    cc: ConditionCode,
    condition: bool,
    rd: CpuRegister,
    fn_code: u8,
) {
    insert![{5} rs, {3} cc, {2} condition, {5} rd, {11} fn_code => &machine_code];
}

pub const fn coprocessor_1(
    machine_code: &mut Instruction,
    fmt: NumberFormat,
    ft: FpuRegister,
    fs: FpuRegister,
    fd: FpuRegister,
    fn_code: u8,
) {
    insert![{5} fmt, {5} ft, {5} fs, {5} fd, {6} fn_code => &machine_code];
}

pub const fn coprocessor_1_with_cc_c(
    machine_code: &mut Instruction,
    fmt: NumberFormat,
    cc: ConditionCode,
    condition: bool,
    fd: FpuRegister,
    fs: FpuRegister,
    fn_code: u8,
) {
    insert![{5} fmt, {3} cc, {2} condition, {5} fs, {5} fd, {6} fn_code => &machine_code];
}

pub const fn coprocessor_1_register_immediate(
    machine_code: &mut Instruction,
    fn_code: u8,
    rt: CpuRegister,
    fs: FpuRegister,
) {
    insert![{5} fn_code, {5} rt, {5} fs, {11} 0 => &machine_code];
}

pub const fn bc1c(machine_code: &mut Instruction, cc: ConditionCode, condition: bool, offset: u16) {
    insert![
        {5} Coprocessor1RegisterImmediateFn::BranchCoprocessor1Flag,
        {3} cc,
        {2} condition,
        {16} offset => &machine_code
    ];
}

pub const fn regimm(machine_code: &mut Instruction, rs: CpuRegister, fn_code: u8, imm: u16) {
    insert![{5} rs, {5} fn_code, {16} imm => &machine_code];
}

pub const fn i_type(machine_code: &mut Instruction, rs: CpuRegister, rt: CpuRegister, imm: u16) {
    insert![{5} rs, {5} rt, {16} imm => &machine_code];
}

pub const fn j_type(machine_code: &mut Instruction, jump_index: u32) {
    insert!({26} jump_index => &machine_code);
}

pub const fn coprocessor_0(
    machine_code: &mut Instruction,
    fn_code: u8,
    rt: CpuRegister,
    rd: Coprocessor0Register,
) {
    insert![{5} fn_code, {5} rt, {5} rd, {11} 0 => &machine_code];
}
