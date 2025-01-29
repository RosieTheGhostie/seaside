pub mod error;
pub mod macros;
pub mod node;

pub use error::{Error, ErrorKind};
pub use node::Node;

use super::{
    directives::{DataTypeDirective, SegmentDirective},
    operation::macros::{coprocessor_0, coprocessor_1, register_immediate, special, special_2},
    BasicOperator, Operand, Token,
};
use crate::config::features::assembler::SpecialDirectives;
use logos::Lexer;
use macros::{assert_token, assert_token_or_none, get_operand, if_enabled, parse_ops};
use std::collections::VecDeque;

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    peeked: VecDeque<Token>,
    special_directives: SpecialDirectives,
}

impl Iterator for Parser<'_> {
    type Item = Result<Node, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Token = match self.next_token()? {
            Ok(token) => token,
            Err(_) => return Some(Err(Error::from(ErrorKind::UnknownToken))),
        };
        Some(match token {
            Token::NewLine => return self.next(),
            Token::BasicOperator(operator) => self.parse_instruction(operator),
            Token::Label(label) => self.parse_label(label),
            Token::SegmentDirective(directive) => self.parse_segment_header(directive),
            Token::DataTypeDirective(DataTypeDirective::Align) => self.parse_align_command(),
            Token::DataTypeDirective(DataTypeDirective::Ascii) => self.parse_string(false),
            Token::DataTypeDirective(DataTypeDirective::Asciiz) => {
                if_enabled!(self, Asciiz (".asciiz") => self.parse_string(true))
            }
            Token::DataTypeDirective(DataTypeDirective::Byte) => self.parse_byte_array(),
            Token::DataTypeDirective(DataTypeDirective::Double) => self.parse_double_array(),
            Token::DataTypeDirective(DataTypeDirective::Float) => self.parse_float_array(),
            Token::DataTypeDirective(DataTypeDirective::Half) => self.parse_half_array(),
            Token::DataTypeDirective(DataTypeDirective::Space) => self.parse_space_command(),
            Token::DataTypeDirective(DataTypeDirective::Word) => self.parse_word_array(),
            _ => Err(Error::new(
                ErrorKind::UnexpectedToken,
                "expected a NewLine, BasicOperator, Label, SegmentDirective, or DataTypeDirective",
            )),
        })
    }
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source, Token>, special_directives: SpecialDirectives) -> Self {
        Self {
            lexer,
            peeked: VecDeque::new(),
            special_directives,
        }
    }
}

