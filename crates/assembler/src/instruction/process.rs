use crate::{error::AssembleError, parser::Operand};
use seaside_constants::{
    ConditionCode,
    register::{Coprocessor0Register, CpuRegister, FpuRegister},
};
use seaside_error::rich::{Label, RichError, RichResult, Span};
use seaside_type_aliases::Address;

pub enum Destination<'src> {
    Address(u32, Span),
    Label(&'src str, Span),
}

// === Adapters ===

/// Attempts to process `operand` with `processor`, but does not treat absence of an operand as an
/// error.
pub fn maybe<F, T>(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
    processor: F,
) -> RichResult<Option<T>>
where
    F: FnOnce(Option<&(Operand<'_>, Span)>, &Span) -> RichResult<T>,
{
    if operand.is_some() {
        processor(operand, expr_span).map(Option::Some)
    } else {
        Ok(None)
    }
}

/// Attempts to process `operand` with `processor`, but if the operand is [`None`], yields a default
/// value instead of an error.
pub fn maybe_or<F, T>(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
    default: T,
    processor: F,
) -> RichResult<T>
where
    F: FnOnce(Option<&(Operand<'_>, Span)>, &Span) -> RichResult<T>,
{
    if operand.is_some() {
        processor(operand, expr_span)
    } else {
        Ok(default)
    }
}

// === Processors ===

/// Attempts to process `operand` as a CPU register.
///
/// As with all register processors, this will work with both named or indexed registers.
pub fn cpu_register(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
) -> RichResult<CpuRegister> {
    const MESSAGE: &str = "expected CPU register";
    let wrong_type_error = |span: &Span| new_error(expr_span.clone(), span.clone(), MESSAGE);
    match operand {
        Some((Operand::Register(register), span)) => {
            CpuRegister::parse_indexed(register).map_err(|_| wrong_type_error(span))
        }
        Some((_, span)) => Err(wrong_type_error(span)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a wrapped CPU register.
///
/// As with all register processors, this will work with both named or indexed registers.
pub fn wrapped_cpu_register(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
) -> RichResult<CpuRegister> {
    const MESSAGE: &str = "expected wrapped CPU register";
    let wrong_type_error = |span: &Span| new_error(expr_span.clone(), span.clone(), MESSAGE);
    match operand {
        Some((Operand::WrappedRegister(register), span)) => {
            CpuRegister::parse_indexed(register).map_err(|_| wrong_type_error(span))
        }
        Some((_, span)) => Err(wrong_type_error(span)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

pub fn offset_cpu_register(
    [operand_a, operand_b]: [Option<&(Operand<'_>, Span)>; 2],
    expr_span: &Span,
) -> RichResult<(i16, CpuRegister)> {
    const MESSAGE: &str = "expected i16 or wrapped CPU register";
    let wrong_type_error = |span: &Span| new_error(expr_span.clone(), span.clone(), MESSAGE);
    match operand_a {
        Some((Operand::Int(imm @ -0x8000..0x8000), _)) => {
            Ok((*imm as i16, wrapped_cpu_register(operand_b, expr_span)?))
        }
        Some((Operand::WrappedRegister(register), span)) => Ok((
            0,
            CpuRegister::parse_indexed(register).map_err(|_| wrong_type_error(span))?,
        )),
        Some((_, span)) => Err(wrong_type_error(span)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as an FPU register.
///
/// As with all register processors, this will work with both named or indexed registers.
pub fn fpu_register(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
) -> RichResult<FpuRegister> {
    const MESSAGE: &str = "expected FPU register";
    let wrong_type_error = |span: &Span| new_error(expr_span.clone(), span.clone(), MESSAGE);
    match operand {
        Some((Operand::Register(register), span)) => {
            FpuRegister::parse_indexed(register).map_err(|_| wrong_type_error(span))
        }
        Some((_, span)) => Err(wrong_type_error(span)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a coprocessor 0 register.
///
/// As with all register processors, this will work with both named or indexed registers.
pub fn coprocessor_0_register(
    operand: Option<&(Operand<'_>, Span)>,
    expr_span: &Span,
) -> RichResult<Coprocessor0Register> {
    const MESSAGE: &str = "expected coprocessor 0 register";
    let wrong_type_error = |span: &Span| new_error(expr_span.clone(), span.clone(), MESSAGE);
    match operand {
        Some((Operand::Register(register), span)) => {
            register.parse().map_err(|_| wrong_type_error(span))
        }
        Some((_, span)) => Err(wrong_type_error(span)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a shift amount.
pub fn shamt(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<u8> {
    const MESSAGE: &str = "expected shift amount (0..32)";
    match operand {
        Some((Operand::Int(shamt @ 0..32), _)) => Ok(*shamt as u8),
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a [destination](Destination).
pub fn destination<'src>(
    operand: Option<&(Operand<'src>, Span)>,
    expr_span: &Span,
) -> RichResult<Destination<'src>> {
    const MESSAGE: &str = "expected address or label";
    match operand {
        Some((Operand::Int(address @ 0..0xffff_ffff), span)) => {
            Ok(Destination::Address(*address as Address, span.clone()))
        }
        Some((Operand::Label(label), span)) => Ok(Destination::Label(label, span.clone())),
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as an unsigned 16 bit immediate.
pub fn imm_u16(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<u16> {
    const MESSAGE: &str = "expected u16 (0..65536)";
    match operand {
        Some((Operand::Int(imm @ 0..=0xffff), _)) => Ok(*imm as u16),
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a signed 16 bit immediate.
pub fn imm_i16(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<i16> {
    const MESSAGE: &str = "expected i16 (-32768..32768)";
    match operand {
        Some((Operand::Int(imm @ -0x8000..0x8000), _)) => Ok(*imm as i16),
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// The message used by [`cc`] and [`maybe_cc`] in the event of a
/// [`WrongType`](AssembleError::WrongType) error.
const CC_MESSAGE: &str = "expected cc index (0..8)";

/// Attempts to process `operand` as a condition code.
pub fn cc(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<ConditionCode> {
    match operand {
        Some((Operand::Int(cc @ 0..8), _)) => {
            Ok(unsafe { ConditionCode::from_u8_unchecked(*cc as _) })
        }
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), CC_MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a [condition code](ConditionCode), yielding the
/// [default](Default) code if it doesn't find one.
///
/// The [`Ok`] value is the condition code along with the "next" operand. If a condition code was
/// found, the next operand is the operand after that; otherwise, it is whatever it found instead.
///
/// This is intended for use in situations where `cc` is not meant to be the last operand, such as
/// in the FPU comparison instructions.
pub fn maybe_cc<'a, 'src: 'a, I>(
    operands_iter: &mut I,
    expr_span: &Span,
) -> RichResult<(ConditionCode, Option<&'a (Operand<'src>, Span)>)>
where
    I: Iterator<Item = &'a (Operand<'src>, Span)>,
{
    match operands_iter.next() {
        Some((Operand::Int(cc @ 0..8), _)) => Ok((
            unsafe { ConditionCode::from_u8_unchecked(*cc as _) },
            operands_iter.next(),
        )),
        Some((Operand::Int(_), span)) => {
            Err(new_error(expr_span.clone(), span.clone(), CC_MESSAGE))
        }
        next @ Some(_) => Ok((ConditionCode::default(), next)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Attempts to process `operand` as a 20 bit error code.
///
/// This code is used exclusively for the [`break`](seaside_constants::fn_codes::SpecialFn::Break)
/// instruction.
pub fn code(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<u32> {
    const MESSAGE: &str = "expected 20 bit code";
    const UPPER_BOUND: i64 = 1 << 20;
    match operand {
        Some((Operand::Int(code @ 0..UPPER_BOUND), _)) => Ok(*code as u32),
        Some((_, span)) => Err(new_error(expr_span.clone(), span.clone(), MESSAGE)),
        None => Err(RichError::new(
            AssembleError::NotEnoughOperands,
            expr_span.clone(),
        )),
    }
}

/// Asserts that no more operands are left to process.
pub fn finish(operand: Option<&(Operand<'_>, Span)>, expr_span: &Span) -> RichResult<()> {
    if let Some((_, span)) = operand {
        Err(
            RichError::new(AssembleError::TooManyOperands, expr_span.clone())
                .with_narrow_span(span.start..expr_span.end),
        )
    } else {
        Ok(())
    }
}

/// Creates a new [`WrongType`](AssembleError::WrongType) error for use in processor functions.
pub fn new_error(expr_span: Span, span: Span, message: &str) -> RichError {
    RichError::new(AssembleError::WrongType, expr_span)
        .with_label(Label::new(span).with_message(message))
}
