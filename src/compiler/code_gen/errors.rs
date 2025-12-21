use crate::compiler::ir_gen::tacky_ast::{TackyBinaryOperator, TackyUnaryOperator};
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
    /// Raised when attempting to convert  from a TACKY unary operator to an assembly unary operator, which is not supported.
    ///
    /// # Arguments
    ///
    /// * `operator`: The TACKY unary operator that could not be converted.
    UnsupportedUnaryOperatorConversion { operator: TackyUnaryOperator },
    /// Raised when attempting to convert from a TACKY binary operator to an equivalent assembly instruction, which is not supported.
    ///
    /// # Arguments
    ///
    /// * `operator`: The TACKY binary operator that could not be converted.
    UnsupportedConditionCodeConversion { operator: TackyBinaryOperator },
    /// Raised when attempting to convert from a TACKY binary operator to an equivalent binary instruction, which is not supported.
    ///
    /// # Arguments
    ///
    /// * `operator`: The TACKY binary operator that could not be converted.
    UnsupportedBinaryOperatorConversion { operator: TackyBinaryOperator },
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
            CodegenError::UnsupportedUnaryOperatorConversion { operator } => {
                write!(
                    f,
                    "Codegen error: Unsupported unary operator conversion '{:?}'",
                    operator
                )
            }
            CodegenError::UnsupportedConditionCodeConversion { operator } => {
                write!(
                    f,
                    "Codegen error: Unsupported condition code conversion '{:?}'",
                    operator
                )
            }
            CodegenError::UnsupportedBinaryOperatorConversion { operator } => {
                write!(
                    f,
                    "Codegen error: Unsupported binary operator conversion '{:?}'",
                    operator
                )
            }
        }
    }
}

impl Error for CodegenError {}
