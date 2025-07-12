use crate::error::AssembleError;
use core::str::CharIndices;
use seaside_error::rich::{RichError, RichResult, Span};

pub struct StringBuilder<'src> {
    raw: CharIndices<'src>,
    span: Span,
    peeked: Option<(usize, char)>,
}

impl<'src> StringBuilder<'src> {
    pub fn new(raw: &'src str, span: Span) -> Self {
        Self {
            raw: raw.char_indices(),
            span,
            peeked: None,
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        self.peeked.take().or_else(|| self.raw.next())
    }

    fn parse_escape_sequence(&mut self, start_index: usize) -> RichResult<char> {
        match self.raw.next() {
            Some((_, '\'')) => Ok('\''),
            Some((_, '"')) => Ok('"'),
            Some((_, '?')) => Ok('?'),
            Some((_, '\\')) => Ok('\\'),
            Some((_, 'a')) => Ok('\x07'), // audible bell
            Some((_, 'b')) => Ok('\x08'), // backspace
            Some((_, 'f')) => Ok('\x0c'), // form feed
            Some((_, 'n')) => Ok('\n'),   // line feed
            Some((_, 'r')) => Ok('\r'),   // carriage return
            Some((_, 't')) => Ok('\t'),   // horizontal tab
            Some((_, 'u')) => self.parse_hex_escape(start_index, 4),
            Some((_, 'U')) => self.parse_hex_escape(start_index, 8),
            Some((_, 'v')) => Ok('\x0b'), // vertical tab
            Some((_, 'x')) => self.parse_hex_escape(start_index, 2),
            Some((_, digit @ '0'..='7')) => Ok(self.parse_octal_escape(from_octal_digit(digit))),
            Some((i, _)) => Err(RichError::new(
                AssembleError::InvalidEscapeSequence,
                self.span.clone(),
            )
            .with_narrow_span(start_index..(i + 1))),
            None => Err(RichError::new(
                AssembleError::UnterminatedStringLiteral,
                self.span.clone(),
            )),
        }
    }

    fn parse_octal_escape(&mut self, mut n: u8) -> char {
        for _ in 0..2 {
            match self.raw.next() {
                Some((_, digit @ '0'..='7')) => n = (n << 3) + from_octal_digit(digit),
                Some(c) => {
                    self.peeked = Some(c);
                    break;
                }
                None => break,
            }
        }
        n as char
    }

    fn parse_hex_escape(&mut self, start_index: usize, length: usize) -> RichResult<char> {
        let mut n: u32 = 0;
        let mut end_index: usize = start_index + 2;
        for _ in 0..length {
            if let Some((i, c)) = self.raw.next() {
                end_index = i + 1;
                n <<= 4;
                n += try_from_hex_digit(c).ok_or_else(|| {
                    RichError::new(AssembleError::InvalidEscapeSequence, self.span.clone())
                        .with_narrow_span(start_index..end_index)
                })?;
            } else {
                return Err(RichError::new(
                    AssembleError::InvalidEscapeSequence,
                    self.span.clone(),
                )
                .with_narrow_span(start_index..end_index));
            }
        }
        char::from_u32(n).ok_or_else(|| {
            RichError::new(AssembleError::InvalidUtf8, self.span.clone())
                .with_narrow_span(start_index..end_index)
        })
    }
}

impl Iterator for StringBuilder<'_> {
    type Item = RichResult<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let (next_index, next_raw_char) = self.next_char()?;
        Some(if next_raw_char == '\\' {
            self.parse_escape_sequence(next_index)
        } else {
            Ok(next_raw_char)
        })
    }
}

const fn from_octal_digit(digit: char) -> u8 {
    (digit as u32 - '0' as u32) as u8
}

const fn try_from_hex_digit(digit: char) -> Option<u32> {
    match digit {
        '0'..='9' => Some(digit as u32 - '0' as u32),
        'A'..='F' => Some(digit as u32 - 'A' as u32 + 0xa),
        'a'..='f' => Some(digit as u32 - 'a' as u32 + 0xa),
        _ => None,
    }
}
