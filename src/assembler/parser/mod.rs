pub mod macros;
pub mod node;
pub mod parse_error;

pub use node::Node;
pub use parse_error::ParseError;

use super::{
    directives::{DataTypeDirective, SegmentDirective},
    operation::macros::{coprocessor_0, coprocessor_1, register_immediate, special, special_2},
    BasicOperator, Operand, Token,
};
use logos::Lexer;
use macros::{assert_token, assert_token_or_none, get_operand, parse_ops};
use std::collections::VecDeque;

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
    peeked: VecDeque<Token>,
}

impl Iterator for Parser<'_> {
    type Item = Result<Node, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token: Token = match self.next_token()? {
            Ok(token) => token,
            Err(_) => return Some(Err(ParseError::UnknownToken)),
        };
        Some(match token {
            Token::NewLine => return self.next(),
            Token::BasicOperator(operator) => self.parse_instruction(operator),
            Token::Label(label) => self.parse_label(label),
            Token::SegmentDirective(directive) => self.parse_segment_header(directive),
            Token::DataTypeDirective(DataTypeDirective::Ascii) => self.parse_string(false),
            Token::DataTypeDirective(DataTypeDirective::AsciiZ) => self.parse_string(true),
            Token::DataTypeDirective(DataTypeDirective::Byte) => self.parse_byte_array(),
            Token::DataTypeDirective(DataTypeDirective::Double) => self.parse_double_array(),
            Token::DataTypeDirective(DataTypeDirective::Float) => self.parse_float_array(),
            Token::DataTypeDirective(DataTypeDirective::Half) => self.parse_half_array(),
            Token::DataTypeDirective(DataTypeDirective::Word) => self.parse_word_array(),
            _ => Err(ParseError::UnexpectedToken(token)),
        })
    }
}

impl<'source> Parser<'source> {
    pub fn new(lexer: Lexer<'source, Token>) -> Self {
        Self {
            lexer,
            peeked: VecDeque::new(),
        }
    }
}

