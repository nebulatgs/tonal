use crate::tokens::{TokenType, Keyword, Separator, Location};
pub type ParseResult<T> = Result<T, ParseError>;
#[derive(Debug)]
pub enum ParseError {
    ExpectedType(TokenType),
    ExpectedKeyword(Keyword),
    ExpectedSeparator(Separator),
    ExpectedArgument,
    EmptyMeta,
    Unknown
}

pub type ParseFinalError = (ParseError, Location);

pub type ParseFinalResult<T> = Result<T, ParseFinalError>;
