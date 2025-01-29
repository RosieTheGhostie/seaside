use std::str::Chars;

use super::{
    directives::{DataTypeDirective, SegmentDirective},
    operation::{
        macros::{coprocessor_0, coprocessor_1, register_immediate, special, special_2},
        BasicOperator::{self, *},
    },
    string_literal::StringLiteralParser,
};
use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"([ \t\f]+|#.*)")]
pub enum Token {
    // --- Basic Symbols ---
    #[regex(r"[\r\n]+")]
    NewLine,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    // --- Keywords ---
    #[token(".data", |_| SegmentDirective::Data)]
    #[token(".extern", |_| SegmentDirective::Extern)]
    #[token(".kdata", |_| SegmentDirective::KData)]
    #[token(".ktext", |_| SegmentDirective::KText)]
    #[token(".text", |_| SegmentDirective::Text)]
    SegmentDirective(SegmentDirective),

    #[token(".ascii", |_| DataTypeDirective::Ascii)]
    #[token(".asciiz", |_| DataTypeDirective::AsciiZ)]
    #[token(".byte", |_| DataTypeDirective::Byte)]
    #[token(".double", |_| DataTypeDirective::Double)]
    #[token(".float", |_| DataTypeDirective::Float)]
    #[token(".half", |_| DataTypeDirective::Half)]
    #[token(".word", |_| DataTypeDirective::Word)]
    DataTypeDirective(DataTypeDirective),

