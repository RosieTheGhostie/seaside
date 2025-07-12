use crate::{
    directives::StringDirective, error::AssembleError, parser::Value, string_builder::StringBuilder,
};
use seaside_error::rich::{Label, RichError, RichResult, Span};
use seaside_int_utils::Endian;
use seaside_type_aliases::Address;
use std::{fs::write, path::PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct SegmentBuildInfo {
    pub base: Address,
    pub next: Address,
    bytes: Vec<u8>,
}

impl SegmentBuildInfo {
    pub fn new(base: Address) -> Self {
        Self {
            base,
            next: base,
            bytes: vec![],
        }
    }

    pub fn export(self, path: PathBuf) -> std::io::Result<()> {
        write(path, &self.bytes)
    }

    pub fn jump_ahead_to(&mut self, expr_span: Span, address: Address) -> RichResult<()> {
        match address.checked_sub(self.next) {
            Some(n) => {
                self.jump_ahead_by(n);
                Ok(())
            }
            None => Err(RichError::new(AssembleError::JumpBehind, expr_span)
                .with_note(format!("next available address is {:#010x}", self.next))),
        }
    }

    pub fn jump_ahead_by(&mut self, n: u32) {
        self.next += n;
        let mut nuls = vec![0u8; n as usize];
        self.bytes.append(&mut nuls);
    }

    pub fn append(&mut self, bytes: &mut Vec<u8>) {
        self.next += bytes.len() as Address;
        self.bytes.append(bytes);
    }

    pub fn append_i8(&mut self, expr_span: Span, values: Vec<(Value, Span)>) -> RichResult<()> {
        let n_bytes = values.len();
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            match value {
                Value::Int(byte @ -0x80..=0x7f) => self.bytes.push(byte as u8),
                Value::Int(_) | Value::Float(_) => {
                    return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                        .with_label(Label::new(span).with_message("expected i8")));
                }
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
        let n_bytes = values.len() << 1;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let half = match value {
                Value::Int(half @ -0x8000..=0x7fff) => half as i16,
                Value::Int(_) | Value::Float(_) => {
                    return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                        .with_label(Label::new(span).with_message("expected i16")));
                }
            };
            let bytes = match endian {
                Endian::Little => half.to_le_bytes(),
                Endian::Big => half.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
        Ok(())
    }

    pub fn append_i32(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        let n_bytes = values.len() << 2;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let word = match value {
                Value::Int(word @ -0x8000_0000..=0x7fff_ffff) => word as i32,
                Value::Int(_) | Value::Float(_) => {
                    return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                        .with_label(Label::new(span).with_message("expected i32")));
                }
            };
            let bytes = match endian {
                Endian::Little => word.to_le_bytes(),
                Endian::Big => word.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
        Ok(())
    }

    pub fn append_f32(
        &mut self,
        expr_span: Span,
        values: Vec<(Value, Span)>,
        endian: Endian,
    ) -> RichResult<()> {
        const F32_MIN: f64 = f32::MIN as f64;
        const F32_MAX: f64 = f32::MAX as f64;

        let n_bytes = values.len() << 2;
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let float = match value {
                Value::Float(float @ F32_MIN..=F32_MAX) => float as f32,
                Value::Int(_) | Value::Float(_) => {
                    return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                        .with_label(Label::new(span).with_message("expected f32")));
                }
            };
            let bytes = match endian {
                Endian::Little => float.to_le_bytes(),
                Endian::Big => float.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
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
        self.next += n_bytes as Address;
        self.bytes.reserve(n_bytes);
        for (value, span) in values {
            let double = match value {
                Value::Float(double) => double,
                Value::Int(_) => {
                    return Err(RichError::new(AssembleError::WrongType, expr_span.clone())
                        .with_label(Label::new(span).with_message("expected f64")));
                }
            };
            let bytes = match endian {
                Endian::Little => double.to_le_bytes(),
                Endian::Big => double.to_be_bytes(),
            };
            for byte in bytes {
                self.bytes.push(byte);
            }
        }
        Ok(())
    }

    pub fn overwrite_u32(&mut self, address: Address, word: u32, endian: Endian) {
        let index = (address - self.base) as usize;
        let old_bytes = self.bytes.get_mut(index..index + 4).unwrap();
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
        let divisor = (1 << alignment) as u32;
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
            let mut buffer: [u8; 4] = [0; 4];
            c.encode_utf8(&mut buffer);
            let n_bytes = c.len_utf8();

            self.bytes.reserve(n_bytes);
            self.next += n_bytes as Address;
            for &byte in &buffer[0..n_bytes] {
                self.bytes.push(byte);
            }
        }
        if matches!(directive, StringDirective::Asciiz) {
            self.next += 1;
            self.bytes.push(b'\0');
        }
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
