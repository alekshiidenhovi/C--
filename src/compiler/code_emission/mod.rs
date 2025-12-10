use crate::compiler::codegen::asm_ast::{
    AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand,
};

pub fn emit_assembly(assembly_ast: &AssemblyAst) -> String {
    match assembly_ast {
        AssemblyAst::Program(function) => emit_function(function),
    }
}

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

fn emit_operand(operand: &Operand) -> String {
    match operand {
        Operand::Imm(value) => format!("${}", value),
        Operand::Register => "%eax".to_string(),
    }
}
