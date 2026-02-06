// use core::fmt::{self, Display, Formatter, Write};

use logos::Logos;
use strum::Display;

use crate::error::LexError;

/// A single "atom" in a MIPS Assembly program.
#[derive(Clone, Debug, Display, Logos, PartialEq)]
#[logos(skip(r"[ \t\f]+|#.*", allow_greedy = true))]
#[logos(error = LexError)]
pub enum Token<'src> {
    /// An error in the lexing stage of assembly.
    #[strum(to_string = "Error({0})")]
    Error(LexError),

    // --- Basic Symbols ---
    /// One or more newline characters.
    #[regex(r"[\r\n]+")]
    #[strum(to_string = r"\n")]
    NewLine,

    /// A control character.
    ///
    /// This currently includes commas (`,`), colons (`:`), and parentheses (`(` & `)`).
    #[regex(r"[,:()]", |lex| lex.slice().chars().next().unwrap())]
    #[strum(to_string = "{0}")]
    Ctrl(char),

    // --- Literals ---
    /// An integer literal.
    #[regex(r"0[oO][0-7]+", |lex| i64::from_str_radix(&lex.slice()[2..], 8))]
    #[regex(r"[+-]?\d+", |lex| lex.slice().parse(), priority = 3)]
    #[regex(r"0[xX][0-9A-Fa-f]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16))]
    #[strum(to_string = "{0}")]
    Int(i64),

    /// A floating-point literal.
    #[regex(
        r"[+-]?(\d+([.]\d*)?([eE][+-]?\d+)?|[.]\d+([eE][+-]?\d+)?)",
        |lex| lex.slice().parse(),
    )]
    #[strum(to_string = "{0}")]
    Float(f64),

    /// A string literal.
    ///
    /// The quotes are not stored.
    #[regex(
        r#""([^"\\\x00-\x1f]|\\[^\x00-\x1f])*""#,
        |lex| lex.slice()[1..].strip_suffix('"').unwrap(),
    )]
    #[strum(to_string = "{0}")]
    String(&'src str),

    // --- Identifier-Like Things ---
    /// The name of a hardware register.
    ///
    /// The `$` is not stored.
    #[regex(r"\$[a-z0-9]+", |lex| &lex.slice()[1..])]
    #[strum(to_string = "${0}")]
    Register(&'src str),

    /// An assembler directive.
    ///
    /// The `.` is not stored.
    #[regex(r"\.[a-z]+", |lex| &lex.slice()[1..])]
    #[strum(to_string = ".{0}")]
    Directive(&'src str),

    /// An identifier.
    ///
    /// These are usually just operators (e.g., `addiu`), but they can also be labels.
    #[regex(r"[a-zA-Z_](?:\.?[a-zA-Z_0-9])*")]
    #[strum(to_string = "{0}")]
    Ident(&'src str),
}

impl<'src> From<Result<Token<'src>, LexError>> for Token<'src> {
    fn from(result: Result<Token<'src>, LexError>) -> Self {
        match result {
            Ok(token) => token,
            Err(err) => Self::Error(err),
        }
    }
}
