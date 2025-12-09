use crate::compiler::tokens::Token;

/// Represents the abstract syntax tree of a program.
#[derive(Debug, PartialEq)]
pub enum Ast {
    /// A program is composed of a single function.
    Program(FunctionDefinition),
}

/// Represents a function definition.
#[derive(Debug, PartialEq)]
pub enum FunctionDefinition {
    /// A function definition consisting of its name and body.
    Function(Token, Statement),
}

/// Represents a statement within a function.
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// A return statement, which returns an expression.
    Return(Expression),
}

/// Represents an expression that evaluates to a value.
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Represents an integer literal constant.
    IntegerConstant(Token),
}
