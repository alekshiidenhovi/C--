use crate::compiler::lexer::tokens::TokenType;
use crate::compiler::parser::cmm_ast::CmmBinaryOperator;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during conversion from C-- AST to TACKY IR.
#[derive(Debug, PartialEq, Clone)]
pub enum IRConversionError {
    /// Raised when the IR conversion process encounters an unexpected token.
    UnexpectedToken {
        expected: TokenType,
        actual: TokenType,
    },
    /// Raised when attempting to convert a binary operator that is not supported.
    UnsupportedBinaryOperatorConversion { operator: CmmBinaryOperator },
}

impl fmt::Display for IRConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IRConversionError::UnexpectedToken { expected, actual } => write!(
                f,
                "IR conversion error: Unexpected token {:?}, expected {:?}",
                actual, expected
            ),
            IRConversionError::UnsupportedBinaryOperatorConversion { operator } => write!(
                f,
                "IR conversion error: Unsupported C-- binary operator conversion {:?}",
                operator
            ),
        }
    }
}

impl Error for IRConversionError {}
