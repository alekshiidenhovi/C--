use std::error::Error;
use std::fmt;

/// Represents errors that can occur during lexing.
#[derive(Debug, PartialEq)]
pub enum LexerError<'a> {
    /// Represents an unexpected character encountered during lexing.
    ///
    /// This error occurs when the lexer finds a character that is not part of the expected grammar
    /// at the current position. This can happen with invalid input or incomplete token structures.
    ///
    /// # Arguments
    ///
    /// * `found`: The unexpected character that was found.
    /// * `expected`: Expected character.
    UnexpectedCharacter { found: char, expected: char },

    /// Represents a non-matching pattern error during lexing.
    ///
    /// This error occurs when the lexer finds a string that does not match the expected pattern.
    ///
    /// # Arguments
    ///
    /// * `found`: The string that did not match the pattern.
    NonmatchingPattern { found: &'a str },

    /// Represents an invalid constant error during lexing.
    InvalidConstant { found: String },
}

impl fmt::Display for LexerError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter { found, expected } => {
                write!(
                    f,
                    "Lexer error: Unexpected character found '{}', expected '{}'",
                    found, expected
                )
            }
            LexerError::NonmatchingPattern { found } => {
                write!(
                    f,
                    "Lexer error: The string did not match the expected pattern: '{}'",
                    found
                )
            }
            LexerError::InvalidConstant { found } => {
                write!(
                    f,
                    "Lexer error: The constant could not be parsed: {}",
                    found
                )
            }
        }
    }
}

impl Error for LexerError<'_> {}
