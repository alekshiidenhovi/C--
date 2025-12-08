/// Represents the abstract syntax tree of a program.
#[derive(Debug)]
pub enum Ast {
    /// A program is composed of a single function.
    Program(Function),
}

/// Represents a function definition.
#[derive(Debug)]
pub enum Function {
    /// A function identified by its name.
    Identifier(String),
    /// A function body consisting of a single statement.
    Statement(Statement),
}

/// Represents a statement within a function.
#[derive(Debug)]
pub enum Statement {
    /// A return statement, which returns an expression.
    Return(Expression),
}

/// Represents an expression that evaluates to a value.
#[derive(Debug)]
pub enum Expression {
    /// Represents an integer literal constant.
    IntegerConstant(i32),
}
