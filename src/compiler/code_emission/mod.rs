use crate::compiler::code_gen::assembly_ast::{
    AssemblyAst, AssemblyBinaryOperator, AssemblyConditionCode, AssemblyFunction,
    AssemblyInstruction, AssemblyOperand, AssemblyRegister, AssemblyUnaryOperator,
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
        AssemblyAst::Program { function } => emit_function(function),
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
            let mut function_code = wrap_instruction(format!(".globl {}", asm_identifier).as_str());
            function_code.push_str(&wrap_label(asm_identifier.as_str()));
            let prologue = wrap_instruction("pushq %rbp") + &wrap_instruction("movq %rsp, %rbp");
            function_code.push_str(&prologue);
            for instruction in instructions {
                function_code.push_str(&format_instruction(instruction));
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
fn format_instruction(instruction: &AssemblyInstruction) -> String {
    match instruction {
        AssemblyInstruction::Mov {
            source,
            destination,
        } => wrap_instruction(
            format!(
                "movl {}, {}",
                format_operand(source, false),
                format_operand(destination, false)
            )
            .as_str(),
        ),
        AssemblyInstruction::Unary { op, operand } => wrap_instruction(
            format!(
                "{} {}",
                format_unary_operator(op),
                format_operand(operand, false)
            )
            .as_str(),
        ),
        AssemblyInstruction::Binary {
            op,
            source,
            destination,
        } => wrap_instruction(
            format!(
                "{} {}, {}",
                format_binary_operator(op),
                format_operand(source, false),
                format_operand(destination, false)
            )
            .as_str(),
        ),
        AssemblyInstruction::Cmp { left, right } => wrap_instruction(
            format!(
                "cmpl {}, {}",
                format_operand(left, false),
                format_operand(right, false)
            )
            .as_str(),
        ),
        AssemblyInstruction::Idiv { operand } => {
            wrap_instruction(format!("idivl {}", format_operand(operand, false)).as_str())
        }
        AssemblyInstruction::AllocateStack { stack_offset } => {
            wrap_instruction(format!("subq ${}, %rsp", stack_offset).as_str())
        }
        AssemblyInstruction::Cdq => wrap_instruction("cdq"),
        AssemblyInstruction::Jmp { label } => wrap_instruction(format!("jmp L{}", label).as_str()),
        AssemblyInstruction::JmpCC { condition, label } => wrap_instruction(
            format!("j{} L{}", transform_condition_code(condition), label,).as_str(),
        ),
        AssemblyInstruction::SetCC { condition, operand } => wrap_instruction(
            format!(
                "set{} {}",
                transform_condition_code(condition),
                format_operand(operand, true)
            )
            .as_str(),
        ),
        AssemblyInstruction::Label(label) => wrap_label(format!("L{}", label).as_str()),
        AssemblyInstruction::Ret => {
            let mut epilogue = wrap_instruction("movq %rbp, %rsp").to_string();
            epilogue.push_str(wrap_instruction("popq %rbp").as_str());
            epilogue.push_str(wrap_instruction("ret").as_str());
            epilogue
        }
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
fn format_unary_operator(op: &AssemblyUnaryOperator) -> String {
    match op {
        AssemblyUnaryOperator::Neg => "negl".to_string(),
        AssemblyUnaryOperator::Not => "notl".to_string(),
    }
}

/// Converts a `BinaryOp` to its corresponding string representation.
///
/// # Arguments
///
/// * `op`: The `BinaryOp` to convert.
///
/// # Returns
///
/// A string representing the binary operation.
fn format_binary_operator(op: &AssemblyBinaryOperator) -> String {
    match op {
        AssemblyBinaryOperator::Add => "addl".to_string(),
        AssemblyBinaryOperator::Sub => "subl".to_string(),
        AssemblyBinaryOperator::Mult => "imull".to_string(),
    }
}

/// Emits assembly code for an operand.
///
/// # Arguments
///
/// * `operand`: A reference to the `Operand` to be emitted.
/// * `use_1byte_representation`: A boolean flag indicating whether to use the 1-byte register representation. 4-byte register representation is used, if false.
///
/// # Returns
///
/// A `String` representing the assembly code for the operand.
fn format_operand(operand: &AssemblyOperand, use_1byte_representation: bool) -> String {
    match operand {
        AssemblyOperand::Imm(value) => format_immediate_value(value),
        AssemblyOperand::Register(register) => format_register(register, use_1byte_representation),
        AssemblyOperand::Stack(offset) => format_stack_offset(offset),
        AssemblyOperand::Pseudo(_) => panic!(
            "Pseudo registers should not be emitted to assembly. Have you converted them correctly to actual register addresses?"
        ),
    }
}

/// Maps a `Register` enum variant to its assembly syntax representation.
///
/// # Arguments
///
/// * `register`: The `Register` enum variant to convert.
/// * `use_1byte_representation`: A boolean flag indicating whether to use the 1-byte register representation. 4-byte register representation is used, if false.
///
/// # Returns
///
/// A `String` representing the AT&T assembly syntax for the given register.
fn format_register(register: &AssemblyRegister, use_1byte_representation: bool) -> String {
    match register {
        AssemblyRegister::AX => match use_1byte_representation {
            true => "%al".to_string(),
            false => "%eax".to_string(),
        },
        AssemblyRegister::DX => match use_1byte_representation {
            true => "%dl".to_string(),
            false => "%edx".to_string(),
        },
        AssemblyRegister::R10 => match use_1byte_representation {
            true => "%r10b".to_string(),
            false => "%r10d".to_string(),
        },
        AssemblyRegister::R11 => match use_1byte_representation {
            true => "%r11b".to_string(),
            false => "%r11d".to_string(),
        },
    }
}

/// Formats an integer as an immediate value string, prefixed with a dollar sign.
///
/// # Arguments
///
/// * `value` - A reference to the i32 integer to format.
///
/// # Returns
///
/// A String representing the formatted immediate value (e.g., "$123").
fn format_immediate_value(value: &i32) -> String {
    format!("${}", value)
}

/// Formats a stack offset as a string.
///
/// # Arguments
///
/// * `offset` - A reference to the i32 integer to format.
///
/// # Returns
///
/// A String representing the formatted stack offset (e.g., "-4(%rbp)").
fn format_stack_offset(offset: &i32) -> String {
    format!("{}(%rbp)", offset)
}

/// Wraps a label with a colon and newline
///
/// # Arguments
///
/// * `label`: The string slice to format.
///
/// # Returns
///
/// A `String` containing the formatted label.
///
fn wrap_label(label: &str) -> String {
    format!("{}:\n", label)
}

///  Wraps an assembly instruction with a tab and newline
///
/// # Arguments
///
/// * `instruction` - The instruction string to format.
///
/// # Returns
///
/// A formatted string with a tab at the beginning and a newline at the end.
fn wrap_instruction(instruction: &str) -> String {
    format!("\t{}\n", instruction)
}

/// Converts an `AssemblyConditionCode` enum variant into its corresponding string representation.
///
/// # Arguments
///
/// * `condition_code` - The `AssemblyConditionCode` enum variant to convert.
///
/// # Returns
///
/// A `String` representing the condition code (e.g., "NE", "EQ").
fn transform_condition_code(condition_code: &AssemblyConditionCode) -> String {
    match condition_code {
        AssemblyConditionCode::E => "e".to_string(),
        AssemblyConditionCode::NE => "ne".to_string(),
        AssemblyConditionCode::G => "g".to_string(),
        AssemblyConditionCode::L => "l".to_string(),
        AssemblyConditionCode::GE => "ge".to_string(),
        AssemblyConditionCode::LE => "le".to_string(),
    }
}