    #[token("sll", |_| special!(ShiftLeftLogical), priority = 3)]
    #[token("movt", |_| special!(MoveConditional, true), priority = 3)]
    #[token("movf", |_| special!(MoveConditional, false), priority = 3)]
    #[token("srl", |_| special!(ShiftRightLogical), priority = 3)]
    #[token("sra", |_| special!(ShiftRightArithmetic), priority = 3)]
    #[token("sllv", |_| special!(ShiftLeftLogicalVariable), priority = 3)]
    #[token("srlv", |_| special!(ShiftRightLogicalVariable), priority = 3)]
    #[token("srav", |_| special!(ShiftRightArithmeticVariable), priority = 3)]
    #[token("jr", |_| special!(JumpRegister), priority = 3)]
    #[token("jalr", |_| special!(JumpAndLinkRegister), priority = 3)]
    #[token("movz", |_| special!(MoveZero), priority = 3)]
    #[token("movn", |_| special!(MoveNotZero), priority = 3)]
    #[token("syscall", |_| special!(SystemCall), priority = 3)]
    #[token("break", |_| special!(Break), priority = 3)]
    #[token("mfhi", |_| special!(MoveFromHigh), priority = 3)]
    #[token("mthi", |_| special!(MoveToHigh), priority = 3)]
    #[token("mflo", |_| special!(MoveFromLow), priority = 3)]
    #[token("mtlo", |_| special!(MoveToLow), priority = 3)]
    #[token("mult", |_| special!(Multiply), priority = 3)]
    #[token("multu", |_| special!(MultiplyUnsigned), priority = 3)]
    #[token("div", |_| special!(Divide), priority = 3)]
    #[token("divu", |_| special!(DivideUnsigned), priority = 3)]
    #[token("add", |_| special!(Add), priority = 3)]
    #[token("addu", |_| special!(AddUnsigned), priority = 3)]
    #[token("sub", |_| special!(Subtract), priority = 3)]
    #[token("subu", |_| special!(SubtractUnsigned), priority = 3)]
    #[token("and", |_| special!(And), priority = 3)]
    #[token("or", |_| special!(Or), priority = 3)]
    #[token("xor", |_| special!(Xor), priority = 3)]
    #[token("nor", |_| special!(Nor), priority = 3)]
    #[token("slt", |_| special!(SetLessThan), priority = 3)]
    #[token("sltu", |_| special!(SetLessThanUnsigned), priority = 3)]
    #[token("tge", |_| special!(TrapGreaterEqual), priority = 3)]
    #[token("tgeu", |_| special!(TrapGreaterEqualUnsigned), priority = 3)]
    #[token("tlt", |_| special!(TrapLessThan), priority = 3)]
    #[token("tltu", |_| special!(TrapLessThanUnsigned), priority = 3)]
    #[token("teq", |_| special!(TrapEqual), priority = 3)]
    #[token("tne", |_| special!(TrapNotEqual), priority = 3)]
    #[token("bltz", |_| register_immediate!(BranchLessThanZero), priority = 3)]
    #[token("bgez", |_| register_immediate!(BranchGreaterEqualZero), priority = 3)]
    #[token("tgei", |_| register_immediate!(TrapGreaterEqualImmediate), priority = 3)]
    #[token("tgeiu", |_| register_immediate!(TrapGreaterEqualImmediateUnsigned), priority = 3)]
    #[token("tlti", |_| register_immediate!(TrapLessThanImmediate), priority = 3)]
    #[token("tltiu", |_| register_immediate!(TrapLessThanImmediateUnsigned), priority = 3)]
    #[token("teqi", |_| register_immediate!(TrapEqualImmediate), priority = 3)]
    #[token("tnei", |_| register_immediate!(TrapNotEqualImmediate), priority = 3)]
    #[token("bltzal", |_| register_immediate!(BranchLessThanZeroAndLink), priority = 3)]
    #[token("bgezal", |_| register_immediate!(BranchGreaterEqualZeroAndLink), priority = 3)]
    #[token("j", |_| Jump, priority = 3)]
    #[token("jal", |_| JumpAndLink, priority = 3)]
    #[token("beq", |_| BranchEqual, priority = 3)]
    #[token("bne", |_| BranchNotEqual, priority = 3)]
    #[token("blez", |_| BranchLessEqualZero, priority = 3)]
    #[token("bgtz", |_| BranchGreaterThanZero, priority = 3)]
    #[token("addi", |_| AddImmediate, priority = 3)]
    #[token("addiu", |_| AddImmediateUnsigned, priority = 3)]
    #[token("slti", |_| SetLessThanImmediate, priority = 3)]
    #[token("sltiu", |_| SetLessThanImmediateUnsigned, priority = 3)]
    #[token("andi", |_| AndImmediate, priority = 3)]
    #[token("ori", |_| OrImmediate, priority = 3)]
    #[token("xori", |_| XorImmediate, priority = 3)]
    #[token("lui", |_| LoadUpperImmediate, priority = 3)]
    #[token("mfc0", |_| coprocessor_0!(MoveFromCoprocessor0), priority = 3)]
    #[token("mtc0", |_| coprocessor_0!(MoveToCoprocessor0), priority = 3)]
    #[token("eret", |_| coprocessor_0!(ErrorReturn), priority = 3)]
    #[token("add.s", |_| coprocessor_1!(<Single> Add), priority = 3)]
    #[token("add.d", |_| coprocessor_1!(<Double> Add), priority = 3)]
    #[token("sub.s", |_| coprocessor_1!(<Single> Subtract), priority = 3)]
    #[token("sub.d", |_| coprocessor_1!(<Double> Subtract), priority = 3)]
    #[token("mul.s", |_| coprocessor_1!(<Single> Multiply), priority = 3)]
    #[token("mul.d", |_| coprocessor_1!(<Double> Multiply), priority = 3)]
    #[token("div.s", |_| coprocessor_1!(<Single> Divide), priority = 3)]
    #[token("div.d", |_| coprocessor_1!(<Double> Divide), priority = 3)]
    #[token("sqrt.s", |_| coprocessor_1!(<Single> SquareRoot), priority = 3)]
    #[token("sqrt.d", |_| coprocessor_1!(<Double> SquareRoot), priority = 3)]
    #[token("abs.s", |_| coprocessor_1!(<Single> AbsoluteValue), priority = 3)]
    #[token("abs.d", |_| coprocessor_1!(<Double> AbsoluteValue), priority = 3)]
    #[token("mov.s", |_| coprocessor_1!(<Single> Move), priority = 3)]
    #[token("mov.d", |_| coprocessor_1!(<Double> Move), priority = 3)]
    #[token("neg.s", |_| coprocessor_1!(<Single> Negate), priority = 3)]
    #[token("neg.d", |_| coprocessor_1!(<Double> Negate), priority = 3)]
    #[token("round.w.s", |_| coprocessor_1!(<Single> RoundWord), priority = 3)]
    #[token("round.w.d", |_| coprocessor_1!(<Double> RoundWord), priority = 3)]
    #[token("trunc.w.s", |_| coprocessor_1!(<Single> TruncateWord), priority = 3)]
    #[token("trunc.w.d", |_| coprocessor_1!(<Double> TruncateWord), priority = 3)]
    #[token("ceil.w.s", |_| coprocessor_1!(<Single> CeilingWord), priority = 3)]
    #[token("ceil.w.d", |_| coprocessor_1!(<Double> CeilingWord), priority = 3)]
    #[token("floor.w.s", |_| coprocessor_1!(<Single> FloorWord), priority = 3)]
    #[token("floor.w.d", |_| coprocessor_1!(<Double> FloorWord), priority = 3)]
    #[token("movt.s", |_| coprocessor_1!(<Single> MoveConditional, true), priority = 3)]
    #[token("movt.d", |_| coprocessor_1!(<Double> MoveConditional, true), priority = 3)]
    #[token("movf.s", |_| coprocessor_1!(<Single> MoveConditional, false), priority = 3)]
    #[token("movf.d", |_| coprocessor_1!(<Double> MoveConditional, false), priority = 3)]
    #[token("movz.s", |_| coprocessor_1!(<Single> MoveZero), priority = 3)]
    #[token("movz.d", |_| coprocessor_1!(<Double> MoveZero), priority = 3)]
    #[token("movn.s", |_| coprocessor_1!(<Single> MoveNotZero), priority = 3)]
    #[token("movn.d", |_| coprocessor_1!(<Double> MoveNotZero), priority = 3)]
    #[token("cvt.s.d", |_| coprocessor_1!(<Double> ConvertToSingle), priority = 3)]
    #[token("cvt.s.w", |_| coprocessor_1!(<Word> ConvertToSingle), priority = 3)]
    #[token("cvt.d.s", |_| coprocessor_1!(<Single> ConvertToDouble), priority = 3)]
    #[token("cvt.d.w", |_| coprocessor_1!(<Word> ConvertToDouble), priority = 3)]
    #[token("cvt.w.s", |_| coprocessor_1!(<Single> ConvertToWord), priority = 3)]
    #[token("cvt.w.d", |_| coprocessor_1!(<Double> ConvertToWord), priority = 3)]
    #[token("c.eq.s", |_| coprocessor_1!(<Single> CompareEqual), priority = 3)]
    #[token("c.eq.d", |_| coprocessor_1!(<Double> CompareEqual), priority = 3)]
    #[token("c.lt.s", |_| coprocessor_1!(<Single> CompareLessThan), priority = 3)]
    #[token("c.lt.d", |_| coprocessor_1!(<Double> CompareLessThan), priority = 3)]
    #[token("c.le.s", |_| coprocessor_1!(<Single> CompareLessEqual), priority = 3)]
    #[token("c.le.d", |_| coprocessor_1!(<Double> CompareLessEqual), priority = 3)]
    #[token("madd", |_| special_2!(MultiplyAdd), priority = 3)]
    #[token("maddu", |_| special_2!(MultiplyAddUnsigned), priority = 3)]
    #[token("mul", |_| special_2!(Multiply), priority = 3)]
    #[token("msub", |_| special_2!(MultiplySubtract), priority = 3)]
    #[token("msubu", |_| special_2!(MultiplySubtractUnsigned), priority = 3)]
    #[token("clz", |_| special_2!(CountLeadingZeroes), priority = 3)]
    #[token("clo", |_| special_2!(CountLeadingOnes), priority = 3)]
    #[token("lb", |_| LoadByte, priority = 3)]
    #[token("lh", |_| LoadHalf, priority = 3)]
    #[token("lwl", |_| LoadWordLeft, priority = 3)]
    #[token("lw", |_| LoadWord, priority = 3)]
    #[token("lbu", |_| LoadByteUnsigned, priority = 3)]
    #[token("lhu", |_| LoadHalfUnsigned, priority = 3)]
    #[token("lwr", |_| LoadWordRight, priority = 3)]
    #[token("sb", |_| StoreByte, priority = 3)]
    #[token("sh", |_| StoreHalf, priority = 3)]
    #[token("swl", |_| StoreWordLeft, priority = 3)]
    #[token("sw", |_| StoreWord, priority = 3)]
    #[token("sc", |_| StoreConditional, priority = 3)]
    #[token("swr", |_| StoreWordRight, priority = 3)]
    #[token("ll", |_| LoadLinked, priority = 3)]
    #[token("lwc1", |_| LoadWordCoprocessor1, priority = 3)]
    #[token("ldc1", |_| LoadDoubleCoprocessor1, priority = 3)]
    #[token("swc1", |_| StoreWordCoprocessor1, priority = 3)]
    #[token("sdc1", |_| StoreDoubleCoprocessor1, priority = 3)]
    BasicOperator(BasicOperator),

