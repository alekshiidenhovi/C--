use crate::compiler::tokens::Token;

/// Represents the abstract syntax tree of a program.
#[derive(Debug, PartialEq)]
pub enum CmmAst {
    /// A program is composed of a single function.
    Program(CmmFunction),
}

/// Represents a function definition.
#[derive(Debug, PartialEq)]
pub enum CmmFunction {
    /// A function definition consisting of its name and body.
    Function(Token, CmmStatement),
}

/// Represents a statement within a function.
#[derive(Debug, PartialEq)]
pub enum CmmStatement {
    /// A return statement, which returns an expression.
    Return(CmmExpression),
}

/// Represents an expression that evaluates to a value.
#[derive(Debug, PartialEq)]
pub enum CmmExpression {
    /// Represents an integer literal constant.
    IntegerConstant(Token),
    Unary(CmmUnaryOperator, Box<CmmExpression>),
}

/// Represents a unary operator.
#[derive(Debug, PartialEq)]
pub enum CmmUnaryOperator {
    Complement,
    Negate,
}
