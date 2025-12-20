/// Represents the abstract syntax tree of a program.
#[derive(Debug, PartialEq)]
pub enum CmmAst {
    /// A program is composed of a single function.
    Program { function: CmmFunction },
}

/// Represents a function definition.
#[derive(Debug, PartialEq)]
pub enum CmmFunction {
    /// A function definition consisting of its name and body.
    Function {
        identifier: String,
        body: CmmStatement,
    },
}

/// Represents a statement within a function.
#[derive(Debug, PartialEq)]
pub enum CmmStatement {
    /// A return statement, which returns an expression.
    Return { expression: CmmExpression },
}

/// Represents an expression that evaluates to a value.
#[derive(Debug, PartialEq)]
pub enum CmmExpression {
    /// Represents an integer literal constant.
    IntegerConstant { value: i32 },
    Unary {
        operator: CmmUnaryOperator,
        expression: Box<CmmExpression>,
    },
    Binary {
        operator: CmmBinaryOperator,
        left: Box<CmmExpression>,
        right: Box<CmmExpression>,
    },
}

/// Represents a unary operator.
#[derive(Debug, PartialEq)]
pub enum CmmUnaryOperator {
    Complement,
    Negate,
    Not,
}

/// Represents a binary operator.
#[derive(Debug, PartialEq)]
pub enum CmmBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}
