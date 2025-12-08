use crate::compiler::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during parsing an AST.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Raised when the parser runs out of tokens to parse, when it expects a token.
    UnexpectedEndOfInput,

    /// Raised when the parser encounters a token that does not match the expected token.
    ///
    /// # Arguments
    ///
    /// * `expected`: The expected token.
    /// * `actual`: The actual token that was encountered.
    UnexpectedToken {
        expected: TokenType,
        actual: TokenType,
    },

    /// Raised when the parser encounters trailing tokens after the program has been parsed.
    UnexpectedTrailingTokens { found: Vec<Token> },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedEndOfInput => write!(f, "Parser error: Unexpected end of input"),
            ParserError::UnexpectedToken { expected, actual } => {
                write!(
                    f,
                    "Parser error: Unexpected token found '{}', expected '{}'",
                    actual, expected
                )
            }
            ParserError::UnexpectedTrailingTokens { found } => {
                write!(
                    f,
                    "Parser error: Unexpected trailing tokens found {:?}",
                    found
                )
            }
        }
    }
}

impl Error for ParserError {}
