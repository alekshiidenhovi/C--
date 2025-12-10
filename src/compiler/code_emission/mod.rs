use crate::compiler::codegen::asm_ast::{
    AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand,
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
/// * `function`: A reference to the `AsmFunctionDefinition` to be emitted.
///
/// # Returns
///
/// A `String` representing the assembly code for the function.
fn emit_function(function: &AsmFunctionDefinition) -> String {
    match function {
        AsmFunctionDefinition::Function {
            identifier,
            instructions,
        } => {
            let mut function_code = format!("\t.globl _{}\n", identifier);
            function_code.push_str(&format!("_{}:\n", identifier));
            for instruction in instructions {
                function_code.push_str(&format!("\t{}\n", emit_instruction(instruction)));
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
fn emit_instruction(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Mov {
            source,
            destination,
        } => format!(
            "mov {}, {}",
            emit_operand(source),
            emit_operand(destination)
        ),
        Instruction::Ret => "ret".to_string(),
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
fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${}", value),
        Operand::Register => "%eax".to_string(),
    }
}
