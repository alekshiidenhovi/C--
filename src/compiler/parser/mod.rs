pub mod cmm_ast;
pub mod errors;

use crate::compiler::lexer::tokens::{Token, TokenType};
use cmm_ast::{
    CmmAst, CmmBinaryOperator, CmmExpression, CmmFunction, CmmStatement, CmmUnaryOperator,
};
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
    /// # use cmm::compiler::lexer::tokens::Token;
    /// # use cmm::compiler::parser::cmm_ast::{CmmAst, CmmFunction, CmmStatement, CmmExpression, CmmUnaryOperator};
    /// # use cmm::compiler::parser::Parser;
    /// # use cmm::compiler::parser::errors::ParserError;
    /// let identifier = "main".to_string();
    /// let tokens = vec![
    ///     Token::IntKeyword,
    ///     Token::Identifier(identifier.clone()),
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
    /// assert_eq!(ast, CmmAst::Program { function: CmmFunction::Function { identifier, body: CmmStatement::Return { expression: CmmExpression::Unary { operator: CmmUnaryOperator::Negate, expression: Box::new(CmmExpression::IntegerConstant { value: 1 }) } } } });
    /// # Ok::<(), ParserError>(())
    /// ```
    pub fn parse_ast(&mut self) -> Result<CmmAst, ParserError> {
        let function = self.parse_function()?;
        if self.position < self.tokens.len() {
            return Err(ParserError::UnexpectedTrailingTokens {
                found: self.tokens[self.position..].to_vec(),
            });
        }
        Ok(CmmAst::Program { function })
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
        Ok(CmmFunction::Function {
            identifier,
            body: statement,
        })
    }

    /// Parses a single statement from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmStatement` if successful, or a `ParserError`.
    fn parse_statement(&mut self) -> Result<CmmStatement, ParserError> {
        let _return = self.expect_token(TokenType::ReturnKeyword)?;
        let expression = self.parse_expression(0)?;
        let _semicolon = self.expect_token(TokenType::Semicolon)?;
        Ok(CmmStatement::Return { expression })
    }

    /// Parses an identifier string from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the identifier string if successful, or a `ParserError`.
    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Identifier(identifier) => Ok(identifier.clone()),
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
    /// Uses the precedence climbing algorithm to parse expressions.
    ///
    /// Supported expressions:
    /// - Binary operations on two factors
    /// - Single factor
    ///
    /// # Arguments
    ///
    /// * `min_precedence` - The minimum precedence of the binary operator to be parsed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmExpression` if successful, or a `ParserError`.
    fn parse_expression(&mut self, min_precedence: u32) -> Result<CmmExpression, ParserError> {
        let mut left = self.parse_factor()?;
        let mut next_token = self.peek_token()?.clone();
        loop {
            if !next_token.is_binary_operator() {
                break;
            }

            // Non-binary operators will get -1 precedence, leading to a break in the next condition check
            let next_token_precedence = next_token
                .get_binary_operator_precedence()
                .map(|x| x as i32)
                .unwrap_or(-1);

            if next_token_precedence < min_precedence as i32 {
                break;
            }

            let operator = self.parse_binary_operator()?;
            let right = self.parse_expression((next_token_precedence + 1) as u32)?;
            left = CmmExpression::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            };
            next_token = self.peek_token()?.clone();
        }
        Ok(left)
    }

    /// Parses a factor from the token stream.
    ///
    /// Supported factor:
    /// - Integer constants
    /// - Unary operations on a factor
    /// - Parenthesized expressions
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmExpression` if successful, or a `ParserError`.
    fn parse_factor(&mut self) -> Result<CmmExpression, ParserError> {
        let token = self.peek_token()?;
        match token {
            Token::Constant(_) => self.parse_constant_integer_factor(),
            Token::Hyphen | Token::Tilde => self.parse_unary_factor(),
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
    fn parse_constant_integer_factor(&mut self) -> Result<CmmExpression, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Constant(value) => Ok(CmmExpression::IntegerConstant { value: *value }),
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
    fn parse_unary_factor(&mut self) -> Result<CmmExpression, ParserError> {
        let operator = self.parse_unary_operator()?;
        let inner_factor = self.parse_factor()?;
        Ok(CmmExpression::Unary {
            operator,
            expression: Box::new(inner_factor),
        })
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
            Token::ExclamationMark => Ok(CmmUnaryOperator::Not),
            _ => Err(ParserError::UnexpectedToken {
                expected: TokenTypeOption::Many(vec![
                    TokenType::Hyphen,
                    TokenType::Tilde,
                    TokenType::ExclamationMark,
                ]),
                actual: token.kind(),
            }),
        }
    }

    /// Parses a binary operator from the token stream.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CmmBinaryOperator` if successful, or a `ParserError`.
    fn parse_binary_operator(&mut self) -> Result<CmmBinaryOperator, ParserError> {
        let token = self.consume_token()?;
        match token {
            Token::Plus => Ok(CmmBinaryOperator::Add),
            Token::Hyphen => Ok(CmmBinaryOperator::Subtract),
            Token::Asterisk => Ok(CmmBinaryOperator::Multiply),
            Token::ForwardSlash => Ok(CmmBinaryOperator::Divide),
            Token::Percent => Ok(CmmBinaryOperator::Remainder),
            Token::DoubleAmpersand => Ok(CmmBinaryOperator::And),
            Token::DoublePipe => Ok(CmmBinaryOperator::Or),
            Token::DoubleEqual => Ok(CmmBinaryOperator::Equal),
            Token::ExclamationEqual => Ok(CmmBinaryOperator::NotEqual),
            Token::LessThan => Ok(CmmBinaryOperator::LessThan),
            Token::GreaterThan => Ok(CmmBinaryOperator::GreaterThan),
            Token::LessThanEqual => Ok(CmmBinaryOperator::LessThanEqual),
            Token::GreaterThanEqual => Ok(CmmBinaryOperator::GreaterThanEqual),
            _ => Err(ParserError::UnexpectedToken {
                expected: TokenTypeOption::Many(vec![
                    TokenType::Plus,
                    TokenType::Hyphen,
                    TokenType::Asterisk,
                    TokenType::ForwardSlash,
                    TokenType::Percent,
                    TokenType::DoubleAmpersand,
                    TokenType::DoublePipe,
                    TokenType::DoubleEqual,
                    TokenType::ExclamationEqual,
                    TokenType::LessThan,
                    TokenType::GreaterThan,
                    TokenType::LessThanEqual,
                    TokenType::GreaterThanEqual,
                ]),
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
        let expression = self.parse_expression(0)?;
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
        let tokens = vec![Token::Constant(1), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression(0);
        assert!(
            result.is_ok(),
            "Should parse valid constant integer expression, got {:?}",
            result
        );
        assert_eq!(result.unwrap(), CmmExpression::IntegerConstant { value: 1 });
    }

    #[test]
    fn test_parse_valid_unary_expression_negate() {
        let tokens = vec![Token::Hyphen, Token::Constant(1), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression(0);
        assert!(
            result.is_ok(),
            "Should parse valid unary expression negate, got {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            CmmExpression::Unary {
                operator: CmmUnaryOperator::Negate,
                expression: Box::new(CmmExpression::IntegerConstant { value: 1 })
            }
        );
    }

    #[test]
    fn test_parse_valid_unary_expression_complement() {
        let tokens = vec![Token::Tilde, Token::Constant(1), Token::Semicolon];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression(0);
        assert!(
            result.is_ok(),
            "Should parse valid unary expression complement, got {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            CmmExpression::Unary {
                operator: CmmUnaryOperator::Complement,
                expression: Box::new(CmmExpression::IntegerConstant { value: 1 })
            }
        );
    }

    #[test]
    fn test_parse_valid_parenthesized_expression() {
        let tokens = vec![
            Token::OpenParen,
            Token::Constant(1),
            Token::CloseParen,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression(0);
        assert!(
            result.is_ok(),
            "Should parse valid parenthesized expression, got {:?}",
            result
        );
        assert_eq!(result.unwrap(), CmmExpression::IntegerConstant { value: 1 });
    }

    #[test]
    fn test_parse_valid_operator_precedence() {
        let tokens = vec![
            Token::Constant(1),
            Token::Asterisk,
            Token::Constant(2),
            Token::Hyphen,
            Token::Constant(3),
            Token::Asterisk,
            Token::OpenParen,
            Token::Constant(4),
            Token::Plus,
            Token::Constant(5),
            Token::CloseParen,
            Token::Semicolon,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_expression(0);
        assert!(
            result.is_ok(),
            "Should parse expression with correct operator precedence, got {:?}",
            result
        );
        assert_eq!(
            result.unwrap(),
            CmmExpression::Binary {
                operator: CmmBinaryOperator::Subtract,
                left: Box::new(CmmExpression::Binary {
                    operator: CmmBinaryOperator::Multiply,
                    left: Box::new(CmmExpression::IntegerConstant { value: 1 }),
                    right: Box::new(CmmExpression::IntegerConstant { value: 2 }),
                }),
                right: Box::new(CmmExpression::Binary {
                    operator: CmmBinaryOperator::Multiply,
                    left: Box::new(CmmExpression::IntegerConstant { value: 3 }),
                    right: Box::new(CmmExpression::Binary {
                        operator: CmmBinaryOperator::Add,
                        left: Box::new(CmmExpression::IntegerConstant { value: 4 }),
                        right: Box::new(CmmExpression::IntegerConstant { value: 5 }),
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_parse_identifier_success() {
        let identifier = "main".to_string();
        let tokens = vec![Token::Identifier(identifier.clone())];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_identifier();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), identifier);
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
            CmmStatement::Return {
                expression: CmmExpression::IntegerConstant { value: 1 }
            }
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
        let identifier = "main".to_string();
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier(identifier.clone()),
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
            CmmFunction::Function {
                identifier: identifier,
                body: CmmStatement::Return {
                    expression: CmmExpression::IntegerConstant { value: 1 }
                }
            }
        );
    }

    #[test]
    fn test_parse_function_failure_unexpected_sequence() {
        let identifier = "main".to_string();
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier(identifier.clone()),
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
        let identifier = "main".to_string();
        let tokens = vec![
            Token::IntKeyword,
            Token::Identifier(identifier.clone()),
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
            CmmAst::Program {
                function: CmmFunction::Function {
                    identifier,
                    body: CmmStatement::Return {
                        expression: CmmExpression::IntegerConstant { value: 1 }
                    }
                }
            }
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
