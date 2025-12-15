pub mod assembly_ast;
pub mod constants;
pub mod errors;

use crate::compiler::ir_gen::tacky_ast::{
    TackyAst, TackyFunction, TackyInstruction, TackyUnaryOperator, TackyValue,
};
use assembly_ast::{
    AssemblyAst, AssemblyFunction, AssemblyInstruction, AssemblyRegister, AssemblyUnaryOperand,
    AssemblyUnaryOperation,
};
use errors::CodegenError;
use std::collections::HashMap;

/// Converts the entire TACKY IR into an assembly AST.
///
/// This is the main entry point for the conversion process.
///
/// # Returns
///
/// A `Result` containing the generated `AssemblyAst` on success,
/// or a `CodegenError` on failure.
///
/// # Examples
///
/// ```
/// # use cmm::compiler::tokens::Token;
/// # use cmm::compiler::ir_gen::tacky_ast::{TackyFunction, TackyAst, TackyInstruction, TackyUnaryOperator, TackyValue};
/// # use cmm::compiler::code_gen::convert_ast;
/// # use cmm::compiler::code_gen::assembly_ast::{AssemblyAst, AssemblyFunction, AssemblyInstruction, AssemblyUnaryOperand, AssemblyUnaryOperation, AssemblyRegister};
/// # use cmm::compiler::code_gen::errors::CodegenError;
/// let identifier = "main".to_string();
/// let temp_name = "tmp.0".to_string();
/// let tacky_ast = TackyAst::Program{ function: TackyFunction::Function {
///     identifier: identifier.clone(),
///     instructions: vec![
///         TackyInstruction::Unary {
///             operator: TackyUnaryOperator::Negate,
///             source: TackyValue::Constant(1),
///             destination: TackyValue::Variable(temp_name.clone()),
///         },
///         TackyInstruction::Return { value: TackyValue::Variable(temp_name) },
///     ],
/// } };
/// let assembly_ast = convert_ast(tacky_ast)?;
/// assert_eq!(assembly_ast, AssemblyAst::Program(AssemblyFunction::Function {
///     identifier,
///     instructions: vec![
///         AssemblyInstruction::AllocateStack(-4),
///         AssemblyInstruction::Mov {
///             source: AssemblyUnaryOperand::Imm(1),
///             destination: AssemblyUnaryOperand::Stack(-4),
///         },
///         AssemblyInstruction::Unary {
///             op: AssemblyUnaryOperation::Neg,
///             operand: AssemblyUnaryOperand::Stack(-4),
///         },
///         AssemblyInstruction::Mov {
///             source: AssemblyUnaryOperand::Stack(-4),
///             destination: AssemblyUnaryOperand::Register(AssemblyRegister::AX),
///         },
///         AssemblyInstruction::Ret,
///     ],
/// }));
/// # Ok::<(), CodegenError>(())
/// ```
pub fn convert_ast(tacky_ast: TackyAst) -> Result<AssemblyAst, CodegenError> {
    let mut asm_ast = match tacky_ast {
        TackyAst::Program { function } => AssemblyAst::Program(convert_function(&function)?),
    };
    let stack_offset = replace_pseudo_registers(&mut asm_ast);
    let asm_ast = fixup_instructions(asm_ast, stack_offset);
    Ok(asm_ast)
}

/// Replaces pseudo registers with actual registers in the assembly AST.
///
/// # Arguments
///
/// * `asm_ast` - The assembly AST to be modified.
///
/// # Returns
///
/// The final stack offset after replacing pseudo registers.
fn replace_pseudo_registers(asm_ast: &mut AssemblyAst) -> i32 {
    let mut identifier_offsets: HashMap<String, i32> = HashMap::new();
    let mut offset_counter = 0;
    match asm_ast {
        AssemblyAst::Program(asm_function) => match asm_function {
            AssemblyFunction::Function {
                identifier: _,
                instructions,
            } => {
                for instruction in instructions.iter_mut() {
                    match instruction {
                        AssemblyInstruction::Mov {
                            source,
                            destination,
                        } => {
                            convert_pseudo_register(
                                source,
                                &mut identifier_offsets,
                                &mut offset_counter,
                            );
                            convert_pseudo_register(
                                destination,
                                &mut identifier_offsets,
                                &mut offset_counter,
                            );
                        }
                        AssemblyInstruction::Unary { op: _, operand } => {
                            convert_pseudo_register(
                                operand,
                                &mut identifier_offsets,
                                &mut offset_counter,
                            );
                        }
                        _ => {}
                    }
                }
            }
        },
    };
    offset_counter
}