impl Parser<'_> {
    fn parse_instruction(&mut self, operator: BasicOperator) -> Result<Node, ParseError> {
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
            _ => return Err(ParseError::InternalLogicIssue),
        };
        Ok(Node::Instruction(operator, operands))
    }

    fn parse_load_or_store_to_gpr(&mut self) -> Result<[Option<Operand>; 4], ParseError> {
        let r0 = get_operand!(self, gpr);
        assert_token!(self, Comma);
        let offset = get_operand!(self, i16);
        let r1 = get_operand!(self, wrapped_gpr);
        assert_token_or_none!(self, NewLine);
        Ok([Some(r0), Some(offset), Some(r1), None])
    }

    fn parse_load_or_store_to_fpr(&mut self) -> Result<[Option<Operand>; 4], ParseError> {
        let r0 = get_operand!(self, fpr);
        assert_token!(self, Comma);
        let offset = get_operand!(self, i16);
        let r1 = get_operand!(self, wrapped_gpr);
        assert_token_or_none!(self, NewLine);
        Ok([Some(r0), Some(offset), Some(r1), None])
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

    fn parse_segment_header(&mut self, directive: SegmentDirective) -> Result<Node, ParseError> {
        match self.next_token() {
            Some(Ok(Token::IntLiteral(addr))) => {
                Ok(Node::SegmentHeader(directive, Some(addr as u32)))
            }
            Some(Ok(token)) => {
                self.peeked.push_back(token);
                Ok(Node::SegmentHeader(directive, None))
            }
            Some(Err(_)) => Err(ParseError::UnknownToken),
            None => Ok(Node::SegmentHeader(directive, None)),
        }
    }

    fn parse_byte_array(&mut self) -> Result<Node, ParseError> {
        let mut bytes: Vec<i8> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(ParseError::UnexpectedToken(token.unwrap().unwrap()));
                }
                Some(Ok(Token::IntLiteral(int))) => match <i32 as TryInto<i8>>::try_into(int) {
                    Ok(byte) => bytes.push(byte),
                    Err(_) => return Err(ParseError::ValueTooLarge),
                },
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => return Err(ParseError::UnexpectedToken(Token::Comma)),
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::ByteArray(bytes));
                }
                Some(Err(_)) => return Err(ParseError::UnknownToken),
                None => return Ok(Node::ByteArray(bytes)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_half_array(&mut self) -> Result<Node, ParseError> {
        let mut halves: Vec<i16> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(ParseError::UnexpectedToken(token.unwrap().unwrap()));
                }
                Some(Ok(Token::IntLiteral(int))) => match <i32 as TryInto<i16>>::try_into(int) {
                    Ok(half) => halves.push(half),
                    Err(_) => return Err(ParseError::ValueTooLarge),
                },
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => return Err(ParseError::UnexpectedToken(Token::Comma)),
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::HalfArray(halves));
                }
                Some(Err(_)) => return Err(ParseError::UnknownToken),
                None => return Ok(Node::HalfArray(halves)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_word_array(&mut self) -> Result<Node, ParseError> {
        let mut words: Vec<i32> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::IntLiteral(_))) if needs_comma => {
                    return Err(ParseError::UnexpectedToken(token.unwrap().unwrap()));
                }
                Some(Ok(Token::IntLiteral(int))) => words.push(int),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => return Err(ParseError::UnexpectedToken(Token::Comma)),
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::WordArray(words));
                }
                Some(Err(_)) => return Err(ParseError::UnknownToken),
                None => return Ok(Node::WordArray(words)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_float_array(&mut self) -> Result<Node, ParseError> {
        let mut floats: Vec<f32> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::FloatLiteral(_))) if needs_comma => {
                    return Err(ParseError::UnexpectedToken(token.unwrap().unwrap()));
                }
                Some(Ok(Token::FloatLiteral(double))) => floats.push(double as f32),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => return Err(ParseError::UnexpectedToken(Token::Comma)),
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::FloatArray(floats));
                }
                Some(Err(_)) => return Err(ParseError::UnknownToken),
                None => return Ok(Node::FloatArray(floats)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_double_array(&mut self) -> Result<Node, ParseError> {
        let mut doubles: Vec<f64> = vec![];
        let mut needs_comma: bool = false;
        loop {
            let token = self.next_token();
            match token {
                Some(Ok(Token::NewLine)) => continue,
                Some(Ok(Token::FloatLiteral(_))) if needs_comma => {
                    return Err(ParseError::UnexpectedToken(token.unwrap().unwrap()));
                }
                Some(Ok(Token::FloatLiteral(double))) => doubles.push(double),
                Some(Ok(Token::Comma)) if needs_comma => {}
                Some(Ok(Token::Comma)) => return Err(ParseError::UnexpectedToken(Token::Comma)),
                Some(Ok(token)) => {
                    self.peeked.push_back(token);
                    return Ok(Node::DoubleArray(doubles));
                }
                Some(Err(_)) => return Err(ParseError::UnknownToken),
                None => return Ok(Node::DoubleArray(doubles)),
            }
            needs_comma = !needs_comma;
        }
    }

    fn parse_string(&mut self, append_nul: bool) -> Result<Node, ParseError> {
        match self.next_token() {
            Some(Ok(Token::StringLiteral(mut string))) => {
                if append_nul {
                    string.push('\0');
                }
                Ok(Node::String(string))
            }
            Some(Ok(Token::NewLine)) => self.parse_string(append_nul),
            Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
            Some(Err(_)) => Err(ParseError::UnknownToken),
            None => Err(ParseError::PrematureEof),
        }
    }

    fn parse_label(&mut self, label: String) -> Result<Node, ParseError> {
        match self.next_token() {
            Some(Ok(Token::Colon)) => Ok(Node::LabelDefinition(label)),
            Some(Ok(token)) => Err(ParseError::UnexpectedToken(token)),
            Some(Err(_)) => Err(ParseError::UnknownToken),
            None => Err(ParseError::PrematureEof),
        }
    }
}
