/// Represents an abstract syntax tree for assembly code.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyAst {
    /// Represents a complete program, containing a single function definition.
    Program { function: AssemblyFunction },
}

/// Represents the definition of a function.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyFunction {
    /// A function with a name and a list of instructions.
    Function {
        identifier: String,
        instructions: Vec<AssemblyInstruction>,
    },
}

/// Represents a single instruction in the assembly code.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyInstruction {
    /// Move operation: copies a value from a source operand to a destination operand.
    Mov {
        source: AssemblyUnaryOperand,
        destination: AssemblyUnaryOperand,
    },
    /// Unary operation: applies a unary operator to an operand.
    Unary {
        op: AssemblyUnaryOperation,
        operand: AssemblyUnaryOperand,
    },
    /// Allocate stack instruction: allocates a specified amount of stack space.
    AllocateStack { stack_offset: i32 },
    /// Return instruction: signifies the end of a function execution.
    Ret,
}

/// Represents an unary operation.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyUnaryOperation {
    /// Negation instruction
    Neg,
    /// Bitwise NOT instruction
    Not,
}

/// Represents an operand for an instruction, which can be an immediate value or a register.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyUnaryOperand {
    /// An immediate integer value.
    Imm(i32),
    /// A CPU register
    Register(AssemblyRegister),
    // A pseudo CPU register
    Pseudo(String),
    /// A stack location
    Stack(i32),
}

/// Represents a CPU register.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyRegister {
    /// AX CPU register
    AX,
    /// R10 scratch register
    R10,
}
