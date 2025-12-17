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
        Box::new(parse_identifier_or_keyword),
        Box::new(parse_double_hyphen),
        Box::new(parse_constant),
        Box::new(create_character_parser('-', Token::Hyphen)),
        Box::new(create_character_parser('~', Token::Tilde)),
        Box::new(create_character_parser('(', Token::OpenParen)),
        Box::new(create_character_parser(')', Token::CloseParen)),
        Box::new(create_character_parser('{', Token::OpenBrace)),
        Box::new(create_character_parser('}', Token::CloseBrace)),
        Box::new(create_character_parser(';', Token::Semicolon)),
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

/// Attempts to parse a double hyphen from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed double hyphen.
/// On failure, returns a non-matching pattern error.
fn parse_double_hyphen(input_str: &str) -> LexerParseResult<Token> {
    static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^--").unwrap());
    match PATTERN.captures(input_str) {
        Some(matched) => {
            let matched_str = &matched[0];
            let remaining_str = input_str.strip_prefix(matched_str).unwrap().to_string();
            let token = Token::DoubleHyphen;
            Ok((remaining_str, token))
        }
        None => Err(LexerError::NonmatchingPattern {
            found: input_str.to_string(),
        }),
    }
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

/// Attempts to parse a single character from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
/// * `target_char`: The character to match.
/// * `parsed_token`: The token to return if the character is matched
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed character.
/// On failure, returns an unexpected character error.
fn create_character_parser(target_char: char, parsed_token: Token) -> LexerParser {
    Box::new(move |input_str: &str| {
        if input_str.is_empty() {
            return Err(LexerError::EmptyInputString);
        }
        let (next_char, rest) = split_first_char(&input_str).unwrap();
        if next_char == target_char {
            Ok((rest.to_string(), parsed_token.clone()))
        } else {
            Err(LexerError::UnexpectedCharacter {
                found: next_char,
                expected: target_char,
            })
        }
    })
}

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        let rest_of_str = chars.as_str();
        return Some((first_char, rest_of_str));
    }
    None
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
    fn parse_character_matching_character() {
        let input = "abc";
        let parser = create_character_parser('a', Token::Identifier("a".to_string()));
        let result = parser(input);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            (String::from("bc"), Token::Identifier("a".to_string()))
        );
    }

    #[test]
    fn parse_character_nonmatching_character() {
        let input = "abc";
        let parser = create_character_parser('b', Token::Identifier("b".to_string()));
        let result = parser(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            LexerError::UnexpectedCharacter {
                found: 'a',
                expected: 'b'
            }
        );
    }

    #[test]
    fn parse_valid_double_hyphen() {
        let input = "--";
        let result = parse_double_hyphen(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from(""), Token::DoubleHyphen));
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
