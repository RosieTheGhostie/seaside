mod macros;
mod process;

use seaside_core::{UnpackedInstruction, prelude::*};
use seaside_rich_error::{RichError, RichResultBuilder, Span, map_err, result::Bailed};

use crate::{error::AssembleError, parser::Operand};
use macros::*;
use process::{Destination, Processor};

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessedInstruction<'src> {
    Resolved(UnpackedInstruction),
    Unresolved(UnresolvedInstruction<'src>),
}

pub fn process_instruction<'src>(
    result_builder: &mut RichResultBuilder,
    operator: &'src str,
    operands: Vec<(Operand<'src>, Span)>,
    expr_span: &Span,
    pc: Address,
) -> Result<ProcessedInstruction<'src>, Bailed> {
    let Some(mut template) = UnpackedInstruction::parse_from_operator(operator) else {
        return result_builder.bail(
            RichError::new(AssembleError::UnknownOperator, expr_span.clone())
                .with_narrow_span(Span {
                    start: expr_span.start,
                    end: expr_span.start + operator.len(),
                })
                .with_help(
                    "if you are trying to use a pseudo-operator, those aren't supported yet",
                ),
        );
    };

    let mut operands_iter = operands.iter();
    let mut processor = Processor::new(
        result_builder,
        &mut operands_iter as &mut dyn Iterator<Item = _>,
        expr_span,
    );
    match &mut template {
        special![{fields} ShiftLeftLogical, ShiftRightLogical, ShiftRightArithmetic] => {
            process::shift_by_amount(&mut processor, fields)
        }
        special!({fields} MoveConditional) => process::cpu_move_conditional(&mut processor, fields),
        special![
            {fields}
            ShiftLeftLogicalVariable,
            ShiftRightLogicalVariable,
            ShiftRightArithmeticVariable,
        ] => process::shift_by_variable(&mut processor, fields),
        special!({fields} JumpRegister) => process::jr(&mut processor, fields),
        special!({fields} JumpAndLinkRegister) => process::jalr(&mut processor, fields),
        special!({_} SystemCall) | coprocessor_0!({_} ErrorReturn) => Ok(()),
        special!({fields} Break) => process::r#break(&mut processor, fields),
        special![{fields} MoveFromHigh, MoveFromLow] => {
            process::move_from_high_or_low(&mut processor, fields)
        }
        special![
            {fields}
            Multiply,
            MultiplyUnsigned,
            DivideUnsigned,
            TrapGreaterEqual,
            TrapGreaterEqualUnsigned,
            TrapLessThan,
            TrapLessThanUnsigned,
            TrapEqual,
            TrapNotEqual,
        ] => process::int_math_2_ops(&mut processor, fields),
        special![
            {fields}
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
        ] => process::int_math_3_ops(&mut processor, fields),
        // bltz $rs, address
        // bltz $rs, label
        register_immediate![
            {fields}
            BranchLessThanZero,
            BranchGreaterEqualZero,
            BranchLessThanZeroAndLink,
            BranchGreaterEqualZeroAndLink,
        ] => {
            fields.rs = processor.cpu_register()?.to_indexed();
            match processor.destination()? {
                Destination::Address(address, span) => {
                    fields.imm = address_to_offset(
                        processor.result_builder,
                        processor.expr_span,
                        &span,
                        address,
                        pc,
                    )? as _;
                }
                Destination::Label(label, span) => {
                    processor.finish()?;
                    return Ok(ProcessedInstruction::Unresolved(UnresolvedInstruction {
                        template,
                        label: (label, span),
                    }));
                }
            };

            Ok(())
        }
        // tgei $rs, imm_16
        register_immediate![
            {fields}
            TrapGreaterEqualImmediate,
            TrapGreaterEqualImmediateUnsigned,
            TrapLessThanImmediate,
            TrapLessThanImmediateUnsigned,
            TrapEqualImmediate,
            TrapNotEqualImmediate,
        ] => {
            fields.rs = processor.cpu_register()?.to_indexed();
            fields.imm = processor.signed_immediate()? as _;

            Ok(())
        }
        // j address
        // j label
        jump![{fields} Jump, JumpAndLink] => {
            match processor.destination()? {
                Destination::Address(address, span) => {
                    fields.set_index(address_to_index(
                        processor.result_builder,
                        processor.expr_span,
                        &span,
                        address,
                        pc,
                    )?);
                }
                Destination::Label(label, span) => {
                    processor.finish()?;
                    return Ok(ProcessedInstruction::Unresolved(UnresolvedInstruction {
                        template,
                        label: (label, span),
                    }));
                }
            };

            Ok(())
        }
        // beq $rs, $rt, address
        // beq $rs, $rt, label
        immediate![
            {fields}
            BranchEqual,
            BranchEqualLikely,
            BranchNotEqual,
            BranchNotEqualLikely,
        ] => {
            fields.rs = processor.cpu_register()?.to_indexed();
            fields.rt = processor.cpu_register()?.to_indexed();
            match processor.destination()? {
                Destination::Address(address, span) => {
                    fields.imm = address_to_offset(
                        processor.result_builder,
                        processor.expr_span,
                        &span,
                        address,
                        pc,
                    )? as _;
                }
                Destination::Label(label, span) => {
                    processor.finish()?;
                    return Ok(ProcessedInstruction::Unresolved(UnresolvedInstruction {
                        template,
                        label: (label, span),
                    }));
                }
            };

            Ok(())
        }
        // blez $rs, address
        // blez $rs, label
        immediate![
            {fields}
            BranchLessEqualZero,
            BranchLessEqualZeroLikely,
            BranchGreaterThanZero,
            BranchGreaterThanZeroLikely,
        ] => {
            fields.rs = processor.cpu_register()?.to_indexed();
            match processor.destination()? {
                Destination::Address(address, span) => {
                    fields.imm = address_to_offset(
                        processor.result_builder,
                        processor.expr_span,
                        &span,
                        address,
                        pc,
                    )? as _;
                }
                Destination::Label(label, span) => {
                    processor.finish()?;
                    return Ok(ProcessedInstruction::Unresolved(UnresolvedInstruction {
                        template,
                        label: (label, span),
                    }));
                }
            };

            Ok(())
        }
        immediate![
            {fields}
            AddImmediate,
            AddImmediateUnsigned,
            SetLessThanImmediate,
            SetLessThanImmediateUnsigned,
        ] => process::int_math_signed_immediate(&mut processor, fields),
        immediate![{fields} AndImmediate, OrImmediate, XorImmediate] => {
            process::int_math_unsigned_immediate(&mut processor, fields)
        }
        immediate!({fields} LoadUpperImmediate) => process::lui(&mut processor, fields),
        coprocessor_0![{fields} MoveFromCoprocessor0, MoveToCoprocessor0] => {
            process::move_between_cpu_and_coprocessor_0(&mut processor, fields)
        }
        coprocessor_1![{fields} Add, Subtract, Multiply, Divide] => {
            process::float_math_3_ops(&mut processor, fields)
        }
        coprocessor_1![
            {fields}
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
        ] => process::float_math_2_ops(&mut processor, fields),
        coprocessor_1!({fields} MoveConditional) => {
            process::coprocessor_1_move_conditional(&mut processor, fields)
        }
        coprocessor_1![{fields} MoveZero, MoveNotZero] => {
            process::coprocessor_1_move_by_comparison(&mut processor, fields)
        }
        coprocessor_1![
            {fields}
            CompareFalse,
            CompareUnordered,
            CompareEqual,
            CompareUnorderedEqual,
            CompareOrderedLessThan,
            CompareUnorderedLessThan,
            CompareOrderedLessEqual,
            CompareUnorderedLessEqual,
            CompareSignalFalse,
            CompareNotGreaterLessEqual,
            CompareSignalEqual,
            CompareNotGreaterLess,
            CompareLessThan,
            CompareNotGreaterEqual,
            CompareLessEqual,
            CompareNotGreaterThan,
        ] => process::float_compare(&mut processor, fields),
        coprocessor_1_register_immediate![{fields} MoveFromCoprocessor1, MoveToCoprocessor1] => {
            process::move_between_cpu_and_coprocessor_1(&mut processor, fields)
        }
        // bc1t address
        // bc1t label
        // bc1t cc, address
        // bc1t cc, label
        coprocessor_1_register_immediate!({fields} BranchCoprocessor1Flag) => {
            fields.set_cc(processor.maybe_condition_code_if_int()?.unwrap_or_default());
            match processor.destination()? {
                Destination::Address(address, span) => {
                    fields.set_imm(address_to_offset(
                        processor.result_builder,
                        processor.expr_span,
                        &span,
                        address,
                        pc,
                    )? as _);
                }
                Destination::Label(label, span) => {
                    processor.finish()?;
                    return Ok(ProcessedInstruction::Unresolved(UnresolvedInstruction {
                        template,
                        label: (label, span),
                    }));
                }
            }

            Ok(())
        }
        coprocessor_1x![
            {fields}
            MultiplyAddSingle,
            MultiplyAddDouble,
            MultiplySubtractSingle,
            MultiplySubtractDouble,
            NegativeMultiplyAddSingle,
            NegativeMultiplyAddDouble,
            NegativeMultiplySubtractSingle,
            NegativeMultiplySubtractDouble,
        ] => process::float_multiply_accumulate(&mut processor, fields),
        coprocessor_2_register_immediate![{fields} MoveFromCoprocessor2, MoveToCoprocessor2] => {
            process::move_between_cpu_and_coprocessor_2(&mut processor, fields)
        }
        special_2![
            {fields}
            MultiplyAdd,
            MultiplyAddUnsigned,
            MultiplySubtract,
            MultiplySubtractUnsigned,
        ] => process::int_multiply_accumulate(&mut processor, fields),
        special_2!({fields} Multiply) => process::mul(&mut processor, fields),
        special_2![{fields} CountLeadingZeroes, CountLeadingOnes] => {
            process::count_leading_bits(&mut processor, fields)
        }
        immediate![
            {fields}
            LoadByte,
            LoadHalf,
            LoadWordLeft,
            LoadWord,
            LoadByteUnsigned,
            LoadHalfUnsigned,
            LoadWordRight,
            StoreByte,
            StoreHalf,
            StoreWordLeft,
            StoreWord,
            StoreConditional,
            StoreWordRight,
            LoadLinked,
        ] => process::cpu_memory_access(&mut processor, fields),
        immediate![
            {fields}
            LoadWordCoprocessor1,
            LoadDoubleCoprocessor1,
            StoreWordCoprocessor1,
            StoreDoubleCoprocessor1,
        ] => process::coprocessor_1_memory_access(&mut processor, fields),
        _ => todo!(),
    }?;

    processor
        .finish()
        .map(|()| ProcessedInstruction::Resolved(template))
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnresolvedInstruction<'src> {
    template: UnpackedInstruction,
    label: (&'src str, Span),
}