impl Parser<'_> {
    fn parse_instruction(&mut self, operator: BasicOperator) -> Result<Node, Error> {
        use BasicOperator::*;
        let operands = match operator {
            special!(SystemCall) | coprocessor_0!(ErrorReturn) => parse_ops!(self),
            special![
                JumpRegister,
                MoveFromHigh,
                MoveToHigh,
                MoveFromLow,
                MoveToLow,
            ] => parse_ops!(self, gpr),
            special!(Break) => parse_ops!(self, code?),
            Jump | JumpAndLink => parse_ops!(self, label),
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
                TrapNotEqual
            ]
            | special_2![
                CountLeadingZeroes,
                CountLeadingOnes,
                MultiplyAdd,
                MultiplyAddUnsigned,
                MultiplySubtract,
                MultiplySubtractUnsigned
            ] => parse_ops!(self, gpr, gpr),
            special!(JumpAndLinkRegister) => parse_ops!(self, gpr, gpr?),
            coprocessor_0![MoveFromCoprocessor0, MoveToCoprocessor0] => parse_ops!(self, gpr, exr),
            register_immediate![
                TrapGreaterEqualImmediate,
                TrapGreaterEqualImmediateUnsigned,
                TrapLessThanImmediate,
                TrapLessThanImmediateUnsigned,
                TrapEqualImmediate,
                TrapNotEqualImmediate,
            ]
            | LoadUpperImmediate => parse_ops!(self, gpr, i16),
            register_immediate![
                BranchLessThanZero,
                BranchGreaterEqualZero,
                BranchLessThanZeroAndLink,
                BranchGreaterEqualZeroAndLink,
            ]
            | BranchLessEqualZero
            | BranchGreaterThanZero => parse_ops!(self, gpr, label),
            coprocessor_1![
                <Single | Double>
                SquareRoot,
                AbsoluteValue,
                Move,
                Negate,
                RoundWord,
                TruncateWord,
                CeilingWord,
                FloorWord,
                ConvertToWord,
            ]
            | coprocessor_1!(<Double | Word> ConvertToSingle)
            | coprocessor_1!(<Single | Word> ConvertToDouble) => parse_ops!(self, fpr, fpr),
            special![
                ShiftLeftLogicalVariable,
                ShiftRightLogicalVariable,
                ShiftRightArithmeticVariable,
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
            | special_2!(Multiply) => parse_ops!(self, gpr, gpr, gpr),
            special!(MoveConditional, true | false) => parse_ops!(self, gpr, gpr, cc?),
            special![ShiftLeftLogical, ShiftRightLogical, ShiftRightArithmetic] => {
                parse_ops!(self, gpr, gpr, shamt)
            }
            AddImmediate
            | AddImmediateUnsigned
            | SetLessThanImmediate
            | SetLessThanImmediateUnsigned => parse_ops!(self, gpr, gpr, i16),
            AndImmediate | OrImmediate | XorImmediate => parse_ops!(self, gpr, gpr, u16),
            BranchEqual | BranchNotEqual => parse_ops!(self, gpr, gpr, label),
            coprocessor_1![<Single | Double> MoveZero, MoveNotZero] => {
                parse_ops!(self, fpr, fpr, gpr)
            }
            coprocessor_1![<Single | Double> Add, Subtract, Multiply, Divide] => {
                parse_ops!(self, fpr, fpr, fpr)
            }
            coprocessor_1![<Single | Double> MoveConditional, true | false] => {
                parse_ops!(self, fpr, fpr, cc)
            }
            coprocessor_1![<Single | Double> CompareEqual, CompareLessThan, CompareLessEqual] => {
                parse_ops!(self, cc?, fpr, fpr)
            }
            LoadByte | LoadHalf | LoadWordLeft | LoadWord | LoadByteUnsigned | LoadHalfUnsigned
            | LoadWordRight | StoreByte | StoreHalf | StoreWordLeft | StoreWord
            | StoreConditional | StoreWordRight | LoadLinked => {
                self.parse_load_or_store_to_gpr()?
            }
            LoadWordCoprocessor1
            | LoadDoubleCoprocessor1
            | StoreWordCoprocessor1
            | StoreDoubleCoprocessor1 => self.parse_load_or_store_to_fpr()?,
            _ => return Err(Error::from(ErrorKind::InternalLogicIssue)),
        };
        Ok(Node::Instruction(operator, operands))
    }

    fn parse_load_or_store_to_gpr(&mut self) -> Result<[Option<Operand>; 3], Error> {
        let r0 = get_operand!(self, gpr);
        assert_token!(self, Comma);
        let offset = get_operand!(self, i16);
        let r1 = get_operand!(self, wrapped_gpr);
        assert_token_or_none!(self, NewLine);
        Ok([Some(r0), Some(offset), Some(r1)])
    }

    fn parse_load_or_store_to_fpr(&mut self) -> Result<[Option<Operand>; 3], Error> {
        let r0 = get_operand!(self, fpr);
        assert_token!(self, Comma);
        let offset = get_operand!(self, i16);
        let r1 = get_operand!(self, wrapped_gpr);
        assert_token_or_none!(self, NewLine);
        Ok([Some(r0), Some(offset), Some(r1)])
    }
}

