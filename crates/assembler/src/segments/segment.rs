use seaside_error::rich::{Label, RichError, RichResult, Span};
use seaside_int_utils::Endian;
use seaside_type_aliases::{Address, Size, UnsignedOffset};

use crate::{
    directives::StringDirective, error::AssembleError, parser::Value, string_builder::StringBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SegmentBuildInfo {
    pub base: Address,
    pub next: Address,
    bytes: Vec<u8>,
}

impl SegmentBuildInfo {
    pub const fn new(base: Address) -> Self {
        Self {
            base,
            next: base,
            bytes: Vec::new(),
        }
    }

    pub fn export(self) -> seaside_executable::Segment {
        self.bytes.into()
    }

    pub fn take_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn jump_ahead_to(&mut self, expr_span: Span, address: Address) -> RichResult<()> {
        if let Some(n) = address.checked_sub(self.next) {
            self.jump_ahead_by(n);
            Ok(())
        } else {
            Err(RichError::new(AssembleError::JumpBehind, expr_span)
                .with_note(format!("next available address is {:#010x}", self.next)))
        }
    }

    pub fn jump_ahead_by(&mut self, n: Size) {
        self.next += n;
        self.bytes.append(&mut vec![0; n as usize]);
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.next += bytes.len() as Address;
        self.bytes.append(bytes);
    }

    pub fn append_i8(&mut self, expr_span: Span, values: Vec<(Value, Span)>) -> RichResult<()> {
        const I8_MIN: i64 = i8::MIN as _;
        const I8_MAX: i64 = i8::MAX as _;

        let n_bytes = values.len();
        self.next += n_bytes as Size;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            if let Value::Int(byte @ I8_MIN..=I8_MAX) = value {
                self.bytes.push(byte as _)
            } else {
                return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                    .with_label(Label::new(span).with_message("expected i8")));
            }
        }

        Ok(())
    }

    pub fn append_i16(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        const I16_MIN: i64 = i16::MIN as _;
        const I16_MAX: i64 = i64::MAX as _;

        let n_bytes = values.len() << 1;
        self.next += n_bytes as UnsignedOffset;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let half: i16 = if let Value::Int(half @ I16_MIN..=I16_MAX) = value {
                half as _
            } else {
                return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                    .with_label(Label::new(span).with_message("expected i16")));
            };

            let bytes = match endian {
                Endian::Little => half.to_le_bytes(),
                Endian::Big => half.to_be_bytes(),
            };

            self.bytes.extend_from_slice(&bytes);
        }

        Ok(())
    }

    pub fn append_i32(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        const I32_MIN: i64 = i32::MIN as _;
        const I32_MAX: i64 = i32::MAX as _;

        let n_bytes = values.len() << 2;
        self.next += n_bytes as UnsignedOffset;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let word: i32 = if let Value::Int(word @ I32_MIN..=I32_MAX) = value {
                word as _
            } else {
                return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                    .with_label(Label::new(span).with_message("expected i32")));
            };

            let bytes = match endian {
                Endian::Little => word.to_le_bytes(),
                Endian::Big => word.to_be_bytes(),
            };

            self.bytes.extend_from_slice(&bytes);
        }

        Ok(())
    }

    pub fn append_f32(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        const F32_MIN: f64 = f32::MIN as _;
        const F32_MAX: f64 = f32::MAX as _;

        let n_bytes = values.len() << 2;
        self.next += n_bytes as UnsignedOffset;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let float: f32 = if let Value::Float(float @ F32_MIN..=F32_MAX) = value {
                float as _
            } else if let Value::Int(int) = value {
                int as _
            } else {
                return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                    .with_label(Label::new(span).with_message("expected f32")));
            };

            let bytes = match endian {
                Endian::Little => float.to_le_bytes(),
                Endian::Big => float.to_be_bytes(),
            };

            self.bytes.extend_from_slice(&bytes);
        }

        Ok(())
    }

    pub fn append_f64(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        let n_bytes = values.len() << 3;
        self.next += n_bytes as UnsignedOffset;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let double: f64 = if let Value::Float(double) = value {
                double
            } else if let Value::Int(int) = value {
                int as _
            } else {
                return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                    .with_label(Label::new(span).with_message("expected f64")));
            };

            let bytes = match endian {
                Endian::Little => double.to_le_bytes(),
                Endian::Big => double.to_be_bytes(),
            };

            self.bytes.extend_from_slice(&bytes);
        }

        Ok(())
    }

    pub fn overwrite_u32(&mut self, address: Address, word: u32, endian: Endian) {
        let index: usize = (address - self.base) as _;
        let old_bytes = self.bytes.get_mut(index..index + size_of::<u32>()).unwrap();
        let new_bytes = match endian {
            Endian::Little => word.to_le_bytes(),
            Endian::Big => word.to_be_bytes(),
        };

        old_bytes[..4].copy_from_slice(&new_bytes);
    }

    pub fn align(&mut self, alignment: u8) {
        if alignment == 0 {
            return;
        }

        let divisor: Size = (1 << alignment) as _;
        let modulus = self.next & (divisor - 1);
        if modulus != 0 {
            self.jump_ahead_by(divisor - modulus);
        }
    }

    pub fn build_string(
        &mut self,
        directive: StringDirective,
        value: &str,
        span: Span,
    ) -> RichResult<()> {
        for c in StringBuilder::new(value, span) {
            let c = c?;
            let mut buffer = [0_u8; 4];
            let n_bytes = c.encode_utf8(&mut buffer).len();

            self.bytes.reserve(n_bytes);
            self.next += n_bytes as UnsignedOffset;
            self.bytes.extend_from_slice(&buffer[..n_bytes]);
        }

        if matches!(directive, StringDirective::Asciiz) {
            self.next += 1;
            self.bytes.push(b'\0');
        }

        Ok(())
    }

    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
