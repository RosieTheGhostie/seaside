mod expected;
mod expr;
mod operand;
mod value;

pub use expr::Expr;
pub use operand::Operand;
pub use value::Value;

use crate::{error::ParseError, token::Token};
use const_format::formatcp;
use logos::{Lexer, Logos, SpannedIter};
use seaside_error::rich::{
    Label, RichError, RichResult, Span, ToErrorCode,
    span::{combine_spans, consume_span},
};

/// The type returned by the [`Parser`] [iterator](Iterator).
///
/// It's essentially an alias usable outside the context of the [`Iterator`], which is useful for
/// writing sub-parser methods.
pub type ParserItem<'src> = Result<(Expr<'src>, Span), RichError>;

/// An [iterator](Iterator) that parses a stream of [token](Token)s.
///
/// # Examples
///
/// ```
/// # use logos::Logos;
/// # use crate::{parser::Parser, token::Token};
/// const SRC_NAME: &str = "sample.asm";
/// const SRC: &str = r#".data
/// kHello: .asciiz "Hello, World!\n"
///
/// .text
/// main:
///     addiu $v0, $0, 4
///     lui $a0, 0x1001
///     syscall
///
///     main.epilogue:
///         addiu $v0, $0, 10
///         syscall
///     main.endepilogue:
/// main.end:
/// "#;
///
/// let mut n_errors: usize = 0;
/// for expr_or_err in Parser::new(Token::lexer(SRC)) {
///     if let Err(err) = expr_or_err {
///         n_errors += 1;
///         let _ = err.report(SRC, SRC_NAME);
///     }
/// }
/// assert_eq!(n_errors, 0, "Parsing failed!");
/// ```
pub struct Parser<'src> {
    /// The underlying stream of [token](Token)s derived from the [lexer](Lexer).
    tokens: SpannedIter<'src, Token<'src>>,
    /// A queue of [token](Token)s that we've peeked at, but have yet to actually use.
    peeked: Vec<(Token<'src>, Span)>,
    /// The [span](Span) of the current [expression](Expr) being parsed.
    expr_span: Span,
    /// A representation of what [token](Token)(s) the parser expected to find.
    ///
    /// This is used to generate better error messages.
    expected: &'static str,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Token::lexer(source).into()
    }

    /// Gets the next unprocessed [token](Token) and its [span](Span).
    ///
    /// This is different from querying the `tokens` iterator because the method first checks for
    /// any tokens in the `peeked` queue.
    fn next_token(&mut self) -> Option<(Token<'src>, Span)> {
        if let Some((token, span)) = self.peeked.pop() {
            Some((token, span))
        } else {
            self.tokens
                .next()
                .map(|(result, span)| (result.into(), span))
        }
    }

    /// Yields the given expression as an item for the [`Parser`] [iterator][Iterator].
    fn r#yield(&self, expr: Expr<'src>) -> ParserItem<'src> {
        Ok((expr, self.expr_span.clone()))
    }

    /// Merges the given span with the current `expr_span`.
    ///
    /// This should typically be called each time a [token](Token)'s information is added to an
    /// [expression](Expr).
    const fn consume_span(&mut self, span: Span) {
        consume_span(&mut self.expr_span, span);
    }

    /// Constructs a new [`RichError`] from an ordinary error.
    fn new_error<E>(&self, err: E) -> RichError
    where
        E: ToErrorCode + ToString,
    {
        RichError::new(err, self.expr_span.clone())
    }

    /// Constructs a new [`RichError`] for when an unexpected token is encountered.
    ///
    /// To tell the user what was expected, set the `expected` field to the desired value before
    /// invoking this method.
    fn new_unexpected_token_error(&self, narrow_span: Span) -> RichError {
        self.new_error(ParseError::UnexpectedToken)
            .with_label(Label::new(narrow_span).with_message(format!("expected {}", self.expected)))
    }

    /// Constructs a new [`RichError`] for when the end of the file is reached too soon.
    ///
    /// This has very similar semantics to
    /// [`new_unexpected_token_error`](Parser::new_unexpected_token_error), as it also reads from
    /// the `expected` field.
    fn new_premature_eof_error(&self) -> RichError {
        let span = self.expr_span.end..self.expr_span.end;
        self.new_error(ParseError::PrematureEof)
            .with_label(Label::new(span).with_message(format!("expected {}", self.expected)))
    }

    /// Concludes a parsing iteration for an [expression](Expr) meant to end at the end of the line.
    ///
    /// The given callback should construct the [expression](Expr) from all the data gathered
    /// throughout the relevant iteration.
    fn expect_line_end<F>(&mut self, callback: F) -> ParserItem<'src>
    where
        F: FnOnce() -> Expr<'src>,
    {
        self.expected = expected::NEWLINE;
        match self.next_token() {
            Some((Token::NewLine, _)) | None => self.r#yield(callback()),
            Some(spanned_token) => Err(self.peek_and_throw_unexpected(spanned_token)),
        }
    }

    /// Adds the spanned [token](Token) provided to the `peeked` queue, then generates a
    /// [`RichError`] via the [`new_unexpected_token_error`](Parser::new_unexpected_token_error)
    /// method.
    ///
    /// This is useful when the [token](Token) in question is potentially meaningful as parsing
    /// continues, but is still invalid in the current context.
    fn peek_and_throw_unexpected(&mut self, spanned_token: (Token<'src>, Span)) -> RichError {
        let (token, span) = spanned_token;
        self.peeked.push((token, span.clone()));
        self.new_unexpected_token_error(span)
    }

    /// Attempts to parse a [segment header](Expr::SegmentHeader).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already,
    /// with its name being passed through the `name` parameter.
    fn parse_segment_header(&mut self, name: &'src str) -> ParserItem<'src> {
        self.expected = formatcp!("{} or {}", expected::INT_LIT, expected::NEWLINE);
        let address = match self.next_token() {
            Some((Token::Int(address @ 0..=0xffff_ffff), span)) => {
                self.consume_span(span);
                Some(address as u32)
            }
            Some((Token::Int(_), span)) => {
                return Err(self
                    .new_error(ParseError::ValueOutsideRange)
                    .with_narrow_span(span)
                    .with_help("addresses must be in the range 0..=0xffffffff"));
            }
            Some((Token::NewLine, _)) | None => None,
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
        };
        self.r#yield(Expr::SegmentHeader {
            directive: name.parse().unwrap(),
            address,
        })
    }

    /// Attempts to parse an [align command](Expr::AlignCommand).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_align_command(&mut self) -> ParserItem<'src> {
        self.expected = expected::INT_LIT;
        let alignment = match self.next_token() {
            Some((Token::Int(alignment @ 0..=3), span)) => {
                self.consume_span(span);
                alignment as u8
            }
            Some((Token::Int(_), span)) => {
                return Err(self
                    .new_error(ParseError::ValueOutsideRange)
                    .with_narrow_span(span)
                    .with_help("alignment must be either 0, 1, 2, or 3"));
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::AlignCommand { alignment })
    }

    /// Attempts to parse a [space command](Expr::SpaceCommand).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_space_command(&mut self) -> ParserItem<'src> {
        self.expected = expected::INT_LIT;
        let n_bytes = match self.next_token() {
            Some((Token::Int(n_bytes @ 0..=0xffff_ffff), span)) => {
                self.consume_span(span);
                n_bytes as u32
            }
            Some((Token::Int(_), span)) => {
                return Err(self
                    .new_error(ParseError::ValueOutsideRange)
                    .with_narrow_span(span)
                    .with_help("the .space directive takes a u32 as input"));
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::SpaceCommand { n_bytes })
    }

    /// Attempts to parse a [`eqv` macro](Expr::EqvMacro).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_eqv_macro(&mut self) -> ParserItem<'src> {
        // This is definitely not a correct implementation, as I'm struggling to wrap my mind around
        // the mechanics of the `.eqv` directive.
        self.expected = expected::IDENT;
        let name = match self.next_token() {
            Some((Token::Ident(name), span)) => {
                self.consume_span(span);
                name
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expected = expected::COMMA;
        match self.next_token() {
            Some((Token::Ctrl(','), span)) => consume_span(&mut self.expr_span, span),
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        }
        self.expected = expected::EXPR;
        let backup_expr_span = self.expr_span.clone();
        let expr = match self.next() {
            Some(Ok((expr, span))) => {
                self.expr_span = combine_spans([backup_expr_span, span]);
                expr
            }
            Some(Err(err)) => return Err(err),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::EqvMacro {
            name,
            expr: Box::new(expr),
        })
    }

    /// Attempts to parse an [include command](Expr::IncludeCommand).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_include_command(&mut self) -> ParserItem<'src> {
        self.expected = expected::STRING_LIT;
        let file_path = match self.next_token() {
            Some((Token::String(file_path), span)) => {
                self.consume_span(span);
                file_path
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::IncludeCommand { file_path })
    }

    /// Attempts to parse a [global command](Expr::GlobalCommand).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_global_command(&mut self) -> ParserItem<'src> {
        fn parse_label<'src>(
            this: &mut Parser<'src>,
            last_span: &mut Span,
            labels: &mut Vec<(&'src str, Span)>,
        ) -> RichResult<()> {
            this.expected = expected::IDENT;
            match this.next_token() {
                Some((Token::Ident(label), span)) => {
                    labels.push((label, span.clone()));
                    *last_span = span;
                    Ok(())
                }
                Some(spanned_token @ (Token::NewLine, _)) => {
                    Err(this.peek_and_throw_unexpected(spanned_token))
                }
                Some(spanned_token) => Err(this.peek_and_throw_unexpected(spanned_token)),
                None => Err(this.new_premature_eof_error()),
            }
        }

        let mut labels = Vec::new();
        let mut last_span = Span::default();
        parse_label(self, &mut last_span, &mut labels)?;
        loop {
            self.expected = formatcp!("{} or {}", expected::COMMA, expected::NEWLINE);
            match self.next_token() {
                Some((Token::Ctrl(','), span)) => last_span = span,
                Some((Token::NewLine, _)) | None => break,
                Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            }
            parse_label(self, &mut last_span, &mut labels)?;
        }

        self.consume_span(last_span);
        self.r#yield(Expr::GlobalCommand { labels })
    }

    /// Attempts to parse a [set command](Expr::SetCommand).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already.
    fn parse_set_command(&mut self) -> ParserItem<'src> {
        self.expected = expected::COMMAND;
        let command = match self.next_token() {
            Some((Token::Ident(command), span)) => {
                self.consume_span(span);
                command
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::SetCommand { command })
    }

    /// Attempts to parse a [string literal](Expr::String).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already,
    /// with its name being passed through the `directive` parameter.
    fn parse_string_literal(&mut self, directive: &'src str) -> ParserItem<'src> {
        self.expected = expected::STRING_LIT;
        let value = match self.next_token() {
            Some((Token::String(file_path), span)) => {
                self.consume_span(span);
                file_path
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expect_line_end(|| Expr::String {
            directive: directive.parse().unwrap(),
            value,
        })
    }

    /// Attempts to parse an [array of values](Expr::ValueArray).
    ///
    /// The [directive](Token::Directive) [token](Token) is assumed to have been processed already,
    /// with its name being passed through the `directive` parameter.
    fn parse_value_array(&mut self, directive: &'src str) -> ParserItem<'src> {
        let mut values = Vec::new();
        let mut last_span;
        self.expected = formatcp!("{} or {}", expected::INT_LIT, expected::FLOAT_LIT);
        match self.next_token() {
            Some((Token::Int(n), span)) => {
                values.push((Value::Int(n), span.clone()));
                last_span = span;
            }
            Some((Token::Float(x), span)) => {
                values.push((Value::Float(x), span.clone()));
                last_span = span;
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        }
        loop {
            self.expected = formatcp!("{} or {}", expected::COMMA, expected::NEWLINE);
            match self.next_token() {
                Some((Token::Ctrl(','), span)) => last_span = span,
                Some((Token::NewLine, _)) | None => break,
                Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            }
            self.expected = formatcp!(
                "{}, {}, or {}",
                expected::INT_LIT,
                expected::FLOAT_LIT,
                expected::NEWLINE,
            );
            match self.next_token() {
                Some((Token::Int(n), span)) => {
                    values.push((Value::Int(n), span.clone()));
                    last_span = span;
                }
                Some((Token::Float(x), span)) => {
                    values.push((Value::Float(x), span.clone()));
                    last_span = span;
                }
                // If the lexer didn't merge adjacent new lines, this would be problematic.
                Some((Token::NewLine, _)) => {}
                Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
                None => break,
            }
        }
        self.consume_span(last_span);
        self.r#yield(Expr::ValueArray {
            directive: directive.parse().unwrap(),
            values,
        })
    }

    /// Attempts to parse a [wrapped register](Operand::WrappedRegister).
    ///
    /// The opening parenthesis [token](Token) is assumed to have been processed already.
    fn parse_wrapped_register(&mut self) -> Result<(Operand<'src>, Span), RichError> {
        self.expected = expected::REGISTER;
        let name = match self.next_token() {
            Some((Token::Register(name), span)) => {
                self.consume_span(span.clone());
                name
            }
            Some(spanned_token) => return Err(self.peek_and_throw_unexpected(spanned_token)),
            None => return Err(self.new_premature_eof_error()),
        };
        self.expected = expected::R_PAREN;
        match self.next_token() {
            Some((Token::Ctrl(')'), span)) => {
                self.consume_span(span);
                Ok((Operand::WrappedRegister(name), self.expr_span.clone()))
            }
            Some(spanned_token) => Err(self.peek_and_throw_unexpected(spanned_token)),
            None => Err(self.new_premature_eof_error()),
        }
    }

    /// Attempts to parse an [expression](Expr) starting with an [identifier](Token::Ident)
    /// [token](Token).
    ///
    /// The [identifier](Token::Ident) [token](Token) is assumed to have been processed already,
    /// with its name being passed through the `ident` parameter.
    ///
    /// The resulting [expression](Expr) can be either a [label definition](Expr::LabelDef) or an
    /// [instruction](Expr::Instruction). Technically speaking, a
    /// [macro defined via `eqv`](Expr::EqvMacro) should also be an option, but I haven't
    /// implemented that yet.
    fn parse_ident(&mut self, ident: &'src str) -> ParserItem<'src> {
        #[derive(Clone, Copy, Debug)]
        enum CommaStatus {
            CannotHave { just_saw_comma: bool },
            CanHave,
            Need,
        }

        impl CommaStatus {
            pub const fn can_have(&self) -> bool {
                !matches!(self, Self::CannotHave { just_saw_comma: _ })
            }
        }

        let mut operands = Vec::new();
        /*
        // Not necessary, but this is what we're expecting.
        self.expected = formatcp!(
            "{}, {}, or {}",
            expected::COLON,
            expected::OPERAND,
            expected::NEWLINE,
        );
        */
        match self.next_token() {
            Some((Token::Ctrl(':'), span)) => {
                self.consume_span(span);
                return self.r#yield(Expr::LabelDef { ident });
            }
            Some(spanned_token) => self.peeked.push(spanned_token),
            None => {}
        }
        let mut comma_status = CommaStatus::CannotHave {
            just_saw_comma: false,
        };
        loop {
            match self.next_token() {
                Some((Token::Error(err), span)) => {
                    return Err(self.new_error(err).with_narrow_span(span));
                }
                Some((Token::NewLine, _)) | None => {
                    return if !matches!(
                        comma_status,
                        CommaStatus::CannotHave {
                            just_saw_comma: true
                        }
                    ) {
                        self.r#yield(Expr::Instruction {
                            operator: ident,
                            operands,
                        })
                    } else {
                        self.expected = expected::OPERAND;
                        Err(self.new_unexpected_token_error(Span {
                            start: self.expr_span.end,
                            end: self.expr_span.end + 1,
                        }))
                    };
                }
                Some((Token::Ctrl(','), span)) => match comma_status {
                    CommaStatus::CannotHave { just_saw_comma: _ } => {
                        self.expected = formatcp!("{} or {}", expected::OPERAND, expected::NEWLINE);
                        return Err(self.peek_and_throw_unexpected((Token::Ctrl(','), span)));
                    }
                    CommaStatus::CanHave | CommaStatus::Need => {
                        self.consume_span(span);
                        comma_status = CommaStatus::CannotHave {
                            just_saw_comma: true,
                        };
                    }
                },
                Some(spanned_token) if matches!(comma_status, CommaStatus::Need) => {
                    self.expected = expected::COMMA;
                    return Err(self.peek_and_throw_unexpected(spanned_token));
                }
                Some((Token::Int(n), span)) if !comma_status.can_have() => {
                    operands.push((Operand::Int(n), span.clone()));
                    self.consume_span(span);
                    comma_status = CommaStatus::CanHave;
                }
                Some((Token::Register(name), span)) if !comma_status.can_have() => {
                    operands.push((Operand::Register(name), span.clone()));
                    self.consume_span(span);
                    comma_status = CommaStatus::Need;
                }
                Some((Token::Ctrl('('), span)) => {
                    self.consume_span(span);
                    let (operand, span) = self.parse_wrapped_register()?;
                    operands.push((operand, span.clone()));
                    self.consume_span(span);
                    comma_status = CommaStatus::Need;
                }
                Some((Token::Ident(label), span)) if !comma_status.can_have() => {
                    operands.push((Operand::Label(label), span.clone()));
                    self.consume_span(span);
                    comma_status = CommaStatus::Need;
                }
                Some((token, span)) => {
                    self.expected = match comma_status {
                        CommaStatus::CannotHave { just_saw_comma: _ } => {
                            formatcp!("{} or {}", expected::OPERAND, expected::NEWLINE)
                        }
                        CommaStatus::CanHave => formatcp!(
                            "{}, {}, or {}",
                            expected::WRAPPED_REGISTER,
                            expected::COMMA,
                            expected::NEWLINE,
                        ),
                        CommaStatus::Need => {
                            formatcp!("{} or {}", expected::COMMA, expected::NEWLINE)
                        }
                    };
                    return Err(self.peek_and_throw_unexpected((token, span)));
                }
            }
        }
    }
}

impl<'src> Iterator for Parser<'src> {
    type Item = ParserItem<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let (token, span) = self.next_token()?;
        self.expr_span = span.clone();
        self.expected = formatcp!(
            "{}, {}, or {}",
            expected::DIRECTIVE,
            expected::IDENT,
            expected::NEWLINE,
        );
        match token {
            // A newline on its own isn't meaningful, so we can just skip it.
            Token::NewLine => self.next(),

            Token::Directive(name @ ("text" | "ktext" | "extern" | "data" | "kdata")) => {
                Some(self.parse_segment_header(name))
            }

            Token::Directive("align") => Some(self.parse_align_command()),
            Token::Directive("space") => Some(self.parse_space_command()),
            Token::Directive("eqv") => Some(self.parse_eqv_macro()),
            Token::Directive("include") => Some(self.parse_include_command()),
            Token::Directive("global" | "globl") => Some(self.parse_global_command()),
            Token::Directive("set") => Some(self.parse_set_command()),

            Token::Directive(directive @ ("ascii" | "asciiz")) => {
                Some(self.parse_string_literal(directive))
            }
            Token::Directive(directive @ ("byte" | "double" | "float" | "half" | "word")) => {
                Some(self.parse_value_array(directive))
            }

            Token::Directive(_) => Some(Err(self
                .new_error(ParseError::UnknownDirective)
                .with_narrow_span(span))),

            Token::Ident(ident) => Some(self.parse_ident(ident)),

            Token::Error(err) => Some(Err(self.new_error(err).with_narrow_span(span))),
            _ => Some(Err(self.new_unexpected_token_error(span))),
        }
    }
}

impl<'src> From<Lexer<'src, Token<'src>>> for Parser<'src> {
    fn from(lexer: Lexer<'src, Token<'src>>) -> Self {
        Self {
            tokens: lexer.spanned(),
            peeked: Vec::new(),
            expr_span: Span::default(),
            // In most situations, you'd probably want to set this to a useful value. I don't bother
            // here, though, because its value will be overwritten by the time it's ever read.
            expected: "",
        }
    }
}