impl UnresolvedInstruction<'_> {
    pub const fn spanned_label(&self) -> &(&str, Span) {
        &self.label
    }

    pub fn resolve(
        mut self,
        result_builder: &mut RichResultBuilder,
        span: &Span,
        address: Address,
        pc: Address,
    ) -> Result<UnpackedInstruction, Bailed> {
        let context = ResolverContext {
            result_builder,
            instruction_span: span,
            label_span: &self.label.1,
            address,
            pc,
        };
        match &mut self.template {
            jump![{fields} Jump, JumpAndLink] => Self::resolve_jump(context, fields),
            register_immediate![
                {fields}
                BranchLessThanZero,
                BranchLessThanZeroLikely,
                BranchLessThanZeroAndLink,
                BranchLessThanZeroAndLinkLikely,
                BranchGreaterEqualZero,
                BranchGreaterEqualZeroLikely,
                BranchGreaterEqualZeroAndLink,
                BranchGreaterEqualZeroAndLinkLikely,
            ] => Self::resolve_register_immediate(context, fields),
            immediate![
                {fields}
                BranchEqual,
                BranchEqualLikely,
                BranchNotEqual,
                BranchNotEqualLikely,
                BranchLessEqualZero,
                BranchLessEqualZeroLikely,
                BranchGreaterThanZero,
                BranchGreaterThanZeroLikely,
            ] => Self::resolve_i_type(context, fields),
            coprocessor_1_register_immediate!({fields} BranchCoprocessor1Flag) => {
                Self::resolve_bc1c(context, fields)
            }
            _ => panic!("marked resolved instruction as unresolved"),
        }?;

        Ok(self.template)
    }

    fn resolve_jump(
        mut context: ResolverContext<'_>,
        fields: &mut seaside_core::instruction::jump::Fields,
    ) -> Result<(), Bailed> {
        context
            .address_to_index()
            .map(|index| fields.set_index(index))
    }

    fn resolve_register_immediate(
        mut context: ResolverContext<'_>,
        fields: &mut seaside_core::instruction::register_immediate::Fields,
    ) -> Result<(), Bailed> {
        context
            .address_to_offset()
            .map(|offset| fields.imm = offset as _)
    }

    fn resolve_i_type(
        mut context: ResolverContext<'_>,
        fields: &mut seaside_core::instruction::immediate::Fields,
    ) -> Result<(), Bailed> {
        context
            .address_to_offset()
            .map(|offset| fields.imm = offset as _)
    }

    fn resolve_bc1c(
        mut context: ResolverContext<'_>,
        fields: &mut seaside_core::instruction::coprocessor_1::RegisterImmediateFields,
    ) -> Result<(), Bailed> {
        context
            .address_to_offset()
            .map(|offset| fields.set_imm(offset as _))
    }
}

