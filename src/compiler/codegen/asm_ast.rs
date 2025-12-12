/// Represents an abstract syntax tree for assembly code.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyAst {
    /// Represents a complete program, containing a single function definition.
    Program(FunctionDefinition),
}

/// Represents the definition of a function.
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionDefinition {
    /// A function with a name and a list of instructions.
    Function {
        identifier: String,
        instructions: Vec<Instruction>,
    },
}

/// Represents a single instruction in the assembly code.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    /// Move operation: copies a value from a source operand to a destination operand.
    Mov {
        source: Operand,
        destination: Operand,
    },
    Unary {
        op: UnaryOp,
        operand: Operand,
    },
    AllocateStack(i32),
    /// Return instruction: signifies the end of a function execution.
    Ret,
}

/// Represents an unary operation.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

/// Represents an operand for an instruction, which can be an immediate value or a register.
#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    /// An immediate integer value.
    Imm(i32),
    /// A CPU register
    Register(Register),
    // A pseudo CPU register
    Pseudo(String),
    /// A stack location
    Stack(i32),
}

/// Represents a CPU register.
#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    AX,
    R10,
}
