pub mod lexer;

use lexer::tokens::Token;

pub fn compile(input: &str) -> Vec<Token> {
    let tokens = lexer::tokenize(input.to_string());
    tokens
}
