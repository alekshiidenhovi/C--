/// Represents the top-level structure of TACKY Intermediate Representation.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyAst {
    /// A complete TACKY function definition.
    Program { function: TackyFunction },
}

/// Represents a TACKY function definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyFunction {
    /// Defines a function with a unique identifier and a list of instructions.
    Function {
        /// The unique name of the function.
        identifier: String,
        /// The sequence of instructions that make up the function's body.
        instructions: Vec<TackyInstruction>,
    },
}

/// Represents a single TACKY instruction.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyInstruction {
    /// An instruction that returns a value from the function.
    Return { value: TackyValue },
    /// An instruction that performs a unary operation on a value.
    Unary {
        /// The unary operator to be applied.
        operator: TackyUnaryOperator,
        /// The source value for the operation.
        source: TackyValue,
        /// The destination where the result of the operation will be stored.
        destination: TackyValue,
    },
    /// An instruction that performs a binary operation on two values.
    Binary {
        /// The binary operator to be applied.
        operator: TackyBinaryOperator,
        /// The first source value for the operation.
        source1: TackyValue,
        /// The second source value for the operation.
        source2: TackyValue,
        /// The destination where the result of the operation will be stored.
        destination: TackyValue,
    },
}

/// Represents a value within the TACKY IR.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyValue {
    /// Represents an integer constant.
    Constant(i32),
    /// Represents a variable, identified by its name.
    Variable(String),
}

/// Represents a unary operator within the TACKY IR.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyUnaryOperator {
    /// Bitwise complement
    Complement,
    /// Arithmetic negation
    Negate,
}

/// Represents a binary operator within the TACKY IR.
#[derive(Debug, Clone, PartialEq)]
pub enum TackyBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}
