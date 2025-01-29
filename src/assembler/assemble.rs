use super::{
    error::Error,
    macros::assemble_field,
    operation::{
        macros::{coprocessor_0, coprocessor_1, special, special_2},
        BasicOperator, Operand,
    },
};
use crate::constants::{number_fmt::NumberFormat, opcodes::Opcode};

pub fn assemble_instruction(
    operator: BasicOperator,
    operands: [Option<Operand>; 3],
) -> Result<u32, Error> {
    use BasicOperator::*;
    use Operand::*;
    let mut machine_code: u32 = 0;
    let opcode = Opcode::from(operator);
    let fn_code = operator.op_or_fn_code();
    assemble_field!(opcode as u8; (6 bits) -> machine_code);
    match operator {
        // sll $rd, $rt, shamt
        special![ShiftLeftLogical, ShiftRightLogical, ShiftRightArithmetic] => match operands {
            [Some(Register(rd)), Some(Register(rt)), Some(Shamt(shamt))] => assemble_r_type(
                &mut machine_code,
                None,
                Some(rt),
                Some(rd),
                Some(shamt),
                fn_code,
            ),
            _ => return Err(Error::InternalLogicIssue),
        },
        // movt $rd, $rs, cc
        special!(MoveConditional, Some(condition)) => match operands {
            [Some(Register(rd)), Some(Register(rs)), Some(Cc(cc))] => {
                assemble_movc(&mut machine_code, rs, cc, condition, rd, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // sllv $rd, $rt, $rs
        special![
            ShiftLeftLogicalVariable,
            ShiftRightLogicalVariable,
            ShiftRightArithmeticVariable,
        ] => match operands {
            [Some(Register(rd)), Some(Register(rt)), Some(Register(rs))] => assemble_r_type(
                &mut machine_code,
                Some(rs),
                Some(rt),
                Some(rd),
                None,
                fn_code,
            ),
            _ => return Err(Error::InternalLogicIssue),
        },
        // jr $rs
        // jalr $rd, $rs
        special![JumpRegister, JumpAndLinkRegister, MoveToHigh, MoveToLow] => match operands {
            [Some(Register(rs)), None, None] => {
                assemble_r_type(&mut machine_code, Some(rs), None, None, None, fn_code);
            }
            [Some(Register(rd)), Some(Register(rs)), None] => {
                assemble_r_type(&mut machine_code, Some(rs), None, Some(rd), None, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // syscall
        special!(SystemCall) => match operands {
            [None, None, None] => {
                assemble_r_type(&mut machine_code, None, None, None, None, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // break
        // break code
        special!(Break) => match operands {
            // break
            [None, None, None] => {
                assemble_field!(0 (20 bits) -> machine_code);
                assemble_field!(fn_code (6 bits) -> machine_code);
            }
            // break code
            [Some(Code(code)), None, None] => {
                assemble_field!(code (20 bits) -> machine_code);
                assemble_field!(fn_code (6 bits) -> machine_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // mfhi $rd
        special![MoveFromHigh, MoveFromLow] => match operands {
            [Some(Register(rd)), None, None] => {
                assemble_r_type(&mut machine_code, None, None, Some(rd), None, fn_code)
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // mult $rs, $rt
        special![
            Multiply,
            MultiplyUnsigned,
            Divide,
            DivideUnsigned,
            TrapGreaterEqual,
            TrapGreaterEqualUnsigned,
            TrapLessThan,
            TrapLessThanUnsigned,
            TrapEqual,
            TrapNotEqual,
        ]
        | special_2![
            MultiplyAdd,
            MultiplyAddUnsigned,
            MultiplySubtract,
            MultiplySubtractUnsigned
        ] => match operands {
            [Some(Register(rs)), Some(Register(rt)), None] => {
                assemble_r_type(&mut machine_code, Some(rs), Some(rt), None, None, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // movz $rd, $rs, $rt
        special![
            MoveZero,
            MoveNotZero,
            Add,
            AddUnsigned,
            Subtract,
            SubtractUnsigned,
            And,
            Or,
            Xor,
            Nor,
            SetLessThan,
            SetLessThanUnsigned,
        ]
        | special_2!(Multiply) => match operands {
            [Some(Register(rd)), Some(Register(rs)), Some(Register(rt))] => assemble_r_type(
                &mut machine_code,
                Some(rs),
                Some(rt),
                Some(rd),
                None,
                fn_code,
            ),
            _ => return Err(Error::InternalLogicIssue),
        },
        // bltz $rs, imm_i16
        RegisterImmediate(_) => match operands {
            [Some(Register(rs)), Some(I16(imm)), None] => {
                assemble_regimm(&mut machine_code, rs, fn_code, imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // j index
        Jump | JumpAndLink => match operands {
            [Some(JumpIndex(index)), None, None] => {
                assemble_field!(index (26 bits) -> machine_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // beq $rs, $rt, imm_i16
        BranchEqual | BranchNotEqual => match operands {
            [Some(Register(rs)), Some(Register(rt)), Some(I16(imm))] => {
                assemble_i_type(&mut machine_code, Some(rs), Some(rt), imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // blez $rs, imm_i16
        BranchLessEqualZero | BranchGreaterThanZero => match operands {
            [Some(Register(rs)), Some(I16(imm)), None] => {
                assemble_i_type(&mut machine_code, Some(rs), None, imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // addi $rt, $rs, imm_i16
        AddImmediate
        | AddImmediateUnsigned
        | SetLessThanImmediate
        | SetLessThanImmediateUnsigned => match operands {
            [Some(Register(rt)), Some(Register(rs)), Some(I16(imm))] => {
                assemble_i_type(&mut machine_code, Some(rs), Some(rt), imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // andi $rt, $rs, imm_u16
        AndImmediate | OrImmediate | XorImmediate => match operands {
            [Some(Register(rt)), Some(Register(rs)), Some(U16(imm))] => {
                assemble_i_type(&mut machine_code, Some(rs), Some(rt), imm as i16);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // lui $rt, imm_i16
        LoadUpperImmediate => match operands {
            [Some(Register(rt)), Some(I16(imm)), None] => {
                assemble_i_type(&mut machine_code, None, Some(rt), imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // mfc0 $rt, $rd
        coprocessor_0![MoveFromCoprocessor0, MoveToCoprocessor0] => match operands {
            [Some(Register(rt)), Some(Register(rd)), None] => {
                assemble_coprocessor_0(&mut machine_code, fn_code, Some(rt), Some(rd), None)
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // eret
        coprocessor_0!(ErrorReturn) => match operands {
            [None, None, None] => {
                assemble_coprocessor_0(&mut machine_code, fn_code, None, None, Some(0x18))
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // add.s $fd, $fs, $ft
        coprocessor_1![{fmt} Add, Subtract, Multiply, Divide] => match operands {
            [Some(Register(fd)), Some(Register(fs)), Some(Register(ft))] => {
                assemble_coprocessor_1(
                    &mut machine_code,
                    fmt,
                    Some(ft),
                    Some(fs),
                    Some(fd),
                    fn_code,
                );
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // sqrt.s $fd, $fs
        coprocessor_1![
            {fmt}
            SquareRoot,
            AbsoluteValue,
            Move,
            Negate,
            RoundWord,
            TruncateWord,
            CeilingWord,
            FloorWord,
            ConvertToSingle,
            ConvertToDouble,
            ConvertToWord,
        ] => match operands {
            [Some(Register(fd)), Some(Register(fs)), None] => {
                assemble_coprocessor_1(&mut machine_code, fmt, None, Some(fs), Some(fd), fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // movt.s $fd, $fs
        // movt.s $fd, $fs, cc
        coprocessor_1!({fmt} MoveConditional, Some(condition)) => match operands {
            // movt.s $fd, $fs
            [Some(Register(fd)), Some(Register(fs)), None] => {
                assemble_coprocessor_1_cc_c(&mut machine_code, fmt, 0, condition, fd, fs, fn_code);
            }
            // movt.s $fd, $fs, cc
            [Some(Register(fd)), Some(Register(fs)), Some(Cc(cc))] => {
                assemble_coprocessor_1_cc_c(&mut machine_code, fmt, cc, condition, fd, fs, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // movz.s $fd, $fs, $rt
        coprocessor_1![{fmt} MoveZero, MoveNotZero] => match operands {
            // movz.s $fd, $fs, $rt
            [Some(Register(fd)), Some(Register(fs)), Some(Register(rt))] => {
                assemble_coprocessor_1(
                    &mut machine_code,
                    fmt,
                    Some(rt),
                    Some(fs),
                    Some(fd),
                    fn_code,
                );
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // c.eq.s $fs, $ft
        // c.eq.s cc, $fs, $ft
        coprocessor_1![{fmt} CompareEqual, CompareLessThan, CompareLessEqual] => match operands {
            // c.eq.s $fs, $ft
            [Some(Register(fs)), Some(Register(ft)), None] => {
                assemble_coprocessor_1(&mut machine_code, fmt, Some(ft), Some(fs), None, fn_code);
            }
            // c.eq.s cc, $fs, $ft
            [Some(Cc(cc)), Some(Register(fs)), Some(Register(ft))] => {
                assemble_coprocessor_1(
                    &mut machine_code,
                    fmt,
                    Some(ft),
                    Some(fs),
                    Some(cc << 2),
                    fn_code,
                );
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // clz $rd, $rs
        special_2![CountLeadingZeroes, CountLeadingOnes] => match operands {
            [Some(Register(rd)), Some(Register(rs)), None] => {
                assemble_r_type(&mut machine_code, Some(rs), None, Some(rd), None, fn_code);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        // lb $rt, imm_i16($rs)
        LoadByte
        | LoadHalf
        | LoadWordLeft
        | LoadWord
        | LoadByteUnsigned
        | LoadHalfUnsigned
        | LoadWordRight
        | StoreByte
        | StoreHalf
        | StoreWordLeft
        | StoreWord
        | StoreConditional
        | StoreWordRight
        | LoadLinked
        | LoadWordCoprocessor1
        | LoadDoubleCoprocessor1
        | StoreWordCoprocessor1
        | StoreDoubleCoprocessor1 => match operands {
            [Some(Register(rt)), Some(I16(imm)), Some(WrappedRegister(rs))] => {
                assemble_i_type(&mut machine_code, Some(rs), Some(rt), imm);
            }
            _ => return Err(Error::InternalLogicIssue),
        },
        _ => unreachable!("all cases should have been covered by now"),
    }
    Ok(machine_code)
}

fn assemble_r_type(
    machine_code: &mut u32,
    rs: Option<u8>,
    rt: Option<u8>,
    rd: Option<u8>,
    shamt: Option<u8>,
    fn_code: u8,
) {
    assemble_field!(rs.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(rt.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(rd.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(shamt.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(fn_code (6 bits) -> *machine_code);
}

fn assemble_coprocessor_1(
    machine_code: &mut u32,
    fmt: NumberFormat,
    ft: Option<u8>,
    fs: Option<u8>,
    fd: Option<u8>,
    fn_code: u8,
) {
    assemble_field!(fmt as u8; (5 bits) -> *machine_code);
    assemble_field!(ft.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(fs.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(fd.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(fn_code (6 bits) -> *machine_code);
}

fn assemble_i_type(machine_code: &mut u32, rs: Option<u8>, rt: Option<u8>, imm: i16) {
    assemble_field!(rs.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(rt.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(imm #(16 bits) -> *machine_code);
}

fn assemble_movc(machine_code: &mut u32, rs: u8, cc: u8, condition: bool, rd: u8, fn_code: u8) {
    let rt = (cc << 2) | if condition { 1 } else { 0 };
    assemble_r_type(machine_code, Some(rs), Some(rt), Some(rd), None, fn_code);
}

fn assemble_coprocessor_1_cc_c(
    machine_code: &mut u32,
    fmt: NumberFormat,
    cc: u8,
    condition: bool,
    fd: u8,
    fs: u8,
    fn_code: u8,
) {
    let ft = (cc << 2) | if condition { 1 } else { 0 };
    assemble_coprocessor_1(machine_code, fmt, Some(ft), Some(fs), Some(fd), fn_code);
}

fn assemble_regimm(machine_code: &mut u32, rs: u8, fn_code: u8, imm: i16) {
    assemble_field!(rs (5 bits) -> *machine_code);
    assemble_field!(fn_code (5 bits) -> *machine_code);
    assemble_field!(imm #(16 bits) -> *machine_code);
}

fn assemble_coprocessor_0(
    machine_code: &mut u32,
    fn_code: u8,
    rt: Option<u8>,
    rd: Option<u8>,
    idk: Option<u16>,
) {
    assemble_field!(fn_code (5 bits) -> *machine_code);
    assemble_field!(rt.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(rd.unwrap_or(0); (5 bits) -> *machine_code);
    assemble_field!(idk.unwrap_or(0); (11 bits) -> *machine_code);
}