    // #[token(r"?")]
    // PseudoOperator,

    // --- Identifiers (kinda) ---
    #[regex(r"[A-Za-z_][0-9A-Za-z_]*", |lex| lex.slice().to_owned())]
    Label(String),

    #[regex(r"\$(3[01]|[12]?\d)", |lex| lex.slice()[1..].parse::<u8>().unwrap())]
    RegisterIndex(u8),

    #[token("$zero", |_| 0)]
    #[token("$at", |_| 1)]
    #[regex(r"\$v[01]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 2)]
    #[regex(r"\$a[0-3]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 4)]
    #[regex(r"\$t[0-7]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 8)]
    #[regex(r"\$s[0-7]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 16)]
    #[regex(r"\$t[89]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 24)]
    #[regex(r"\$k[01]", |lex| lex.slice().chars().nth(2).unwrap().to_digit(10).unwrap() as u8 + 26)]
    #[token("$gp", |_| 28)]
    #[token("$sp", |_| 29)]
    #[token("$fp", |_| 30)]
    #[token("$ra", |_| 31)]
    CpuRegisterName(u8),

    #[token("$vaddr", |_| 8)]
    #[token("$status", |_| 12)]
    #[token("$cause", |_| 13)]
    #[token("$epc", |_| 14)]
    Cop0RegisterName(u8),