impl Parser<'_> {
    fn next_token(&mut self) -> Option<Result<Token, ()>> {
        if let Some(token) = self.peeked.pop_front() {
            Some(Ok(token))
        } else {
            self.lexer.next()
        }
    }

    fn parse_segment_header(&mut self, directive: SegmentDirective) -> Result<Node, Error> {
        match self.next_token() {
            Some(Ok(Token::IntLiteral(addr))) => {
                Ok(Node::SegmentHeader(directive, Some(addr as u32)))
            }
            Some(Ok(token)) => {
                self.peeked.push_back(token);
                Ok(Node::SegmentHeader(directive, None))
            }
            Some(Err(_)) => Err(Error::from(ErrorKind::UnknownToken)),
            None => Ok(Node::SegmentHeader(directive, None)),
        }
    }

    fn parse_byte_array(&mut self) -> Result<Node, Error> {
        let mut bytes: Vec<i8> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a Comma or the end of the array",
                    ));
                }
                Some(Ok(Token::IntLiteral(int))) => match <i32 as TryInto<i8>>::try_into(int) {
                    Ok(byte) => bytes.push(byte),
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::ValueOutsideRange,
                            format!("{int} is outside the valid range for an i8"),
                        ));
                    }
                },
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected an IntLiteral or the end of the array",
                    ));
                }
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::ByteArray(bytes));
                }
                Some(Err(_)) => return Err(Error::from(ErrorKind::UnknownToken)),
                None => return Ok(Node::ByteArray(bytes)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_half_array(&mut self) -> Result<Node, Error> {
        let mut halves: Vec<i16> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a Comma or the end of the array",
                    ));
                }
                Some(Ok(Token::IntLiteral(int))) => match <i32 as TryInto<i16>>::try_into(int) {
                    Ok(half) => halves.push(half),
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::ValueOutsideRange,
                            format!("{int} is outside the valid range for an i16"),
                        ));
                    }
                },
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected an IntLiteral or the end of the array",
                    ));
                }
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::HalfArray(halves));
                }
                Some(Err(_)) => return Err(Error::from(ErrorKind::UnknownToken)),
                None => return Ok(Node::HalfArray(halves)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_word_array(&mut self) -> Result<Node, Error> {
        let mut words: Vec<i32> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a Comma or the end of the array",
                    ));
                }
                Some(Ok(Token::IntLiteral(int))) => words.push(int),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected an IntLiteral or the end of the array",
                    ));
                }
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::WordArray(words));
                }
                Some(Err(_)) => return Err(Error::from(ErrorKind::UnknownToken)),
                None => return Ok(Node::WordArray(words)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_float_array(&mut self) -> Result<Node, Error> {
        let mut floats: Vec<f32> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::FloatLiteral(_))) if needs_comma => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a Comma or the end of the array",
                    ));
                }
                Some(Ok(Token::FloatLiteral(double))) => floats.push(double as f32),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a FloatLiteral or the end of the array",
                    ));
                }
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::FloatArray(floats));
                }
                Some(Err(_)) => return Err(Error::from(ErrorKind::UnknownToken)),
                None => return Ok(Node::FloatArray(floats)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_double_array(&mut self) -> Result<Node, Error> {
        let mut doubles: Vec<f64> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::FloatLiteral(_))) if needs_comma => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a Comma or the end of the array",
                    ));
                }
                Some(Ok(Token::FloatLiteral(double))) => doubles.push(double),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken,
                        "expected a FloatLiteral or the end of the array",
                    ));
                }
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::DoubleArray(doubles));
                }
                Some(Err(_)) => return Err(Error::from(ErrorKind::UnknownToken)),
                None => return Ok(Node::DoubleArray(doubles)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_string(&mut self, append_nul: bool) -> Result<Node, Error> {
        match self.next_token() {
            Some(Ok(Token::StringLiteral(mut string))) => {
                if append_nul {
                    string.push('\0');
                }
                Ok(Node::String(string))
            }
            Some(Ok(Token::NewLine)) => self.parse_string(append_nul),
            Some(Ok(_)) => Err(Error::new(
                ErrorKind::UnexpectedToken,
                "expected a StringLiteral or a NewLine",
            )),
            Some(Err(_)) => Err(Error::from(ErrorKind::UnknownToken)),
            None => Err(Error::from(ErrorKind::PrematureEof)),
        }
    }

    fn parse_align_command(&mut self) -> Result<Node, Error> {
        match self.next_token() {
            Some(Ok(Token::IntLiteral(x))) if (0..4).contains(&x) => {
                Ok(Node::AlignCommand(x as u8))
            }
            Some(Ok(Token::IntLiteral(_))) => Err(Error::new(
                ErrorKind::ValueOutsideRange,
                ".align can only accept an alignment on the range 0..=3",
            )),
            Some(Ok(_)) => Err(Error::new(
                ErrorKind::UnexpectedToken,
                "expected an IntLiteral",
            )),
            Some(Err(_)) => Err(Error::from(ErrorKind::UnknownToken)),
            None => Err(Error::from(ErrorKind::PrematureEof)),
        }
    }

    fn parse_space_command(&mut self) -> Result<Node, Error> {
        match self.next_token() {
            Some(Ok(Token::IntLiteral(x))) if x >= 0 => Ok(Node::ByteArray(vec![0; x as usize])),
            Some(Ok(Token::IntLiteral(_))) => Err(Error::new(
                ErrorKind::ValueOutsideRange,
                ".space can only accept a positive number of bytes",
            )),
            Some(Ok(_)) => Err(Error::new(
                ErrorKind::UnexpectedToken,
                "expected an IntLiteral",
            )),
            Some(Err(_)) => Err(Error::from(ErrorKind::UnknownToken)),
            None => Err(Error::from(ErrorKind::PrematureEof)),
        }
    }

    fn parse_label(&mut self, label: String) -> Result<Node, Error> {
        match self.next_token() {
            Some(Ok(Token::Colon)) => Ok(Node::LabelDefinition(label)),
            Some(Ok(_)) => Err(Error::new(ErrorKind::UnexpectedToken, "expected a Colon")),
            Some(Err(_)) => Err(Error::from(ErrorKind::UnknownToken)),
            None => Err(Error::from(ErrorKind::PrematureEof)),
        }
    }
}
