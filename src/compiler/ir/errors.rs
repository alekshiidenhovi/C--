use crate::compiler::tokens::TokenType;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during conversion from C-- AST to TACKY IR.
#[derive(Debug, PartialEq)]
pub enum IRConversionError {
    /// Raised when the IR conversion process encounters an unexpected token.
    UnexpectedToken {
        expected: TokenType,
        actual: TokenType,
    },
}

impl fmt::Display for IRConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IRConversionError::UnexpectedToken { expected, actual } => write!(
                f,
                "IR conversion error: Unexpected token {:?}, expected {:?}",
                actual, expected
            ),
        }
    }
}

impl Error for IRConversionError {}
