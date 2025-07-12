pub mod operator;

mod assemble;
mod macros;
mod process;

pub use operator::Operator;

use crate::{error::AssembleError, parser::Operand};
use assemble::insert;
use macros::*;
use process::{Destination, maybe, maybe_or};
use seaside_constants::{
    ConditionCode, Opcode,
    register::{CpuRegister, FpuRegister},
};
use seaside_error::rich::{RichError, RichResult, Span};
use seaside_type_aliases::{Address, Instruction};

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessedInstruction<'src> {
    MachineCode(Instruction),
    Unresolved(UnresolvedInstruction<'src>),
}

pub fn process_instruction<'src>(
    operator: &'src str,
    operands: Vec<(Operand<'src>, Span)>,
    expr_span: &Span,
    pc: Address,
) -> RichResult<ProcessedInstruction<'src>> {
    use Operator::*;

    let operator_span = Span {
        start: expr_span.start,
        end: expr_span.start + operator.len(),
    };

    // Using `map_err` would cause `expr_span` to get moved out unless we cloned it. :(
    let operator: Operator = operator.parse().map_err(|_| {
        RichError::new(AssembleError::UnknownOperator, expr_span.clone())
            .with_narrow_span(operator_span)
            .with_help("if you are trying to use a pseudo-operator, those aren't supported yet")
    })?;
    let mut operands_iter = operands.iter();
    let opcode = Opcode::from(operator);
    let fn_code = operator.op_or_fn_code();

    let mut machine_code: Instruction = opcode as Instruction;
    match operator {
        // sll $rd, $rt, shamt
        special![ShiftLeftLogical, ShiftRightLogical, ShiftRightArithmetic] => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let shamt = process::shamt(operands_iter.next(), expr_span)?;
            assemble::r_type(&mut machine_code, CpuRegister::Zero, rt, rd, shamt, fn_code);
        }
        // movt $rd, $rs
        // movt $rd, $rs, cc
        special!(MoveConditional, condition: condition) => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let cc = maybe_or(
                operands_iter.next(),
                expr_span,
                ConditionCode::_0,
                process::cc,
            )?;
            assemble::movc(&mut machine_code, rs, cc, condition, rd, fn_code);
        }
        // sllv $rd, $rt, $rs
        special![
            ShiftLeftLogicalVariable,
            ShiftRightLogicalVariable,
            ShiftRightArithmeticVariable
        ] => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            assemble::r_type(&mut machine_code, rs, rt, rd, 0, fn_code);
        }
        // jr $rs
        special![JumpRegister, MoveToHigh, MoveToLow] => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            insert!({5} rs, {21} fn_code => machine_code);
        }
        // jalr $rs
        // jalr $rd, $rs
        special!(JumpAndLinkRegister, condition: _) => {
            let rs_or_rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let (rs, rd) =
                if let Some(rs) = maybe(operands_iter.next(), expr_span, process::cpu_register)? {
                    (rs, rs_or_rd)
                } else {
                    (rs_or_rd, CpuRegister::ReturnAddr)
                };
            assemble::r_type(&mut machine_code, rs, CpuRegister::Zero, rd, 0, fn_code);
        }
        // syscall
        special!(SystemCall, condition: _) => {
            insert!({26} fn_code => machine_code);
        }
        // break
        // break code
        special!(Break, condition: _) => {
            let code = maybe_or(operands_iter.next(), expr_span, 0, process::code)?;
            insert!({20} code, {6} fn_code => machine_code);
        }
        // mfhi $rd
        special![MoveFromHigh, MoveFromLow] => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            insert!({15} rd, {11} fn_code => machine_code);
        }
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
            MultiplySubtractUnsigned,
        ] => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            assemble::r_type(&mut machine_code, rs, rt, CpuRegister::Zero, 0, fn_code);
        }
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
        | special_2!(Multiply) => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            assemble::r_type(&mut machine_code, rs, rt, rd, 0, fn_code);
        }
        // bltz $rs, address
        // bltz $rs, label
        register_immediate![
            BranchLessThanZero,
            BranchGreaterEqualZero,
            BranchLessThanZeroAndLink,
            BranchGreaterEqualZeroAndLink,
        ] => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let offset = match process::destination(operands_iter.next(), expr_span)? {
                Destination::Address(address, span) => {
                    address_to_offset(address, pc, expr_span, &span)?
                }
                Destination::Label(label, span) => {
                    process::finish(operands_iter.next(), expr_span)?;
                    return Ok(ProcessedInstruction::Unresolved(
                        UnresolvedInstruction::BranchRegImm {
                            operator,
                            rs,
                            label: (label, span),
                        },
                    ));
                }
            };
            assemble::regimm(&mut machine_code, rs, fn_code, offset as u16);
        }
        // tgei $rs, imm_16
        register_immediate![
            TrapGreaterEqualImmediate,
            TrapGreaterEqualImmediateUnsigned,
            TrapLessThanImmediate,
            TrapLessThanImmediateUnsigned,
            TrapEqualImmediate,
            TrapNotEqualImmediate,
        ] => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let imm = process::imm_i16(operands_iter.next(), expr_span)?;
            assemble::regimm(&mut machine_code, rs, fn_code, imm as u16);
        }
        // j address
        // j label
        Jump | JumpAndLink => {
            let jump_index = match process::destination(operands_iter.next(), expr_span)? {
                Destination::Address(address, span) => {
                    address_to_index(address, pc, expr_span, &span)?
                }
                Destination::Label(label, span) => {
                    process::finish(operands_iter.next(), expr_span)?;
                    return Ok(ProcessedInstruction::Unresolved(
                        UnresolvedInstruction::Jump {
                            operator,
                            label: (label, span),
                        },
                    ));
                }
            };
            assemble::j_type(&mut machine_code, jump_index);
        }
        // beq $rs, $rt, address
        // beq $rs, $rt, label
        BranchEqual | BranchNotEqual => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let offset = match process::destination(operands_iter.next(), expr_span)? {
                Destination::Address(address, span) => {
                    address_to_offset(address, pc, expr_span, &span)?
                }
                Destination::Label(label, span) => {
                    process::finish(operands_iter.next(), expr_span)?;
                    return Ok(ProcessedInstruction::Unresolved(
                        UnresolvedInstruction::BranchIType {
                            operator,
                            rs,
                            rt,
                            label: (label, span),
                        },
                    ));
                }
            };
            assemble::i_type(&mut machine_code, rs, rt, offset as u16);
        }
        // blez $rs, address
        // blez $rs, label
        BranchLessEqualZero | BranchGreaterThanZero => {
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let offset = match process::destination(operands_iter.next(), expr_span)? {
                Destination::Address(address, span) => {
                    address_to_offset(address, pc, expr_span, &span)?
                }
                Destination::Label(label, span) => {
                    process::finish(operands_iter.next(), expr_span)?;
                    return Ok(ProcessedInstruction::Unresolved(
                        UnresolvedInstruction::BranchIType {
                            operator,
                            rs,
                            rt: CpuRegister::Zero,
                            label: (label, span),
                        },
                    ));
                }
            };
            assemble::i_type(&mut machine_code, rs, CpuRegister::Zero, offset as u16);
        }
        // addi $rt, $rs, imm_i16
        AddImmediate
        | AddImmediateUnsigned
        | SetLessThanImmediate
        | SetLessThanImmediateUnsigned => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let imm = process::imm_i16(operands_iter.next(), expr_span)?;
            assemble::i_type(&mut machine_code, rs, rt, imm as u16);
        }
        // andi $rt, $rs, imm_u16
        AndImmediate | OrImmediate | XorImmediate => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            let imm = process::imm_u16(operands_iter.next(), expr_span)?;
            assemble::i_type(&mut machine_code, rs, rt, imm);
        }
        // lui $rt, imm_i16
        LoadUpperImmediate => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let imm = process::imm_u16(operands_iter.next(), expr_span)?;
            assemble::i_type(&mut machine_code, CpuRegister::Zero, rt, imm);
        }
        // mfc0 $rt, $rd
        // note: $rd is a coprocessor 0 register, not a cpu register
        coprocessor_0![MoveFromCoprocessor0, MoveToCoprocessor0] => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let rd = process::coprocessor_0_register(operands_iter.next(), expr_span)?;
            assemble::coprocessor_0(&mut machine_code, fn_code, rt, rd);
        }
        // eret
        coprocessor_0!(ErrorReturn) => {
            insert!({5} fn_code, {21} 0x18 => machine_code);
        }
        // add.s $fd, $fs, $ft
        coprocessor_1![{fmt} Add, Subtract, Multiply, Divide] => {
            let fd = process::fpu_register(operands_iter.next(), expr_span)?;
            let fs = process::fpu_register(operands_iter.next(), expr_span)?;
            let ft = process::fpu_register(operands_iter.next(), expr_span)?;
            assemble::coprocessor_1(&mut machine_code, fmt, ft, fs, fd, fn_code);
        }
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
        ] => {
            let fd = process::fpu_register(operands_iter.next(), expr_span)?;
            let fs = process::fpu_register(operands_iter.next(), expr_span)?;
            assemble::coprocessor_1(&mut machine_code, fmt, FpuRegister::F0, fs, fd, fn_code);
        }
        // movt.s $fd, $fs
        // movt.s $fd, $fs, cc
        coprocessor_1!({fmt} MoveConditional, condition: condition) => {
            let fd = process::fpu_register(operands_iter.next(), expr_span)?;
            let fs = process::fpu_register(operands_iter.next(), expr_span)?;
            let cc = maybe_or(
                operands_iter.next(),
                expr_span,
                ConditionCode::_0,
                process::cc,
            )?;
            assemble::coprocessor_1_with_cc_c(
                &mut machine_code,
                fmt,
                cc,
                condition,
                fd,
                fs,
                fn_code,
            );
        }
        // movz.s $fd, $fs, $rt
        coprocessor_1![{fmt} MoveZero, MoveNotZero] => {
            let fd = process::fpu_register(operands_iter.next(), expr_span)?;
            let fs = process::fpu_register(operands_iter.next(), expr_span)?;
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            assemble::coprocessor_1(&mut machine_code, fmt, rt.to_fpu(), fs, fd, fn_code);
        }
        // c.eq.s $fs, $ft
        // c.eq.s cc, $fs, $ft
        /* c.eq.s 7, $f21, $f22
         *   op    fmt   $ft   $fs  cc       fn
         * 010001 10000 10110 10101 111 00 110010
         *  fpu    .s   $f22  $f21   7      c.eq
         */
        coprocessor_1![{fmt} CompareEqual, CompareLessThan, CompareLessEqual] => {
            let (cc, next) = process::maybe_cc(&mut operands_iter, expr_span)?;
            let fs = process::fpu_register(next, expr_span)?;
            let ft = process::fpu_register(operands_iter.next(), expr_span)?;
            insert!({5} fmt, {5} ft, {5} fs, {3} cc, {8} fn_code => machine_code);
        }
        // mfc1 $rt, $fs
        coprocessor_1_register_immediate![MoveFromCoprocessor1, MoveToCoprocessor1] => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let fs = process::fpu_register(operands_iter.next(), expr_span)?;
            assemble::coprocessor_1_register_immediate(&mut machine_code, fn_code, rt, fs);
        }
        // bc1t address
        // bc1t label
        // bc1t cc, address
        // bc1t cc, label
        coprocessor_1_register_immediate!(BranchCoprocessor1Flag, condition: condition) => {
            let (cc, next) = process::maybe_cc(&mut operands_iter, expr_span)?;
            let offset = match process::destination(next, expr_span)? {
                Destination::Address(address, span) => {
                    address_to_offset(address, pc, expr_span, &span)?
                }
                Destination::Label(label, span) => {
                    process::finish(operands_iter.next(), expr_span)?;
                    return Ok(ProcessedInstruction::Unresolved(
                        UnresolvedInstruction::BranchCoprocessor1Flag {
                            cc,
                            condition,
                            label: (label, span),
                        },
                    ));
                }
            };
            assemble::bc1c(&mut machine_code, cc, condition, offset as u16);
        }
        // clz $rd, $rs
        special_2![CountLeadingZeroes, CountLeadingOnes] => {
            let rd = process::cpu_register(operands_iter.next(), expr_span)?;
            let rs = process::cpu_register(operands_iter.next(), expr_span)?;
            assemble::r_type(&mut machine_code, rs, CpuRegister::Zero, rd, 0, fn_code);
        }
        // lb $rt, ($rs)
        // lb $rt, imm_i16($rs)
        LoadByte | LoadHalf | LoadWordLeft | LoadWord | LoadByteUnsigned | LoadHalfUnsigned
        | LoadWordRight | StoreByte | StoreHalf | StoreWordLeft | StoreWord | StoreConditional
        | StoreWordRight | LoadLinked => {
            let rt = process::cpu_register(operands_iter.next(), expr_span)?;
            let (imm, rs) = process::offset_cpu_register(
                [operands_iter.next(), operands_iter.next()],
                expr_span,
            )?;
            assemble::i_type(&mut machine_code, rs, rt, imm as u16);
        }
        // lwc1 $ft, ($rs)
        // lwc1 $ft, imm_i16($rs)
        LoadWordCoprocessor1
        | LoadDoubleCoprocessor1
        | StoreWordCoprocessor1
        | StoreDoubleCoprocessor1 => {
            let ft = process::fpu_register(operands_iter.next(), expr_span)?;
            let (imm, rs) = process::offset_cpu_register(
                [operands_iter.next(), operands_iter.next()],
                expr_span,
            )?;
            assemble::i_type(&mut machine_code, rs, ft.to_cpu(), imm as u16);
        }
    }

    process::finish(operands_iter.next(), expr_span)?;
    Ok(ProcessedInstruction::MachineCode(machine_code))
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnresolvedInstruction<'src> {
    Jump {
        operator: Operator,
        label: (&'src str, Span),
    },
    BranchRegImm {
        operator: Operator,
        rs: CpuRegister,
        label: (&'src str, Span),
    },
    BranchIType {
        operator: Operator,
        rs: CpuRegister,
        rt: CpuRegister,
        label: (&'src str, Span),
    },
    BranchCoprocessor1Flag {
        cc: ConditionCode,
        condition: bool,
        label: (&'src str, Span),
    },
}

impl UnresolvedInstruction<'_> {
    pub const fn spanned_label(&self) -> &(&str, Span) {
        match self {
            Self::Jump { operator: _, label }
            | Self::BranchRegImm {
                operator: _,
                rs: _,
                label,
            }
            | Self::BranchIType {
                operator: _,
                rs: _,
                rt: _,
                label,
            }
            | Self::BranchCoprocessor1Flag {
                cc: _,
                condition: _,
                label,
            } => label,
        }
    }

    pub fn resolve(self, span: &Span, address: Address, pc: Address) -> RichResult<Instruction> {
        use UnresolvedInstruction::*;
        match self {
            Jump {
                operator,
                label: (_, label_span),
            } => Self::resolve_jump(operator, &label_span, span, address, pc),
            BranchRegImm {
                operator,
                rs,
                label: (_, label_span),
            } => Self::resolve_register_immediate(operator, rs, &label_span, span, address, pc),
            BranchIType {
                operator,
                rs,
                rt,
                label: (_, label_span),
            } => Self::resolve_i_type(operator, rs, rt, &label_span, span, address, pc),
            BranchCoprocessor1Flag {
                cc,
                condition,
                label: (_, label_span),
            } => Self::resole_bc1c(cc, condition, &label_span, span, address, pc),
        }
    }

    fn resolve_jump(
        operator: Operator,
        label_span: &Span,
        instruction_span: &Span,
        address: Address,
        pc: Address,
    ) -> RichResult<Instruction> {
        let mut machine_code: Instruction = 0;
        let jump_index = address_to_index(address, pc, instruction_span, label_span)?;
        insert!({6} Opcode::from(operator) => machine_code);
        assemble::j_type(&mut machine_code, jump_index);
        Ok(machine_code)
    }

    fn resolve_register_immediate(
        operator: Operator,
        rs: CpuRegister,
        label_span: &Span,
        instruction_span: &Span,
        address: Address,
        pc: Address,
    ) -> RichResult<Instruction> {
        let mut machine_code: Instruction = 0;
        let offset = address_to_offset(address, pc, instruction_span, label_span)?;
        insert!({6} Opcode::RegisterImmediate => machine_code);
        assemble::regimm(
            &mut machine_code,
            rs,
            operator.op_or_fn_code(),
            offset as u16,
        );
        Ok(machine_code)
    }

    fn resolve_i_type(
        operator: Operator,
        rs: CpuRegister,
        rt: CpuRegister,
        label_span: &Span,
        instruction_span: &Span,
        address: Address,
        pc: Address,
    ) -> RichResult<Instruction> {
        let mut machine_code: Instruction = 0;
        let offset = address_to_offset(address, pc, instruction_span, label_span)?;
        insert!({6} Opcode::from(operator) => machine_code);
        assemble::i_type(&mut machine_code, rs, rt, offset as u16);
        Ok(machine_code)
    }

    fn resole_bc1c(
        cc: ConditionCode,
        condition: bool,
        label_span: &Span,
        instruction_span: &Span,
        address: Address,
        pc: Address,
    ) -> RichResult<Instruction> {
        let mut machine_code: Instruction = Opcode::Coprocessor1 as Instruction;
        let offset = address_to_offset(address, pc, instruction_span, label_span)?;
        assemble::bc1c(&mut machine_code, cc, condition, offset as u16);
        Ok(machine_code)
    }
}

fn address_to_offset(
    address: Address,
    pc: Address,
    instruction_span: &Span,
    address_span: &Span,
) -> RichResult<i16> {
    let offset = (address as i32 - pc as i32) / 4 - 1;
    <i32 as TryInto<i16>>::try_into(offset).map_err(|_| {
        RichError::new(AssembleError::OffsetTooLarge, instruction_span.clone())
            .with_narrow_span(address_span.clone())
            .with_note("can only branch by -128..128 KiB at a time")
    })
}

fn address_to_index(
    address: Address,
    pc: Address,
    instruction_span: &Span,
    address_span: &Span,
) -> RichResult<u32> {
    // A jump index is essentially the index of the instruction to jump to in the current "block".
    // These "blocks" are 0x10000000 in size, so as long as the most significant nibble of `address`
    // and `pc` are the same, there will be a valid jump index.
    //
    // This limitation comes from the width of the instruction. All instructions are 4 bytes (32
    // bits) wide, and the 6 most significant bits are reserved for the opcode. For `j` and `jal`,
    // this leaves 26 bits for the index, as they only have one argument. We can ignore the 2 least
    // significant bits of the address (they're guaranteed to be 0s and therefore redundant), so
    // we only have 30 bits of meaningful data to store. Still, that's 4 too many bits for our
    // instruction. To get around this, we discard the 4 most significant bits of the address and
    // have the processor derive those bits from the program counter at runtime.
    //
    // Here's an example of how this would work:
    //
    // ```mips
    // .text 0x00400000
    // main:
    //     jal Foo
    //
    //     main.epilogue:
    //         addiu $v0, $0, 10
    //         syscall
    //     main.endepilogue:
    // main.end:
    //
    // .text 0x0068129c
    // Foo:
    //     # ...
    // Foo.end:
    // ```
    //
    // `jal Foo` is at address 0x00400000, so the program counter it'll use in its calculation is
    // 0x00400004. `Foo` is at 0x0068129c, which breaks down into the following bits:
    //
    // block index         jump index
    //    0000     00000110100000010010100111 00
    //
    // As previously stated, the most significant nibble must match that of the program counter so
    // the correct address can be derived at runtime. It is in this case, so we can safely ignore
    // it. We may also discard the two least significant bits, leaving us with this index:
    //
    // 00000110100000010010100111
    //
    // Finally, we can slot it into our `jal` instruction:
    //
    // 000011 00000110100000010010100111
    //  jal              Foo
    //
    // Thus, `jal Foo` becomes 0x0c1a04a7.
    let pc_plus_4 = pc.checked_add(4).ok_or_else(|| {
        RichError::new(
            AssembleError::ProgramCounterOverflow,
            instruction_span.clone(),
        )
        .with_narrow_span(address_span.clone())
    })?;
    if (address ^ pc_plus_4) & 0xf0000000 == 0 {
        Ok((address & 0x0fffffff) >> 2)
    } else {
        Err(
            RichError::new(AssembleError::JumpTooLarge, instruction_span.clone())
                .with_narrow_span(address_span.clone())
                .with_note("jumps can only reach addresses within the current 256 MiB block")
                .with_help("consider chaining multiple jump instructions to reach your target"),
        )
    }
}