struct ResolverContext<'a> {
    pub result_builder: &'a mut RichResultBuilder,
    pub instruction_span: &'a Span,
    pub label_span: &'a Span,
    pub address: Address,
    pub pc: Address,
}

impl ResolverContext<'_> {
    pub fn address_to_offset(&mut self) -> Result<i16, Bailed> {
        address_to_offset(
            self.result_builder,
            self.instruction_span,
            self.label_span,
            self.address,
            self.pc,
        )
    }

    pub fn address_to_index(&mut self) -> Result<u32, Bailed> {
        address_to_index(
            self.result_builder,
            self.instruction_span,
            self.label_span,
            self.address,
            self.pc,
        )
    }
}

fn address_to_offset(
    result_builder: &mut RichResultBuilder,
    instruction_span: &Span,
    address_span: &Span,
    address: Address,
    pc: Address,
) -> Result<i16, Bailed> {
    let offset = (address as i32 - pc as i32) / 4 - 1;
    offset.try_into().map_err(map_err!(
        result_builder,
        RichError::new(AssembleError::OffsetTooLarge, instruction_span.clone())
            .with_narrow_span(address_span.clone())
            .with_note("can only branch by -128..128 KiB at a time"),
    ))
}

fn address_to_index(
    result_builder: &mut RichResultBuilder,
    instruction_span: &Span,
    address_span: &Span,
    address: Address,
    pc: Address,
) -> Result<u32, Bailed> {
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
    let Some(pc_plus_4) = pc.checked_add(4) else {
        return result_builder.bail(
            RichError::new(
                AssembleError::ProgramCounterOverflow,
                instruction_span.clone(),
            )
            .with_narrow_span(address_span.clone()),
        );
    };
    if (address ^ pc_plus_4) & 0xf0000000 == 0 {
        Ok((address & 0x0fffffff) >> 2)
    } else {
        result_builder.bail(
            RichError::new(AssembleError::JumpTooLarge, instruction_span.clone())
                .with_narrow_span(address_span.clone())
                .with_note("jumps can only reach addresses within the current 256 MiB block")
                .with_help("consider chaining multiple jump instructions to reach your target"),
        )
    }
}
