pub mod cmm_ast;
pub mod errors;

use crate::compiler::tokens::{Token, TokenType};
use cmm_ast::{CmmAst, CmmExpression, CmmFunction, CmmStatement, CmmUnaryOperator};
use errors::{ParserError, TokenTypeOption};

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
    /// A `Result` containing the `CmmAst` if parsing is successful, or a `ParserError` if an error occurs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmm::compiler::tokens::Token;
    /// # use cmm::compiler::parser::cmm_ast::{CmmAst, CmmFunction, CmmStatement, CmmExpression, CmmUnaryOperator};
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
    ///     Token::Hyphen,
    ///     Token::OpenParen,
    ///     Token::Constant(1),
    ///     Token::CloseParen,
    ///     Token::Semicolon,
    ///     Token::CloseBrace,
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse_ast()?;
    /// assert_eq!(ast, CmmAst::Program(CmmFunction::Function(Token::Identifier("main".to_string()), CmmStatement::Return(CmmExpression::Unary(CmmUnaryOperator::Negate, Box::new(CmmExpression::IntegerConstant(Token::Constant(1))))))));
    /// # Ok::<(), ParserError>(())
    /// ```
    pub fn parse_ast(&mut self) -> Result<CmmAst, ParserError> {
        let function = self.parse_function()?;
        if self.position < self.tokens.len() {
            return Err(ParserError::UnexpectedTrailingTokens {
                found: self.tokens[self.position..].to_vec(),
            });
        }
        Ok(CmmAst::Program(function))
    }

    /// Parses a function definition from the token stream.
    ///
    /// A function definition is expected to start with `int`, followed by an identifier,
    /// parentheses, and a body containing a statement.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `CmmFunction` if successful, or a `ParserError`.
    fn parse_function(&mut self) -> Result<CmmFunction, ParserError> {
        let _int = self.expect_token(TokenType::IntKeyword)?;
        let identifier = self.parse_identifier()?;
        let _open_paren = self.expect_token(TokenType::OpenParen)?;
        let _void = self.expect_token(TokenType::VoidKeyword)?;
        let _close_paren = self.expect_token(TokenType::CloseParen)?;
        let _open_brace = self.expect_token(TokenType::OpenBrace)?;
        let statement = self.parse_statement()?;
        let _close_brace = self.expect_token(TokenType::CloseBrace)?;
        Ok(CmmFunction::Function(identifier, statement))
    }

    /// Parses a single statement from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmStatement` if successful, or a `ParserError`.
    fn parse_statement(&mut self) -> Result<CmmStatement, ParserError> {
        let _return = self.expect_token(TokenType::ReturnKeyword)?;
        let expression = self.parse_expression()?;
        let _semicolon = self.expect_token(TokenType::Semicolon)?;
        Ok(CmmStatement::Return(expression))
    }

    /// Parses an identifier token from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the identifier string if successful, or a `ParserError`.
    fn parse_identifier(&mut self) -> Result<Token, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Identifier(_) => Ok(token.clone()),
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: TokenTypeOption::One(TokenType::Identifier),
                    actual: token.kind(),
                });
            }
        }
    }

    /// Parses an expression from the token stream.
    ///
    /// Supported expressions:
    /// - Integer constants
    /// - Unary operators (negation and complement)
    /// - Parenthesized expressions
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmExpression` if successful, or a `ParserError`.
    fn parse_expression(&mut self) -> Result<CmmExpression, ParserError> {
        let token = self.peek_token()?;
        match token {
            Token::Constant(_) => self.parse_constant_integer_expression(),
            Token::Hyphen | Token::Tilde => self.parse_unary_expression(),
            Token::OpenParen => self.parse_parenthesized_expression(),
            _ => Err(ParserError::UnexpectedToken {
                expected: TokenTypeOption::Many(vec![
                    TokenType::Constant,
                    TokenType::Hyphen,
                    TokenType::Tilde,
                    TokenType::OpenParen,
                ]),
                actual: token.kind(),
            }),
        }
    }

    /// Parses a constant integer expression from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmExpression` if successful, or a `ParserError`.
    fn parse_constant_integer_expression(&mut self) -> Result<CmmExpression, ParserError> {
        let token = self.consume_token()?;
        match token.kind() {
            TokenType::Constant => Ok(CmmExpression::IntegerConstant(token.clone())),
            _ => Err(ParserError::UnexpectedToken {
                expected: TokenTypeOption::One(TokenType::Constant),
                actual: token.kind(),
            }),
        }
    }

    /// Parses a unary expression from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed unary `CmmExpression` if successful, or a `ParserError`.
    fn parse_unary_expression(&mut self) -> Result<CmmExpression, ParserError> {
        let operator = self.parse_unary_operator()?;
        let inner_expression = self.parse_expression()?;
        Ok(CmmExpression::Unary(operator, Box::new(inner_expression)))
    }

    /// Parses a unary operator from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmUnaryOperator` if successful, or a `ParserError`.
    fn parse_unary_operator(&mut self) -> Result<CmmUnaryOperator, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Hyphen => Ok(CmmUnaryOperator::Negate),
            Token::Tilde => Ok(CmmUnaryOperator::Complement),
            _ => Err(ParserError::UnexpectedToken {
                expected: TokenTypeOption::Many(vec![TokenType::Hyphen, TokenType::Tilde]),
                actual: token.kind(),
            }),
        }
    }

    /// Parses a parenthesized expression from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmExpression` if successful, or a `ParserError`.
    fn parse_parenthesized_expression(&mut self) -> Result<CmmExpression, ParserError> {
        let _open_paren = self.expect_token(TokenType::OpenParen)?;
        let expression = self.parse_expression()?;
        let _close_paren = self.expect_token(TokenType::CloseParen)?;
        Ok(expression)
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
                expected: TokenTypeOption::One(expected_type),
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
    fn consume_token(&mut self) -> Result<&Token, ParserError> {
        if self.position >= self.tokens.len() {
            return Err(ParserError::UnexpectedEndOfInput);
        }
        let token = &self.tokens[self.position];
        self.position += 1;
        Ok(token)
    }

    /// Peeks at the next token from the stream without consuming it.
    ///
    /// # Returns
    ///
    /// A `Result` containing the next `Token` if available, or a `ParserError` if the end of input is reached.
    fn peek_token(&mut self) -> Result<&Token, ParserError> {
        if self.position >= self.tokens.len() {
            return Err(ParserError::UnexpectedEndOfInput);
        }
        let token = &self.tokens[self.position];
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
        assert_eq!(token.clone(), Token::IntKeyword);
    }

    #[test]
    fn test_consume_single_token_failure_no_tokens() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.consume_token();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParserError::UnexpectedEndOfInput);
    }

    #[test]
    fn test_expect_token_success() {
        let tokens = vec![Token::IntKeyword];
        let mut parser = Parser::new(tokens);
        let result = parser.expect_token(TokenType::IntKeyword);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    #[test]
    fn test_expect_token_failure_no_tokens() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.expect_token(TokenType::IntKeyword);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParserError::UnexpectedEndOfInput);
    }

    #[test]
    fn test_expect_token_failure_unexpected_sequence() {
        let tokens = vec![Token::IntKeyword];
        let mut parser = Parser::new(tokens);
        let result = parser.expect_token(TokenType::ReturnKeyword);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken {
                expected: TokenTypeOption::One(TokenType::ReturnKeyword),
                actual: TokenType::IntKeyword
            }
        );
    }

    #[test]
    fn test_parse_valid_constant_integer_expression() {
        let tokens = vec![Token::Constant(1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmExpression::IntegerConstant(Token::Constant(1))
        );
    }

    #[test]
    fn test_parse_valid_unary_expression_negate() {
        let tokens = vec![Token::Hyphen, Token::Constant(1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmExpression::Unary(
                CmmUnaryOperator::Negate,
                Box::new(CmmExpression::IntegerConstant(Token::Constant(1)))
            )
        );
    }

    #[test]
    fn test_parse_valid_unary_expression_complement() {
        let tokens = vec![Token::Tilde, Token::Constant(1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmExpression::Unary(
                CmmUnaryOperator::Complement,
                Box::new(CmmExpression::IntegerConstant(Token::Constant(1)))
            )
        );
    }

    #[test]
    fn test_parse_valid_parenthesized_expression() {
        let tokens = vec![Token::OpenParen, Token::Constant(1), Token::CloseParen];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmExpression::IntegerConstant(Token::Constant(1))
        );
    }

    #[test]
    fn test_parse_identifier_success() {
        let tokens = vec![Token::Identifier("main".to_string())];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_identifier();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Token::Identifier("main".to_string()));
    }

    #[test]
    fn test_parse_identifier_failure_unexpected_sequence() {
        let tokens = vec![Token::IntKeyword];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_identifier();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken {
                expected: TokenTypeOption::One(TokenType::Identifier),
                actual: TokenType::IntKeyword
            }
        );
    }

    #[test]
    fn test_parse_statement_success() {
        let tokens = vec![Token::ReturnKeyword, Token::Constant(1), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_statement();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmStatement::Return(CmmExpression::IntegerConstant(Token::Constant(1)))
        );
    }

    #[test]
    fn test_parse_statement_failure_unexpected_sequence() {
        let tokens = vec![Token::ReturnKeyword, Token::VoidKeyword, Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_statement();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken {
                expected: TokenTypeOption::Many(vec![
                    TokenType::Constant,
                    TokenType::Hyphen,
                    TokenType::Tilde,
                    TokenType::OpenParen
                ]),
                actual: TokenType::VoidKeyword
            }
        );
    }

    #[test]
    fn test_parse_function_success() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::VoidKeyword,
            Token::CloseParen,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Constant(1),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_function();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmFunction::Function(
                Token::Identifier("main".to_string()),
                CmmStatement::Return(CmmExpression::IntegerConstant(Token::Constant(1)))
            )
        );
    }

    #[test]
    fn test_parse_function_failure_unexpected_sequence() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::VoidKeyword,
            Token::CloseParen,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Constant(1),
            Token::Semicolon,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_function();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken {
                expected: TokenTypeOption::One(TokenType::CloseBrace),
                actual: TokenType::Semicolon
            }
        );
    }

    #[test]
    fn test_parse_ast_success() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::VoidKeyword,
            Token::CloseParen,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Constant(1),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_ast();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            CmmAst::Program(CmmFunction::Function(
                Token::Identifier("main".to_string()),
                CmmStatement::Return(CmmExpression::IntegerConstant(Token::Constant(1)))
            ))
        );
    }

    #[test]
    fn test_parse_ast_failure_no_tokens() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_ast();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParserError::UnexpectedEndOfInput);
    }

    #[test]
    fn test_parse_ast_failure_too_short_sequence() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::VoidKeyword,
            Token::CloseParen,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_ast();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParserError::UnexpectedEndOfInput);
    }

    #[test]
    fn test_parse_ast_failure_unexpected_sequence() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::ReturnKeyword,
            Token::VoidKeyword,
            Token::CloseParen,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Constant(1),
            Token::Semicolon,
            Token::CloseBrace,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_ast();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedToken {
                expected: TokenTypeOption::One(TokenType::OpenParen),
                actual: TokenType::ReturnKeyword
            }
        );
    }

    #[test]
    fn test_parse_ast_failure_unexpected_trailing_tokens() {
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParen,
            Token::VoidKeyword,
            Token::CloseParen,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Constant(1),
            Token::Semicolon,
            Token::CloseBrace,
            Token::Semicolon,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_ast();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParserError::UnexpectedTrailingTokens {
                found: vec![Token::Semicolon, Token::Semicolon]
            }
        );
    }
}