/// Converts a pseudo-register operand to a stack operand.
///
/// This function takes an `Operand` and attempts to convert it from a `Pseudo` variant
/// (representing an identifier) to a `Stack` variant (representing a memory offset).
///
/// # Arguments
///
/// * `operand`: A mutable reference to the `Operand` to be converted. If it's a `Pseudo`
///   variant, it will be modified in place to become a `Stack` variant.
/// * `identifier_offsets`: A mutable reference to a `HashMap` that maps identifier strings
///   to their allocated stack offsets (`i32`).
/// * `offset_counter`: A mutable reference to an `i32` that acts as a counter for
///   allocating new stack offsets. It is decremented for each new identifier.
///
/// # Returns
///
/// This function does not return a value, but it modifies the `operand` argument in place.
fn convert_pseudo_register(
    operand: &mut AssemblyUnaryOperand,
    identifier_offsets: &mut HashMap<String, i32>,
    offset_counter: &mut i32,
) -> () {
    match operand {
        AssemblyUnaryOperand::Pseudo(identifier) => {
            if let Some(offset) = identifier_offsets.get(identifier) {
                *operand = AssemblyUnaryOperand::Stack(*offset);
                return;
            }
            *offset_counter -= constants::STACK_ADDRESS_OFFSET;
            identifier_offsets.insert(identifier.clone(), *offset_counter);
            *operand = AssemblyUnaryOperand::Stack(*offset_counter);
        }
        _ => {}
    }
}

/// Fixes up instructions by allocating stack space and resolving memory-to-memory operations.
///
/// # Arguments
///
/// * `asm_ast`: The `AssemblyAst` to process.
/// * `stack_offset`: The total stack space in bytes to allocate for the function.
///
/// # Returns
///
/// A new `AssemblyAst` with the instructions fixed up.
fn fixup_instructions(asm_ast: AssemblyAst, stack_offset: i32) -> AssemblyAst {
    match asm_ast {
        AssemblyAst::Program(asm_function) => match asm_function {
            AssemblyFunction::Function {
                identifier,
                instructions,
            } => {
                let instructions = allocate_stack_space(instructions.clone(), stack_offset.clone());
                let mut fixed_instructions = vec![];
                for instruction in instructions.iter() {
                    fixed_instructions.append(&mut fixup_memory_to_memory_operation(instruction));
                }
                AssemblyAst::Program(AssemblyFunction::Function {
                    identifier: identifier.to_string(),
                    instructions: fixed_instructions,
                })
            }
        },
    }
}

/// Inserts an instruction to allocate stack space at the beginning of the instruction list.
///
/// # Arguments
///
/// * `instructions` - The vector of instructions to modify.
/// * `stack_offset` - The amount of stack space to allocate.
///
/// # Returns
///
/// A new vector of instructions with the `AllocateStack` instruction prepended
fn allocate_stack_space(
    mut instructions: Vec<AssemblyInstruction>,
    stack_offset: i32,
) -> Vec<AssemblyInstruction> {
    instructions.insert(0, AssemblyInstruction::AllocateStack(stack_offset));
    instructions
}

fn fixup_memory_to_memory_operation(
    asm_instruction: &AssemblyInstruction,
) -> Vec<AssemblyInstruction> {
    let scratch_register_operand = AssemblyUnaryOperand::Register(AssemblyRegister::R10);
    match asm_instruction {
        AssemblyInstruction::Mov {
            source,
            destination,
        } => match (source, destination) {
            (AssemblyUnaryOperand::Stack(_), AssemblyUnaryOperand::Stack(_)) => {
                let move1 = AssemblyInstruction::Mov {
                    source: source.clone(),
                    destination: scratch_register_operand.clone(),
                };
                let move2 = AssemblyInstruction::Mov {
                    source: scratch_register_operand.clone(),
                    destination: destination.clone(),
                };
                vec![move1, move2]
            }
            _ => vec![asm_instruction.clone()],
        },
        _ => vec![asm_instruction.clone()],
    }
}

///
/// # Arguments
///
///  * `tacky_function` - A reference to the TACKY `TackyFunction` to convert.
///
/// # Returns
///
/// A `Result` containing the generated `AssemblyFunction` on success,
/// or a `CodegenError` on failure.
fn convert_function(tacky_function: &TackyFunction) -> Result<AssemblyFunction, CodegenError> {
    let function = match tacky_function {
        TackyFunction::Function {
            identifier,
            instructions: tacky_instructions,
        } => AssemblyFunction::Function {
            identifier: identifier.clone(),
            instructions: convert_instructions(tacky_instructions),
        },
    };
    Ok(function)
}

