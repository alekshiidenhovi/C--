pub mod errors;
pub mod tokens;

use errors::LexerError;
use regex::Regex;
use std::sync::LazyLock;
use tokens::Token;

type ParseResult<T> = Result<(String, T), LexerError>;
type Parser = fn(&str) -> ParseResult<Token>;

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
    let parsers: Vec<Parser> = vec![
        parse_identifier_or_keyword,
        parse_constant,
        parse_semicolon,
        parse_open_brace,
        parse_close_brace,
        parse_open_paren,
        parse_close_paren,
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

/// Attempts to parse a semicolon from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed semicolon.
/// On failure, returns a non-matching pattern error.
fn parse_semicolon(input_str: &str) -> ParseResult<Token> {
    let parsed_char = parse_character(input_str, ';');
    match parsed_char {
        Ok((remaining_str, _)) => Ok((remaining_str, Token::Semicolon)),
        Err(err) => Err(err),
    }
}

/// Attempts to parse an open brace from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed open brace.
/// On failure, returns a non-matching pattern error.
fn parse_open_brace(input_str: &str) -> ParseResult<Token> {
    let parsed_char = parse_character(input_str, '{');
    match parsed_char {
        Ok((remaining_str, _)) => Ok((remaining_str, Token::OpenBrace)),
        Err(err) => Err(err),
    }
}

/// Attempts to parse a closed brace from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed closed brace.
/// On failure, returns a non-matching pattern error.
fn parse_close_brace(input_str: &str) -> ParseResult<Token> {
    let parsed_char = parse_character(input_str, '}');
    match parsed_char {
        Ok((remaining_str, _)) => Ok((remaining_str, Token::CloseBrace)),
        Err(err) => Err(err),
    }
}

/// Attempts to parse an open parenthesis from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed open parenthesis.
/// On failure, returns a non-matching pattern error.
fn parse_open_paren(input_str: &str) -> ParseResult<Token> {
    let parsed_char = parse_character(input_str, '(');
    match parsed_char {
        Ok((remaining_str, _)) => Ok((remaining_str, Token::OpenParen)),
        Err(err) => Err(err),
    }
}

/// Attempts to parse a closed parenthesis from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed closed parenthesis.
/// On failure, returns a non-matching pattern error.
fn parse_close_paren(input_str: &str) -> ParseResult<Token> {
    let parsed_char = parse_character(input_str, ')');
    match parsed_char {
        Ok((remaining_str, _)) => Ok((remaining_str, Token::CloseParen)),
        Err(err) => Err(err),
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
fn parse_identifier_or_keyword(input_str: &str) -> ParseResult<Token> {
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
fn parse_constant(input_str: &str) -> ParseResult<Token> {
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
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed character.
/// On failure, returns an unexpected character error.
fn parse_character(input_str: &str, target_char: char) -> ParseResult<char> {
    if input_str.is_empty() {
        return Err(LexerError::EmptyInputString);
    }
    let (next_char, rest) = split_first_char(&input_str).unwrap();
    if next_char == target_char {
        Ok((rest.to_string(), next_char))
    } else {
        Err(LexerError::UnexpectedCharacter {
            found: next_char,
            expected: target_char,
        })
    }
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
        let result = parse_character(input, 'a');
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (String::from("bc"), 'a'));
    }

    #[test]
    fn parse_character_nonmatching_character() {
        let input = "abc";
        let result = parse_character(input, 'b');
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
