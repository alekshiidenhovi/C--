/// Represents an abstract syntax tree for assembly code.
#[derive(Debug, PartialEq)]
pub enum AssemblyAst {
    /// Represents a complete program, containing a single function definition.
    Program(FunctionDefinition),
}

/// Represents the definition of a function.
#[derive(Debug, PartialEq)]
pub enum FunctionDefinition {
    /// A function with a name and a list of instructions.
    Function(String, Vec<Instruction>),
}

/// Represents a single instruction in the assembly code.
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Move operation: copies a value from a source operand to a destination operand.
    Mov(Operand, Operand),
    /// Return instruction: signifies the end of a function execution.
    Ret,
}

/// Represents an operand for an instruction, which can be an immediate value or a register.
#[derive(Debug, PartialEq)]
pub enum Operand {
    /// An immediate integer value.
    Imm(i32),
    /// A register.
    Register,
}
