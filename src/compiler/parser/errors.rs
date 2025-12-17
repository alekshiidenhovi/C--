use crate::compiler::lexer::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Represents a field that can contain a single token type or a set of token types.
#[derive(Debug, PartialEq)]
pub enum TokenTypeOption {
    One(TokenType),
    Many(Vec<TokenType>),
}

/// Represents errors that can occur during parsing an AST.
#[derive(Debug, PartialEq)]
pub enum ParserError {
    /// Raised when the parser runs out of tokens to parse, when it expects a token.
    UnexpectedEndOfInput,

    /// Raised when the parser encounters a token that does not match the expected token.
    ///
    /// # Arguments
    ///
    /// * `expected`: The expected or set of expected tokens.
    /// * `actual`: The actual token that was encountered.
    UnexpectedToken {
        expected: TokenTypeOption,
        actual: TokenType,
    },

    /// Raised when the parser encounters trailing tokens after the program has been parsed.
    UnexpectedTrailingTokens { found: Vec<Token> },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedEndOfInput => write!(f, "Parser error: Unexpected end of input"),
            ParserError::UnexpectedToken { expected, actual } => match expected {
                TokenTypeOption::One(expected) => {
                    write!(
                        f,
                        "Parser error: Unexpected token {:?}, expected {:?}",
                        actual, expected
                    )
                }
                TokenTypeOption::Many(expected) => {
                    write!(
                        f,
                        "Parser error: Unexpected token {:?}, expected one of {:?}",
                        actual, expected
                    )
                }
            },
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
