pub mod errors;
pub mod tokens;

use errors::LexerError;
use regex::Regex;
use std::sync::LazyLock;
use tokens::Token;

/// Represents the result of a parsing operation, which can either be a success
/// containing the remaining unparsed string and the parsed value, or a `LexerError` after a
/// failure.
///
/// # Type Parameters
///
/// * `T`: The type of the successfully parsed value.
type LexerParseResult<T> = Result<(String, T), LexerError>;

/// A type alias for a function that parses a string slice into a `LexerParseResult<Token>`.
///
/// This is commonly used for defining lexer functions that consume input and produce tokens.
type LexerParser = Box<dyn Fn(&str) -> LexerParseResult<Token>>;

/// Tokenizes an input string into a vector of `Token`s.
///
/// This function iterates through the input string, attempting to parse it
/// using a predefined set of parsers. It trims whitespace before each parsing
/// attempt and continues until the string is empty.
///
/// # Arguments
///
/// * `input_str`: A string slice that represents the code to be tokenized.
///
/// # Returns
///
/// A `Vec<Token>` containing the recognized tokens from the input string.
///
/// # Examples
///
/// ```
/// # use cmm::compiler::lexer::tokenize;
/// # use cmm::compiler::lexer::tokens::Token;
///
/// let tokens = tokenize("int main(void) { return 1; }");
/// assert_eq!(tokens, vec![
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
/// ]);
/// ```
pub fn tokenize(input_str: &str) -> Vec<Token> {
    let mut string_stream = input_str.to_string();
    let mut token_vec = Vec::new();
    let parsers: Vec<LexerParser> = vec![
        // Custom parsers
        Box::new(parse_identifier_or_keyword),
        Box::new(parse_constant),
        // Two character tokens
        create_regex_parser(Regex::new(r"^--").unwrap(), Token::DoubleHyphen),
        create_regex_parser(Regex::new(r"^&&").unwrap(), Token::DoubleAmpersand),
        create_regex_parser(Regex::new(r"^\|\|").unwrap(), Token::DoublePipe),
        create_regex_parser(Regex::new(r"^==").unwrap(), Token::DoubleEqual),
        create_regex_parser(Regex::new(r"^!=").unwrap(), Token::ExclamationEqual),
        create_regex_parser(Regex::new(r"^<=").unwrap(), Token::LessThanEqual),
        create_regex_parser(Regex::new(r"^>=").unwrap(), Token::GreaterThanEqual),
        // Single character tokens
        create_regex_parser(Regex::new(r"^\-").unwrap(), Token::Hyphen),
        create_regex_parser(Regex::new(r"^\~").unwrap(), Token::Tilde),
        create_regex_parser(Regex::new(r"^\(").unwrap(), Token::OpenParen),
        create_regex_parser(Regex::new(r"^\)").unwrap(), Token::CloseParen),
        create_regex_parser(Regex::new(r"^\{").unwrap(), Token::OpenBrace),
        create_regex_parser(Regex::new(r"^\}").unwrap(), Token::CloseBrace),
        create_regex_parser(Regex::new(r"^\;").unwrap(), Token::Semicolon),
        create_regex_parser(Regex::new(r"^\+").unwrap(), Token::Plus),
        create_regex_parser(Regex::new(r"^\*").unwrap(), Token::Asterisk),
        create_regex_parser(Regex::new(r"^\/").unwrap(), Token::ForwardSlash),
        create_regex_parser(Regex::new(r"^\%").unwrap(), Token::Percent),
        create_regex_parser(Regex::new(r"^\!").unwrap(), Token::ExclamationMark),
        create_regex_parser(Regex::new(r"^<").unwrap(), Token::LessThan),
        create_regex_parser(Regex::new(r"^>").unwrap(), Token::GreaterThan),
    ];
    loop {
        string_stream = string_stream.trim_start().to_string();
        if string_stream.is_empty() {
            break;
        }
        for parser in parsers.iter() {
            if let Ok((remaining_str, token)) = parser(&string_stream) {
                token_vec.push(token);
                string_stream = remaining_str;
                continue;
            }
        }
    }
    token_vec
}

