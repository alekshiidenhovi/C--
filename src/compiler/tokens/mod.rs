use std::fmt;

/// Represents a token in the C language.
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
    /// # use cmm::compiler::tokens::Token;
    /// # use cmm::compiler::tokens::TokenType;
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
        }
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
        }
    }
}

/// Represents the type of a token.
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
        }
    }
}
