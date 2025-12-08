/// Represents the abstract syntax tree of a program.
#[derive(Debug)]
pub enum Ast {
    /// A program is composed of a single function.
    Program(FunctionDefinition),
}

/// Represents a function definition.
#[derive(Debug)]
pub enum FunctionDefinition {
    /// A function definition consisting of its name and body.
    Function(String, Statement),
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
