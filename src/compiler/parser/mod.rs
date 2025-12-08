pub mod ast;
pub mod errors;

use crate::compiler::tokens::{Token, TokenType};
use ast::{Ast, Expression, FunctionDefinition, Statement};
use errors::ParserError;

/// Represents a parser for a given sequence of tokens.
///
/// It is responsible for consuming tokens and constructing an Abstract Syntax Tree (AST).
pub struct Parser {
    /// The sequence of tokens to be parsed.
    pub tokens: Vec<Token>,
    /// The current position within the `tokens` vector.
    pub position: usize,
}

impl Parser {
    /// Creates a new `Parser` instance.
    ///
    /// # Arguments
    ///
    /// * `tokens`: A vector of `Token`s to be parsed.
    ///
    /// # Returns
    ///
    /// A new `Parser` instance initialized with the provided tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parses the entire sequence of tokens into an Abstract Syntax Tree (AST).
    ///
    /// This is the main entry point for the parsing process.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Ast` if parsing is successful, or a `ParserError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmm::compiler::tokens::Token;
    /// # use cmm::compiler::parser::ast::{Ast, FunctionDefinition, Statement, Expression};
    /// # use cmm::compiler::parser::Parser;
    /// # use cmm::compiler::parser::errors::ParserError;
    /// let tokens = vec![
    ///     Token::IntKeyword,
    ///     Token::Identifier("main".to_string()),
    ///     Token::OpenParen,
    ///     Token::VoidKeyword,
    ///     Token::CloseParen,
    ///     Token::OpenBrace,
    ///     Token::ReturnKeyword,
    ///     Token::Constant(1),
    ///     Token::Semicolon,
    ///     Token::CloseBrace,
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse_ast()?;
    /// assert_eq!(ast, Ast::Program(FunctionDefinition::Function("main".to_string(), Statement::Return(Expression::IntegerConstant(1)))));
    /// # Ok::<(), ParserError>(())
    /// ```
    pub fn parse_ast(&mut self) -> Result<Ast, ParserError> {
        let function = self.parse_function()?;
        if self.position < self.tokens.len() {
            return Err(ParserError::UnexpectedTrailingTokens {
                found: self.tokens.clone(),
            });
        }
        Ok(Ast::Program(function))
    }

    /// Parses a function definition from the token stream.
    ///
    /// A function definition is expected to start with `int`, followed by an identifier,
    /// parentheses, and a body containing a statement.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FunctionDefinition` if successful, or a `ParserError`.
    fn parse_function(&mut self) -> Result<FunctionDefinition, ParserError> {
        let _int = self.expect_token(TokenType::IntKeyword)?;
        let identifier = self.parse_identifier()?;
        let _open_paren = self.expect_token(TokenType::OpenParen)?;
        let _void = self.expect_token(TokenType::VoidKeyword)?;
        let _close_paren = self.expect_token(TokenType::CloseParen)?;
        let _open_brace = self.expect_token(TokenType::OpenBrace)?;
        let statement = self.parse_statement()?;
        let _close_brace = self.expect_token(TokenType::CloseBrace)?;
        Ok(FunctionDefinition::Function(identifier, statement))
    }

    /// Parses a single statement from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Statement` if successful, or a `ParserError`.
    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        let _return = self.expect_token(TokenType::ReturnKeyword)?;
        let expression = self.parse_expression()?;
        let _semicolon = self.expect_token(TokenType::Semicolon)?;
        Ok(Statement::Return(expression))
    }

    /// Parses an identifier token from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the identifier string if successful, or a `ParserError`.
    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Identifier(identifier) => Ok(identifier),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: TokenType::Identifier,
                    actual: token.kind(),
                });
            }
        }
    }

    /// Parses an expression from the token stream.
    ///
    /// Currently, only integer constants are supported as expressions.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Expression` if successful, or a `ParserError`.
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Constant(value) => Ok(Expression::IntegerConstant(value)),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: TokenType::Constant,
                    actual: token.kind(),
                });
            }
        }
    }

    /// Consumes the next token from the stream and checks if it matches the expected token.
    ///
    /// # Arguments
    ///
    /// * `expected`: The `Token` that is expected.
    ///
    /// # Returns
    ///
    /// A `Result` containing `()` if the token matches, or a `ParserError` if it does not match or if the end of input is reached unexpectedly.
    fn expect_token(&mut self, expected_type: TokenType) -> Result<(), ParserError> {
        let actual = self.consume_token()?;
        let actual_type = actual.kind();
        if actual_type != expected_type {
            return Err(ParserError::UnexpectedToken {
                expected: expected_type,
                actual: actual_type,
            });
        }
        Ok(())
    }

    /// Consumes and returns the next token from the stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the next `Token` if available, or a `ParserError` if the end of input is reached.
    fn consume_token(&mut self) -> Result<Token, ParserError> {
        if self.position >= self.tokens.len() {
            return Err(ParserError::UnexpectedEndOfInput);
        }
        let token = self.tokens[self.position].clone();
        self.position += 1;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_single_token_success() {
        let tokens = vec![Token::IntKeyword];
        let mut parser = Parser::new(tokens);
        let token = parser.consume_token().unwrap();
        assert_eq!(token, Token::IntKeyword);
    }

    #[test]
    fn test_consume_single_token_failure_no_tokens() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.consume_token();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParserError::UnexpectedEndOfInput);
    }
}
