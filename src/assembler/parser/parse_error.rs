use super::super::Token;

pub enum ParseError {
    UnknownToken,
    UnexpectedToken(Token),
    UnexpectedTokenNoContext,
    PrematureEof,
    ValueTooLarge,
    InternalLogicIssue,
}
