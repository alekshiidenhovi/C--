use std::fmt;

/// Represents a token in the C-- language.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
    DoubleHyphen,
    Plus,
    Asterisk,
    ForwardSlash,
    Percent,
}

impl Token {
    /// Returns the `TokenType` variant of the `Token`.
    ///
    /// This function maps the different `Token` enum variants to their corresponding
    /// `TokenType` enum variants, providing a simple way to categorize tokens.
    ///
    /// # Returns
    ///
    /// The `TokenType` that represents the kind of the token.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmm::compiler::lexer::tokens::Token;
    /// # use cmm::compiler::lexer::tokens::TokenType;
    ///
    /// let identifier_token = Token::Identifier(String::from("variable"));
    /// assert_eq!(identifier_token.kind(), TokenType::Identifier);
    ///
    /// let int_keyword_token = Token::IntKeyword;
    /// assert_eq!(int_keyword_token.kind(), TokenType::IntKeyword);
    /// ```
    pub fn kind(&self) -> TokenType {
        match self {
            Token::Identifier(_) => TokenType::Identifier,
            Token::Constant(_) => TokenType::Constant,
            Token::IntKeyword => TokenType::IntKeyword,
            Token::VoidKeyword => TokenType::VoidKeyword,
            Token::ReturnKeyword => TokenType::ReturnKeyword,
            Token::OpenParen => TokenType::OpenParen,
            Token::CloseParen => TokenType::CloseParen,
            Token::OpenBrace => TokenType::OpenBrace,
            Token::CloseBrace => TokenType::CloseBrace,
            Token::Semicolon => TokenType::Semicolon,
            Token::Tilde => TokenType::Tilde,
            Token::Hyphen => TokenType::Hyphen,
            Token::DoubleHyphen => TokenType::DoubleHyphen,
            Token::Plus => TokenType::Plus,
            Token::Asterisk => TokenType::Asterisk,
            Token::ForwardSlash => TokenType::ForwardSlash,
            Token::Percent => TokenType::Percent,
        }
    }

    /// Checks if the token is a binary operator.
    ///
    /// # Returns
    ///
    /// True if the token is a binary operator, false otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmm::compiler::lexer::tokens::Token;
    /// # use cmm::compiler::lexer::tokens::TokenType;
    ///
    /// let token = Token::Plus;
    /// assert_eq!(token.is_binary_operator(), true);
    ///
    /// let token = Token::IntKeyword;
    /// assert_eq!(token.is_binary_operator(), false);
    /// ```
    pub fn is_binary_operator(&self) -> bool {
        match self {
            Token::Plus => true,
            Token::Hyphen => true,
            Token::Asterisk => true,
            Token::ForwardSlash => true,
            Token::Percent => true,
            _ => false,
        }
    }

    /// Gets the precedence of a binary operator.
    ///
    /// # Returns
    ///
    /// The precedence of the binary operator, or an error if the token is not a binary operator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmm::compiler::lexer::tokens::Token;
    /// # use cmm::compiler::lexer::tokens::TokenType;
    ///
    /// let summation_token = Token::Plus;
    /// let multiplication_token = Token::Asterisk;
    /// assert!(summation_token.get_binary_operator_precedence().is_ok());
    /// assert!(multiplication_token.get_binary_operator_precedence().is_ok());
    /// assert!(summation_token.get_binary_operator_precedence().unwrap() < multiplication_token.get_binary_operator_precedence().unwrap());
    ///
    /// let token = Token::IntKeyword;
    /// assert!(token.get_binary_operator_precedence().is_err());
    /// ```
    pub fn get_binary_operator_precedence(&self) -> Result<u32, String> {
        let precedence = match self {
            Token::Plus => 45,
            Token::Hyphen => 45,
            Token::Asterisk => 50,
            Token::ForwardSlash => 50,
            Token::Percent => 50,
            _ => return Err(format!("Token {:?} is not a binary operator", self)),
        };
        Ok(precedence)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(identifier) => write!(f, "Identifier: {}", identifier),
            Token::Constant(constant) => write!(f, "Constant: {}", constant),
            Token::IntKeyword => write!(f, "IntKeyword"),
            Token::VoidKeyword => write!(f, "VoidKeyword"),
            Token::ReturnKeyword => write!(f, "ReturnKeyword"),
            Token::OpenParen => write!(f, "OpenParen"),
            Token::CloseParen => write!(f, "CloseParen"),
            Token::OpenBrace => write!(f, "OpenBrace"),
            Token::CloseBrace => write!(f, "CloseBrace"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Tilde => write!(f, "Tilde"),
            Token::Hyphen => write!(f, "Hyphen"),
            Token::DoubleHyphen => write!(f, "DoubleHyphen"),
            Token::Plus => write!(f, "Plus"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::ForwardSlash => write!(f, "ForwardSlash"),
            Token::Percent => write!(f, "Percent"),
        }
    }
}

/// Represents the type of a C-- token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    Constant,
    IntKeyword,
    VoidKeyword,
    ReturnKeyword,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
    DoubleHyphen,
    Plus,
    Asterisk,
    ForwardSlash,
    Percent,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::Constant => write!(f, "Constant"),
            TokenType::IntKeyword => write!(f, "IntKeyword"),
            TokenType::VoidKeyword => write!(f, "VoidKeyword"),
            TokenType::ReturnKeyword => write!(f, "ReturnKeyword"),
            TokenType::OpenParen => write!(f, "OpenParen"),
            TokenType::CloseParen => write!(f, "CloseParen"),
            TokenType::OpenBrace => write!(f, "OpenBrace"),
            TokenType::CloseBrace => write!(f, "CloseBrace"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Tilde => write!(f, "Tilde"),
            TokenType::Hyphen => write!(f, "Hyphen"),
            TokenType::DoubleHyphen => write!(f, "DoubleHyphen"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Asterisk => write!(f, "Asterisk"),
            TokenType::ForwardSlash => write!(f, "ForwardSlash"),
            TokenType::Percent => write!(f, "Percent"),
        }
    }
}