/// Creates a new lexer parser based on a regex pattern.
///
/// # Arguments
///
/// * `pattern`: The regex pattern to match.
/// * `token`: The token to return if the pattern matches.
///
/// # Returns
///
/// A new lexer parser that matches the pattern and returns the given token.
fn create_regex_parser(pattern: Regex, token: Token) -> LexerParser {
    Box::new(move |input_str: &str| {
        match pattern.captures(input_str) {
            Some(matched) => {
                let matched_str = &matched[0];
                // Strip the matched prefix to get the remaining string
                let remaining_str = input_str.strip_prefix(matched_str).unwrap().to_string();

                // Clone the token because the closure captures it by value
                // but needs to return it multiple times across different calls.
                Ok((remaining_str, token.clone()))
            }
            None => Err(LexerError::NonmatchingPattern {
                found: input_str.to_string(),
            }),
        }
    })
}

/// Attempts to parse an identifier or keyword from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed identifier or keyword.
/// On failure, returns a non-matching pattern error.
fn parse_identifier_or_keyword(input_str: &str) -> LexerParseResult<Token> {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z_]\w*\b").unwrap());
    match PATTERN.captures(input_str) {
        Some(matched) => {
            let matched_str = &matched[0];
            let remaining_str = input_str.strip_prefix(matched_str).unwrap().to_string();
            let token = match matched_str {
                "int" => Token::IntKeyword,
                "void" => Token::VoidKeyword,
                "return" => Token::ReturnKeyword,
                _ => Token::Identifier(matched_str.to_string()),
            };
            Ok((remaining_str, token))
        }
        None => Err(LexerError::NonmatchingPattern {
            found: input_str.to_string(),
        }),
    }
}

/// Attempts to parse a constant integer from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse, must be in decimal format.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed constant integer.
/// On failure, returns a non-matching pattern error.
fn parse_constant(input_str: &str) -> LexerParseResult<Token> {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9]+\b").unwrap());
    match PATTERN.captures(input_str) {
        Some(matched) => {
            let matched_str = &matched[0];
            let remaining_str = input_str.strip_prefix(matched_str).unwrap().to_string();
            let parsed_int =
                matched_str
                    .parse::<i32>()
                    .map_err(|_| LexerError::InvalidConstant {
                        found: matched_str.to_string(),
                    })?;
            let token = Token::Constant(parsed_int);
            Ok((remaining_str, token))
        }
        None => Err(LexerError::NonmatchingPattern {
            found: input_str.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_constant_only() {
        let input = "123";
        let result = parse_constant(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from(""), Token::Constant(123)));
    }

    #[test]
    fn test_parse_invalid_constant() {
        let input = "123;abc";
        let result = parse_constant(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            (String::from(";abc"), Token::Constant(123))
        );
    }

    #[test]
    fn test_parse_valid_constant_with_trailing_characters() {
        let input = "123abc";
        let result = parse_constant(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            LexerError::NonmatchingPattern {
                found: "123abc".to_string()
            }
        );
    }

    #[test]
    fn parse_valid_single_hyphen() {
        let input = "-a";
        let parser = create_regex_parser(Regex::new(r"^-").unwrap(), Token::Hyphen);
        let result = parser(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from("a"), Token::Hyphen));
    }

    #[test]
    fn parse_valid_double_hyphen() {
        let input = "--a";
        let parser = create_regex_parser(Regex::new(r"^--").unwrap(), Token::DoubleHyphen);
        let result = parser(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from("a"), Token::DoubleHyphen));
    }

    #[test]
    fn parse_valid_return_keyword() {
        let input = "return 2;";
        let result = parse_identifier_or_keyword(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from(" 2;"), Token::ReturnKeyword));
    }

    #[test]
    fn parse_valid_void_keyword() {
        let input = "void";
        let result = parse_identifier_or_keyword(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from(""), Token::VoidKeyword));
    }

    #[test]
    fn parse_valid_int_keyword() {
        let input = "int";
        let result = parse_identifier_or_keyword(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from(""), Token::IntKeyword));
    }

    #[test]
    fn parse_valid_identifier() {
        let input = "main";
        let result = parse_identifier_or_keyword(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            (String::from(""), Token::Identifier(input.to_string()))
        );
    }

    #[test]
    fn parse_invalid_identifier() {
        assert!(parse_identifier_or_keyword("1_number_first_not_allowed").is_err());
    }
}
