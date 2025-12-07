pub mod errors;
pub mod tokens;

use errors::LexerError;
use regex::Regex;
use std::sync::LazyLock;
use tokens::Token;

type ParseResult<'a, T> = Result<(String, T), LexerError<'a>>;

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        let rest_of_str = chars.as_str();
        return Some((first_char, rest_of_str));
    }
    None
}

pub fn tokenize(mut input_string: String) -> Vec<Token> {
    let mut token_vec = Vec::new();
    loop {
        input_string = input_string.trim_start().to_string();
        if input_string.is_empty() {
            break;
        }
    }
    token_vec
}

/// Attempts to parse a keyword from the input string.
///
/// # Arguments
///
/// * `input_str`: The input string to parse.
///
/// # Returns
///
/// On successful parsing, return a tuple of remaining input string and the parsed keyword.
/// On failure, returns a non-matching pattern error.
//fn parse_keyword<'a>(input_str: &'a str) -> ParseResult<'a, Token> {
//     if input_str == "int" {}
//     Err(LexerError::NonmatchingPattern { found: input_str })
// }

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
///

fn parse_constant<'a>(input_str: &'a str) -> ParseResult<'a, Token> {
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
        None => Err(LexerError::NonmatchingPattern { found: input_str }),
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
fn parse_character<'a>(input_str: &'a str, target_char: char) -> ParseResult<'a, char> {
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
            LexerError::NonmatchingPattern { found: "123abc" }
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
}
