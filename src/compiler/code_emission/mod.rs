use crate::compiler::code_gen::assembly_ast::{
    AssemblyAst, AssemblyFunction, AssemblyInstruction, AssemblyRegister, AssemblyUnaryOperand,
    AssemblyUnaryOperation,
};

/// Emits assembly code from an abstract syntax tree.
///
/// # Arguments
///
/// * `assembly_ast`: A reference to the `AssemblyAst` to be converted into assembly code.
///
/// # Returns
///
/// A `String` containing the generated assembly code.
pub fn emit_assembly(assembly_ast: &AssemblyAst) -> String {
    match assembly_ast {
        AssemblyAst::Program(function) => emit_function(function),
    }
}

/// Emits assembly code for a single function definition.
///
/// # Arguments
///
/// * `function`: A reference to the `AssemblyFunction` to be emitted.
///
/// # Returns
///
/// A `String` representing the assembly code for the function.
fn emit_function(function: &AssemblyFunction) -> String {
    match function {
        AssemblyFunction::Function {
            identifier,
            instructions,
        } => {
            let asm_identifier = "_".to_string() + identifier;
            let mut function_code = format!("\t.globl {}\n", asm_identifier);
            function_code.push_str(&format!("{}:\n", asm_identifier));
            function_code.push_str(&format!("\tpushq %rbp\n"));
            function_code.push_str(&format!("\tmovq %rsp, %rbp\n"));
            for instruction in instructions {
                function_code.push_str(&format!("{}\n", emit_instruction(instruction)));
            }
            function_code
        }
    }
}

/// Emits assembly code for a single instruction.
///
/// # Arguments
///
/// * `instruction`: A reference to the `Instruction` to be emitted.
///
/// # Returns
///
/// A `String` representing the assembly code for the instruction.
fn emit_instruction(instruction: &AssemblyInstruction) -> String {
    match instruction {
        AssemblyInstruction::Mov {
            source,
            destination,
        } => format!(
            "\tmovl {}, {}",
            emit_operand(source),
            emit_operand(destination)
        ),
        AssemblyInstruction::Ret => {
            let mut epilogue = "\tmovq %rbp, %rsp\n".to_string();
            epilogue.push_str("\tpopq %rbp\n");
            epilogue.push_str("\tret\n");
            epilogue
        }
        AssemblyInstruction::Unary { op, operand } => {
            format!("\t{} {}", emit_unary_op(op), emit_operand(operand))
        }
        AssemblyInstruction::AllocateStack(stack_size) => format!("\tsubq ${}, %rsp", stack_size),
    }
}

/// Converts a `UnaryOp` to its corresponding string representation.
///
/// # Arguments
///
/// * `op`: The `UnaryOp` to convert.
///
/// # Returns
///
/// A string representing the unary operation.
fn emit_unary_op(op: &AssemblyUnaryOperation) -> String {
    match op {
        AssemblyUnaryOperation::Neg => "negl".to_string(),
        AssemblyUnaryOperation::Not => "notl".to_string(),
    }
}

/// Emits assembly code for an operand.
///
/// # Arguments
///
/// * `operand`: A reference to the `Operand` to be emitted.
///
/// # Returns
///
/// A `String` representing the assembly code for the operand.
fn emit_operand(operand: &AssemblyUnaryOperand) -> String {
    match operand {
        AssemblyUnaryOperand::Imm(value) => format!("${}", value),
        AssemblyUnaryOperand::Register(register) => format!("{}", emit_register(register)),
        AssemblyUnaryOperand::Stack(offset) => format!("{offset}(%rbp)", offset = offset),
        AssemblyUnaryOperand::Pseudo(_) => panic!(
            "Pseudo registers should not be emitted to assembly. Have you converted them correctly to actual register addresses?"
        ),
    }
}

/// Maps a `Register` enum variant to its assembly syntax representation.
///
/// # Arguments
///
/// * `register`: The `Register` enum variant to convert.
///
/// # Returns
///
/// A `String` representing the AT&T assembly syntax for the given register.
fn emit_register(register: &AssemblyRegister) -> String {
    match register {
        AssemblyRegister::AX => "%eax".to_string(),
        AssemblyRegister::R10 => "%r10d".to_string(),
    }
}
