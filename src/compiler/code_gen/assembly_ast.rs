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
    /// Move instruction: copies a value from a source operand to a destination operand.
    Mov {
        source: AssemblyOperand,
        destination: AssemblyOperand,
    },
    /// Unary instruction: applies a unary operator to an operand.
    Unary {
        op: AssemblyUnaryOperator,
        operand: AssemblyOperand,
    },
    /// Binary instruction: applies a binary operator to two operands.
    Binary {
        op: AssemblyBinaryOperator,
        source: AssemblyOperand,
        destination: AssemblyOperand,
    },
    /// Divide instruction: divides an operand with values stored in %eax and %edx.
    Idiv { operand: AssemblyOperand },
    /// Convert Doubleword to Quadword (CDQ) instruction: performs sign extension on the value stored in %eax.
    Cdq,
    /// Allocate stack instruction: allocates a specified amount of stack space.
    AllocateStack { stack_offset: i32 },
    /// Return instruction: signifies the end of a function execution.
    Ret,
}

/// Represents an unary operator.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyUnaryOperator {
    /// Negation instruction
    Neg,
    /// Bitwise NOT instruction
    Not,
}

/// Represents a binary operator.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyBinaryOperator {
    Add,
    Sub,
    Mult,
}

/// Represents an operand for an instruction, which can be an immediate value or a register.
#[derive(Debug, PartialEq, Clone)]
pub enum AssemblyOperand {
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
    /// DX CPU register
    DX,
    /// R10 scratch register
    R10,
    /// R11 scratch register
    R11,
}