/// Converts TACKY instructions into assembly instructions.
///
/// # Arguments
///
///  * `tacky_instructions` - A reference to the TACKY `TackyInstruction`s to convert.
///
/// # Returns
///
/// A `Result` containing a vector of `AssemblyInstruction`s on success,
/// or a `CodegenError` on failure.
fn convert_instructions(tacky_instructions: &Vec<TackyInstruction>) -> Vec<AssemblyInstruction> {
    let mut asm_instructions = vec![];
    for tacky_instruction in tacky_instructions.iter() {
        match tacky_instruction {
            TackyInstruction::Return { value } => {
                let mov_instruction = AssemblyInstruction::Mov {
                    source: convert_operand(&value),
                    destination: AssemblyUnaryOperand::Register(AssemblyRegister::AX),
                };
                let ret_instruction = AssemblyInstruction::Ret;
                asm_instructions.push(mov_instruction);
                asm_instructions.push(ret_instruction);
            }
            TackyInstruction::Unary {
                operator,
                source,
                destination,
            } => {
                let mov_instruction = AssemblyInstruction::Mov {
                    source: convert_operand(&source),
                    destination: convert_operand(&destination),
                };
                let unary_instruction = AssemblyInstruction::Unary {
                    op: convert_operator(&operator),
                    operand: convert_operand(&destination),
                };
                asm_instructions.push(mov_instruction);
                asm_instructions.push(unary_instruction);
            }
        }
    }
    asm_instructions
}

fn convert_operator(tacky_operator: &TackyUnaryOperator) -> AssemblyUnaryOperation {
    match tacky_operator {
        TackyUnaryOperator::Negate => AssemblyUnaryOperation::Neg,
        TackyUnaryOperator::Complement => AssemblyUnaryOperation::Not,
    }
}

fn convert_operand(tacky_operand: &TackyValue) -> AssemblyUnaryOperand {
    match tacky_operand {
        TackyValue::Constant(value) => AssemblyUnaryOperand::Imm(*value),
        TackyValue::Variable(name) => AssemblyUnaryOperand::Pseudo(name.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_instructions_success() {
        let identifier = "tmp.0".to_string();
        let tacky_instructions = vec![
            TackyInstruction::Unary {
                operator: TackyUnaryOperator::Negate,
                source: TackyValue::Constant(1),
                destination: TackyValue::Variable(identifier.clone()),
            },
            TackyInstruction::Return {
                value: TackyValue::Variable(identifier.clone()),
            },
        ];
        let result = convert_instructions(&tacky_instructions);
        assert_eq!(
            result,
            vec![
                AssemblyInstruction::Mov {
                    source: AssemblyUnaryOperand::Imm(1),
                    destination: AssemblyUnaryOperand::Pseudo(identifier.clone()),
                },
                AssemblyInstruction::Unary {
                    op: AssemblyUnaryOperation::Neg,
                    operand: AssemblyUnaryOperand::Pseudo(identifier.clone()),
                },
                AssemblyInstruction::Mov {
                    source: AssemblyUnaryOperand::Pseudo(identifier.clone()),
                    destination: AssemblyUnaryOperand::Register(AssemblyRegister::AX),
                },
                AssemblyInstruction::Ret,
            ]
        );
    }

    #[test]
    fn test_replace_pseudo_registers_success() {
        let identifier = "main".to_string();
        let pseudo_register_name = "tmp.0".to_string();
        let mut asm_ast = AssemblyAst::Program(AssemblyFunction::Function {
            identifier: identifier.clone(),
            instructions: vec![
                AssemblyInstruction::Mov {
                    source: AssemblyUnaryOperand::Imm(1),
                    destination: AssemblyUnaryOperand::Pseudo(pseudo_register_name),
                },
                AssemblyInstruction::Ret,
            ],
        });
        let offset = replace_pseudo_registers(&mut asm_ast);
        assert_eq!(offset, -4);
        assert_eq!(
            asm_ast,
            AssemblyAst::Program(AssemblyFunction::Function {
                identifier,
                instructions: vec![
                    AssemblyInstruction::Mov {
                        source: AssemblyUnaryOperand::Imm(1),
                        destination: AssemblyUnaryOperand::Stack(-4),
                    },
                    AssemblyInstruction::Ret,
                ],
            })
        );
    }
}