    #[regex(r"\$f(3[01]|[12]?\d)", |lex| lex.slice()[2..].parse::<u8>().unwrap())]
    Cop1RegisterName(u8),

    // --- Literals ---
    #[regex(r"0[oO][0-7]+", |lex| u32::from_str_radix(&lex.slice()[2..], 8).unwrap() as i32)]
    #[regex(r"[+-]?\d+", |lex| lex.slice().parse::<i32>().unwrap(), priority = 3)]
    #[regex(r"0[xX][0-9A-Fa-f]+", |lex| u32::from_str_radix(&lex.slice()[2..], 16).unwrap() as i32)]
    IntLiteral(i32),

    #[regex(
        r"[+-]?(\d+([.]\d*)?([eE][+-]?\d+)?|[.]\d+([eE][+-]?\d+)?)",
        |lex| lex.slice().parse::<f64>().unwrap(),
    )]
    FloatLiteral(f64),

    #[regex(
        r#""([^"\\\x00-\x1f]|\\(['"?\\abfnrtv]|[0-7]{1,3}|x[a-fA-F0-9]{2}|u[a-fA-F0-9]{4}))*""#,
        |lex| StringLiteralParser::new(lex.slice()).collect::<String>(),
    )]
    StringLiteral(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        const SOURCE: &str = r#".data
kHello: .asciiz "Hello, World!\n""#;
        let expected_tokens: [Token; 6] = [
            Token::SegmentDirective(SegmentDirective::Data),
            Token::NewLine,
            Token::Label("kHello".to_string()),
            Token::Colon,
            Token::DataTypeDirective(DataTypeDirective::AsciiZ),
            Token::StringLiteral("Hello, World!\n".to_string()),
        ];
        for (expected, got) in std::iter::zip(expected_tokens, Token::lexer(SOURCE)) {
            assert_eq!(
                expected,
                got.unwrap_or_else(|_| panic!("lexing failed (expected: {expected:?})"))
            );
        }
    }

    #[test]
    fn smallest_possible_program() {
        const SOURCE: &str = r#".text
addiu $v0, $0, 10
syscall""#;
        let expected_tokens: [Token; 10] = [
            Token::SegmentDirective(SegmentDirective::Text),
            Token::NewLine,
            Token::BasicOperator(BasicOperator::AddImmediateUnsigned),
            Token::CpuRegisterName(2),
            Token::Comma,
            Token::RegisterIndex(0),
            Token::Comma,
            Token::IntLiteral(10),
            Token::NewLine,
            Token::BasicOperator(special!(SystemCall)),
        ];
        for (expected, got) in std::iter::zip(expected_tokens, Token::lexer(SOURCE)) {
            assert_eq!(
                expected,
                got.unwrap_or_else(|_| panic!("lexing failed (expected: {expected:?})"))
            );
        }
    }
}
