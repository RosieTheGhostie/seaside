use const_format::formatcp;
use seaside_core::{
    instruction::{
        coprocessor_0, coprocessor_1, coprocessor_1x, coprocessor_2, immediate, special, special_2,
    },
    prelude::*,
    u3, u5,
};
use seaside_rich_error::{Label, RichError, RichResultBuilder, Span, result::Bailed};

use crate::{error::AssembleError, options::Options, parser::Operand};

pub struct Processor<'a, 'src> {
    pub(super) result_builder: &'a mut RichResultBuilder,
    options: &'a Options,

    pub(super) expr_span: &'a Span,
    operator_span: Span,
    operands_iter: &'a mut dyn Iterator<Item = &'a (Operand<'src>, Span)>,
    peeked: Vec<&'a (Operand<'src>, Span)>,
}

impl<'a, 'src> Processor<'a, 'src> {
    #![allow(dead_code)]

    pub const fn new(
        result_builder: &'a mut RichResultBuilder,
        options: &'a Options,
        expr_span: &'a Span,
        operator_span: Span,
        operands_iter: &'a mut dyn Iterator<Item = &'a (Operand<'src>, Span)>,
    ) -> Self {
        Self {
            result_builder,
            options,
            expr_span,
            operator_span,
            operands_iter,
            peeked: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Option<&'a (Operand<'src>, Span)> {
        if let Some(&last_peeked) = self.peeked.last() {
            Some(last_peeked)
        } else if let Some(operand) = self.operands_iter.next() {
            self.peeked.push(operand);
            Some(operand)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<&'a (Operand<'src>, Span)> {
        self.peeked.pop().or_else(|| self.operands_iter.next())
    }

    pub fn cpu_register(&mut self) -> Result<CpuRegister, Bailed> {
        self.must_have(Self::_cpu_register)
    }

    pub fn maybe_cpu_register(&mut self) -> Result<Option<CpuRegister>, Bailed> {
        self.maybe(Self::_cpu_register)
    }

    fn _cpu_register(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<CpuRegister, Bailed> {
        const MESSAGE: &str = "expected CPU register";

        if let Operand::Register(register) = operand
            && let Ok(register) = CpuRegister::parse_indexed(register)
        {
            self.warn_asm_temp(register, operand_span);
            Ok(register)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn wrapped_cpu_register(&mut self) -> Result<CpuRegister, Bailed> {
        self.must_have(Self::_wrapped_cpu_register)
    }

    pub fn maybe_wrapped_cpu_register(&mut self) -> Result<Option<CpuRegister>, Bailed> {
        self.maybe(Self::_wrapped_cpu_register)
    }

    fn _wrapped_cpu_register(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<CpuRegister, Bailed> {
        const MESSAGE: &str = "expected wrapped CPU register";

        if let Operand::WrappedRegister(register) = operand
            && let Ok(register) = CpuRegister::parse_indexed(register)
        {
            self.warn_asm_temp(register, operand_span);
            Ok(register)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn offset_cpu_register(&mut self) -> Result<(i16, CpuRegister), Bailed> {
        self.must_have(Self::_offset_cpu_register)
    }

    pub fn maybe_offset_cpu_register(&mut self) -> Result<Option<(i16, CpuRegister)>, Bailed> {
        self.maybe(Self::_offset_cpu_register)
    }

    fn _offset_cpu_register(
        &mut self,
        (operand_a, operand_a_span): &(Operand<'_>, Span),
    ) -> Result<(i16, CpuRegister), Bailed> {
        const MESSAGE: &str = "expected an optional i16 followed by a wrapped CPU register";

        if let Operand::Int(imm @ -0x8000..0x8000) = operand_a {
            let Some(operand_b) = self.pop() else {
                return self.result_builder.bail(self.not_enough_operands_error());
            };

            self._wrapped_cpu_register(operand_b)
                .map(|register| (*imm as i16, register))
        } else if let Operand::WrappedRegister(register) = operand_a
            && let Ok(register) = CpuRegister::parse_indexed(register)
        {
            Ok((0, register))
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_a_span, MESSAGE))
        }
    }

    fn warn_asm_temp(&mut self, register: CpuRegister, span: &Span) {
        if register == CpuRegister::AsmTemp && !self.options.explicit_asm_temp {
            self.result_builder.add_error(
                RichError::new_warning(AssembleError::ExplicitAsmTemp, self.expr_span.clone())
                    .with_narrow_span(span.clone()),
            )
        }
    }

    pub fn coprocessor_0_register(&mut self) -> Result<Coprocessor0Register, Bailed> {
        self.must_have(Self::_coprocessor_0_register)
    }

    pub fn maybe_coprocessor_0_register(&mut self) -> Result<Option<Coprocessor0Register>, Bailed> {
        self.maybe(Self::_coprocessor_0_register)
    }

    fn _coprocessor_0_register(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<Coprocessor0Register, Bailed> {
        const MESSAGE: &str = "expected coprocessor 0 register";

        if let Operand::Register(register) = operand
            && let Ok(register) = register.parse()
        {
            Ok(register)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn coprocessor_1_register(&mut self) -> Result<FpuRegister, Bailed> {
        self.must_have(Self::_coprocessor_1_register)
    }

    pub fn maybe_coprocessor_1_register(&mut self) -> Result<Option<FpuRegister>, Bailed> {
        self.maybe(Self::_coprocessor_1_register)
    }

    fn _coprocessor_1_register(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<FpuRegister, Bailed> {
        const MESSAGE: &str = "expected coprocessor 1 register";

        if let Operand::Register(register) = operand
            && let Ok(register) = FpuRegister::parse_indexed(register)
        {
            Ok(register)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn shift_amount(&mut self) -> Result<u5, Bailed> {
        self.must_have(Self::_shift_amount)
    }

    pub fn maybe_shift_amount(&mut self) -> Result<Option<u5>, Bailed> {
        self.maybe(Self::_shift_amount)
    }

    fn _shift_amount(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<u5, Bailed> {
        const MIN_SHAMT: i64 = u5::MIN.as_i64();
        const MAX_SHAMT: i64 = u5::MAX.as_i64();
        const MESSAGE: &str = formatcp!("expected shift amount ({MIN_SHAMT}..={MAX_SHAMT})");

        if let Operand::Int(shamt @ MIN_SHAMT..=MAX_SHAMT) = operand {
            // SAFETY: `u5::MIN <= shamt` and `shamt <= u5::MAX`.
            Ok(unsafe { u5::new_unchecked(*shamt as _) })
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn destination(&mut self) -> Result<Destination<'src>, Bailed> {
        self.must_have(Self::_destination)
    }

    pub fn maybe_destination(&mut self) -> Result<Option<Destination<'src>>, Bailed> {
        self.maybe(Self::_destination)
    }

    fn _destination(
        &mut self,
        (operand, operand_span): &(Operand<'src>, Span),
    ) -> Result<Destination<'src>, Bailed> {
        const MIN_ADDRESS: i64 = Address::MIN as _;
        const MAX_ADDRESS: i64 = Address::MAX as _;
        const MESSAGE: &str = "expected address or label";

        if let Operand::Int(address @ MIN_ADDRESS..=MAX_ADDRESS) = operand {
            Ok(Destination::Address(
                *address as Address,
                operand_span.clone(),
            ))
        } else if let Operand::Label(label) = operand {
            Ok(Destination::Label(label, operand_span.clone()))
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn unsigned_immediate(&mut self) -> Result<u16, Bailed> {
        self.must_have(Self::_unsigned_immediate)
    }

    pub fn maybe_unsigned_immediate(&mut self) -> Result<Option<u16>, Bailed> {
        self.maybe(Self::_unsigned_immediate)
    }

    fn _unsigned_immediate(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<u16, Bailed> {
        const MIN_IMM: i64 = u16::MIN as _;
        const MAX_IMM: i64 = u16::MAX as _;
        const MESSAGE: &str = formatcp!("expected u16 ({MIN_IMM}..={MAX_IMM})");

        if let Operand::Int(imm @ MIN_IMM..=MAX_IMM) = operand {
            Ok(*imm as _)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn signed_immediate(&mut self) -> Result<i16, Bailed> {
        self.must_have(Self::_signed_immediate)
    }

    pub fn maybe_signed_immediate(&mut self) -> Result<Option<i16>, Bailed> {
        self.maybe(Self::_signed_immediate)
    }

    fn _signed_immediate(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<i16, Bailed> {
        const MIN_IMM: i64 = i16::MIN as _;
        const MAX_IMM: i64 = i16::MAX as _;
        const MESSAGE: &str = formatcp!("expected i16 ({MIN_IMM}..={MAX_IMM})");

        if let Operand::Int(imm @ MIN_IMM..=MAX_IMM) = operand {
            Ok(*imm as _)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn condition_code(&mut self) -> Result<ConditionCode, Bailed> {
        self.must_have(Self::_condition_code)
    }

    pub fn maybe_condition_code(&mut self) -> Result<Option<ConditionCode>, Bailed> {
        self.maybe(Self::_condition_code)
    }

    /// Attempts to process the next operand as a [condition code](ConditionCode) only if it happens
    /// to be an integer.
    ///
    /// If the next operand is not an integer, it will not be consumed.
    ///
    /// This is intended for use in situations where `cc` is not meant to be the last operand, such
    /// as in the coprocessor 1 comparison instructions.
    pub fn maybe_condition_code_if_int(&mut self) -> Result<Option<ConditionCode>, Bailed> {
        self.maybe_if(Self::_condition_code, |operand| {
            matches!(operand, Operand::Int(_))
        })
    }

    fn _condition_code(
        &mut self,
        (operand, operand_span): &(Operand<'_>, Span),
    ) -> Result<ConditionCode, Bailed> {
        const MIN_INDEX: i64 = u3::MIN.as_i64();
        const MAX_INDEX: i64 = u3::MAX.as_i64();
        const MESSAGE: &str = formatcp!("expected cc index ({MIN_INDEX}..={MAX_INDEX})");

        if let Operand::Int(cc @ MIN_INDEX..=MAX_INDEX) = operand {
            // SAFETY: `MIN_INDEX <= cc` and `cc <= MAX_INDEX`.
            Ok(ConditionCode(unsafe { u3::new_unchecked(*cc as _) }))
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn code(&mut self) -> Result<u32, Bailed> {
        self.must_have(Self::_code)
    }

    pub fn maybe_code(&mut self) -> Result<Option<u32>, Bailed> {
        self.maybe(Self::_code)
    }

    fn _code(&mut self, (operand, operand_span): &(Operand<'_>, Span)) -> Result<u32, Bailed> {
        const MIN_CODE: i64 = 0;
        const MAX_CODE: i64 = (1 << 20) - 1;
        const MESSAGE: &str = "expected 20 bit code";

        if let Operand::Int(code @ MIN_CODE..=MAX_CODE) = operand {
            Ok(*code as _)
        } else {
            self.result_builder
                .bail(self.wrong_type_error(operand_span, MESSAGE))
        }
    }

    pub fn finish(mut self) -> Result<(), Bailed> {
        match self.pop() {
            None => Ok(()),
            Some((_, span)) => {
                return self.result_builder.bail(
                    RichError::new(AssembleError::TooManyOperands, self.expr_span.clone())
                        .with_narrow_span(span.start..self.expr_span.end),
                );
            }
        }
    }

    fn must_have<T, F>(&mut self, method: F) -> Result<T, Bailed>
    where
        F: FnOnce(&mut Self, &'a (Operand<'src>, Span)) -> Result<T, Bailed>,
    {
        if let Some(operand) = self.pop() {
            method(self, operand)
        } else {
            self.result_builder.bail(self.not_enough_operands_error())
        }
    }

    fn maybe<T, F>(&mut self, method: F) -> Result<Option<T>, Bailed>
    where
        F: FnOnce(&mut Self, &'a (Operand<'src>, Span)) -> Result<T, Bailed>,
    {
        if let Some(operand) = self.peek() {
            method(self, operand).map(|ok| {
                self.pop();
                Some(ok)
            })
        } else {
            Ok(None)
        }
    }

    fn maybe_if<T, F1, F2>(&mut self, method: F1, predicate: F2) -> Result<Option<T>, Bailed>
    where
        F1: FnOnce(&mut Self, &'a (Operand<'src>, Span)) -> Result<T, Bailed>,
        F2: FnOnce(&Operand<'src>) -> bool,
    {
        if let Some(operand) = self.peek()
            && predicate(&operand.0)
        {
            self.pop();
            method(self, operand).map(Some)
        } else {
            Ok(None)
        }
    }

    fn wrong_type_error(&self, operand_span: &Span, message: &str) -> RichError {
        RichError::new(AssembleError::WrongType, self.expr_span.clone())
            .with_label(Label::new(operand_span.clone()).with_message(message))
    }

    fn not_enough_operands_error(&self) -> RichError {
        RichError::new(AssembleError::NotEnoughOperands, self.expr_span.clone())
            .with_narrow_span(self.operator_span.clone())
    }
}

pub enum Destination<'src> {
    Address(Address, Span),
    Label(&'src str, Span),
}

/// ```mips
/// sll $rd, $rt, shamt
/// ```
pub fn shift_by_amount(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.shamt = processor.shift_amount()?;

    Ok(())
}

/// ```mips
/// sllv $rd, $rt, $rs
/// ```
pub fn shift_by_variable(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// movt $rd, $rs
/// movt $rd, $rs, cc
/// ```
pub fn cpu_move_conditional(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.set_cc(processor.maybe_condition_code()?.unwrap_or_default());

    Ok(())
}

/// ```mips
/// movt.s $fd, $fs
/// movt.s $fd, $fs, cc
/// ```
pub fn coprocessor_1_move_conditional(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::NormalFields,
) -> Result<(), Bailed> {
    fields.fd = processor.coprocessor_1_register()?.to_indexed();
    fields.fs = processor.coprocessor_1_register()?.to_indexed();
    fields.set_cc_in_ft(processor.maybe_condition_code()?.unwrap_or_default());

    Ok(())
}

/// ```mips
/// mfhi $rd
/// ```
pub fn move_from_high_or_low(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// mfc0 $rt, $rd # note: $rd is a coprocessor 0 register
/// ```
pub fn move_between_cpu_and_coprocessor_0(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_0::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.rd = processor.coprocessor_0_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// mfc1 $rt, $fs
/// ```
pub fn move_between_cpu_and_coprocessor_1(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::RegisterImmediateFields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.set_fs(processor.coprocessor_0_register()?.to_indexed());

    Ok(())
}

/// ```mips
/// mfc2 $rt, $rd # note: $rd is a coprocessor 2 register
/// ```
pub fn move_between_cpu_and_coprocessor_2(
    processor: &mut Processor<'_, '_>,
    _fields: &mut coprocessor_2::Fields,
) -> Result<(), Bailed> {
    processor.result_builder.bail(
        RichError::new(AssembleError::UnknownOperator, processor.expr_span.clone()).with_note(
            "seaside recognizes a few coprocessor 2 operators, but they aren't supported",
        ),
    )
}

/// ```mips
/// movz.s $fd, $fs, $rt
/// ```
pub fn coprocessor_1_move_by_comparison(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::NormalFields,
) -> Result<(), Bailed> {
    fields.fd = processor.coprocessor_1_register()?.to_indexed();
    fields.fs = processor.coprocessor_1_register()?.to_indexed();
    fields.ft = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// lb $rt, ($rs)
/// lb $rt, imm_i16($rs)
/// ```
pub fn cpu_memory_access(
    processor: &mut Processor<'_, '_>,
    fields: &mut immediate::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    let (imm, rs) = processor.offset_cpu_register()?;
    fields.imm = imm as _;
    fields.rs = rs.to_indexed();

    Ok(())
}

/// ```mips
/// lwc1 $ft, ($rs)
/// lwc1 $ft, imm_i16($rs)
/// ```
pub fn coprocessor_1_memory_access(
    processor: &mut Processor<'_, '_>,
    fields: &mut immediate::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.coprocessor_1_register()?.to_indexed();
    let (imm, rs) = processor.offset_cpu_register()?;
    fields.imm = imm as _;
    fields.rs = rs.to_indexed();

    Ok(())
}

/// ```mips
/// jr $rs
/// ```
pub fn jr(processor: &mut Processor<'_, '_>, fields: &mut special::Fields) -> Result<(), Bailed> {
    fields.rs = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// jalr $rs
/// jalr $rd, $rs
/// ```
pub fn jalr(processor: &mut Processor<'_, '_>, fields: &mut special::Fields) -> Result<(), Bailed> {
    let rs_or_rd = processor.cpu_register()?;
    let (rs, rd) = match processor.maybe_cpu_register()? {
        Some(rs) => (rs, rs_or_rd),
        None => (rs_or_rd, CpuRegister::ReturnAddr),
    };

    fields.rs = rs.to_indexed();
    fields.rd = rd.to_indexed();

    Ok(())
}

/// ```mips
/// break
/// break code
/// ```
pub fn r#break(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    if let Some(code) = processor.maybe_code()? {
        fields.set_code(code);
    }

    Ok(())
}

/// ```mips
/// lui $rt, imm_u16
/// ```
pub fn lui(
    processor: &mut Processor<'_, '_>,
    fields: &mut immediate::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.imm = processor.unsigned_immediate()? as _;

    Ok(())
}

/// ```mips
/// clz $rd, $rs
/// ```
pub fn count_leading_bits(
    processor: &mut Processor<'_, '_>,
    fields: &mut special_2::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// addi $rt, $rs, imm_i16
/// ```
pub fn int_math_signed_immediate(
    processor: &mut Processor<'_, '_>,
    fields: &mut immediate::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.imm = processor.signed_immediate()? as _;

    Ok(())
}

/// ```mips
/// andi $rt, $rs, imm_u16
/// ```
pub fn int_math_unsigned_immediate(
    processor: &mut Processor<'_, '_>,
    fields: &mut immediate::Fields,
) -> Result<(), Bailed> {
    fields.rt = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.imm = processor.unsigned_immediate()? as _;

    Ok(())
}

/// ```mips
/// mult $rs, $rt
/// ```
pub fn int_math_2_ops(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// movz $rd, $rs, $rt
/// ```
pub fn int_math_3_ops(
    processor: &mut Processor<'_, '_>,
    fields: &mut special::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// madd $rs, $rt
/// ```
pub fn int_multiply_accumulate(
    processor: &mut Processor<'_, '_>,
    fields: &mut special_2::Fields,
) -> Result<(), Bailed> {
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// mul $rd, $rs, $rt
/// ```
pub fn mul(
    processor: &mut Processor<'_, '_>,
    fields: &mut special_2::Fields,
) -> Result<(), Bailed> {
    fields.rd = processor.cpu_register()?.to_indexed();
    fields.rs = processor.cpu_register()?.to_indexed();
    fields.rt = processor.cpu_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// sqrt.s $fd, $fs
/// ```
pub fn float_math_2_ops(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::NormalFields,
) -> Result<(), Bailed> {
    fields.fd = processor.coprocessor_1_register()?.to_indexed();
    fields.fs = processor.coprocessor_1_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// add.s $fd, $fs, $ft
/// ```
pub fn float_math_3_ops(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::NormalFields,
) -> Result<(), Bailed> {
    fields.fd = processor.coprocessor_1_register()?.to_indexed();
    fields.fs = processor.coprocessor_1_register()?.to_indexed();
    fields.ft = processor.coprocessor_1_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// madd.s $fd, $fr, $fs, $ft
/// ```
pub fn float_multiply_accumulate(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1x::Fields,
) -> Result<(), Bailed> {
    fields.fd = processor.coprocessor_1_register()?.to_indexed();
    fields.fr = processor.coprocessor_1_register()?.to_indexed();
    fields.fs = processor.coprocessor_1_register()?.to_indexed();
    fields.ft = processor.coprocessor_1_register()?.to_indexed();

    Ok(())
}

/// ```mips
/// c.eq.s $fs, $ft
/// c.eq.s cc, $fs, $ft
/// ```
pub fn float_compare(
    processor: &mut Processor<'_, '_>,
    fields: &mut coprocessor_1::NormalFields,
) -> Result<(), Bailed> {
    fields.set_cc_in_fd(processor.maybe_condition_code_if_int()?.unwrap_or_default());
    fields.fs = processor.coprocessor_1_register()?.to_indexed();
    fields.ft = processor.coprocessor_1_register()?.to_indexed();

    Ok(())
}
