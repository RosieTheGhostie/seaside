use logos::{Lexer, Logos};

pub struct StringLiteralParser<'lit> {
    lexer: Lexer<'lit, Token>,
}

impl<'lit> StringLiteralParser<'lit> {
    pub fn new(literal: &'lit str) -> Self {
        Self {
            lexer: Token::lexer(literal),
        }
    }
}

impl Iterator for StringLiteralParser<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        let (Normal(c) | SimpleEscape(c) | OctalEscape(c) | HexadecimalEscape(c)
        | UnicodeEscape(c)) = self.lexer.next()?.ok()?;
        Some(c)
    }
}

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r#"""#)]
enum Token {
    #[regex(r#"[^"\\\x00-\x1f]"#, |lex| lex.slice().chars().next().unwrap())]
    Normal(char),

    #[regex(r#"\\['"?\\abfnrtv]"#, |lex| simple_escape(lex.slice().chars().nth(1).unwrap()))]
    SimpleEscape(char),

    #[regex(
        r#"\\[0-7]{1,3}"#,
        |lex| char::from_u32(u32::from_str_radix(&lex.slice()[1..], 8).unwrap()).unwrap(),
    )]
    OctalEscape(char),

    #[regex(
        r#"\\x[a-fA-F0-9]{2}"#,
        |lex| char::from_u32(u32::from_str_radix(&lex.slice()[2..], 16).unwrap()).unwrap(),
    )]
    HexadecimalEscape(char),

    #[regex(
        r#"\\u[a-fA-F0-9]{4}"#,
        |lex| char::from_u32(u32::from_str_radix(&lex.slice()[2..], 16).unwrap()).unwrap(),
    )]
    UnicodeEscape(char),
}

fn simple_escape(c: char) -> char {
    match c {
        '\'' | '"' | '?' | '\\' => c,
        'a' => '\x07',
        'b' => '\x08',
        'f' => '\x0c',
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        'v' => '\x0b',
        _ => unreachable!("you used me :("),
    }
}
