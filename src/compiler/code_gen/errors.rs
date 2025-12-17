use crate::compiler::lexer::tokens::TokenType;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during code generation.
#[derive(Debug, PartialEq)]
pub enum CodegenError {
    /// Raised when an unexpected token is encountered during code generation.
    ///
    /// This error occurs when the code generator encounters a token that does not match the expected
    /// token type.
    ///
    /// # Arguments
    ///
    /// * `expected`: The expected token type.
    /// * `actual`: The actual token type that was encountered.
    UnexpectedToken {
        expected: TokenType,
        actual: TokenType,
    },
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CodegenError::UnexpectedToken { expected, actual } => {
                write!(
                    f,
                    "Codegen error: Unexpected token found '{}', expected '{}'",
                    actual, expected
                )
            }
        }
    }
}

impl Error for CodegenError {}
