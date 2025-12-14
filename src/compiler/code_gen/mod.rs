pub mod asm_ast;
pub mod constants;
pub mod errors;

use crate::compiler::ir_gen::tacky_ir::{
    TackyFunction, TackyIR, TackyInstruction, TackyUnaryOperator, TackyValue,
};
use asm_ast::{AssemblyAst, FunctionDefinition, Instruction, Operand, Register, UnaryOp};
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
/// # use cmm::compiler::ir_gen::tacky_ir::{TackyFunction, TackyIR, TackyInstruction, TackyUnaryOperator, TackyValue};
/// # use cmm::compiler::code_gen::convert_ast;
/// # use cmm::compiler::code_gen::asm_ast::{AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand, UnaryOp, Register};
/// # use cmm::compiler::code_gen::errors::CodegenError;
/// # use std::collections::LinkedList;
/// let identifier = "main".to_string();
/// let temp_name = "tmp.0".to_string();
/// let tacky_ast = TackyIR::Program(TackyFunction::Function {
///     identifier: identifier.clone(),
///     instructions: vec![
///         TackyInstruction::Unary {
///             operator: TackyUnaryOperator::Negate,
///             source: TackyValue::Constant(1),
///             destination: TackyValue::Variable(temp_name.clone()),
///         },
///         TackyInstruction::Return(TackyValue::Variable(temp_name)),
///     ],
/// });
/// let assembly_ast = convert_ast(tacky_ast)?;
/// assert_eq!(assembly_ast, AssemblyAst::Program(AsmFunctionDefinition::Function {
///     identifier,
///     instructions: vec![
///         Instruction::AllocateStack(-4),
///         Instruction::Mov {
///             source: Operand::Imm(1),
///             destination: Operand::Stack(-4),
///         },
///         Instruction::Unary {
///             op: UnaryOp::Neg,
///             operand: Operand::Stack(-4),
///         },
///         Instruction::Mov {
///             source: Operand::Stack(-4),
///             destination: Operand::Register(Register::AX),
///         },
///         Instruction::Ret,
///     ],
/// }));
/// # Ok::<(), CodegenError>(())
/// ```
pub fn convert_ast(tacky_ast: TackyIR) -> Result<AssemblyAst, CodegenError> {
    let mut asm_ast = match tacky_ast {
        TackyIR::Program(c_function) => AssemblyAst::Program(convert_function(&c_function)?),
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
            FunctionDefinition::Function {
                identifier: _,
                instructions,
            } => {
                for instruction in instructions.iter_mut() {
                    match instruction {
                        Instruction::Mov {
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
                        Instruction::Unary { op: _, operand } => {
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
    operand: &mut Operand,
    identifier_offsets: &mut HashMap<String, i32>,
    offset_counter: &mut i32,
) -> () {
    match operand {
        Operand::Pseudo(identifier) => {
            if let Some(offset) = identifier_offsets.get(identifier) {
                *operand = Operand::Stack(*offset);
                return;
            }
            *offset_counter -= constants::STACK_ADDRESS_OFFSET;
            identifier_offsets.insert(identifier.clone(), *offset_counter);
            *operand = Operand::Stack(*offset_counter);
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
            FunctionDefinition::Function {
                identifier,
                instructions,
            } => {
                let instructions = allocate_stack_space(instructions.clone(), stack_offset.clone());
                let mut fixed_instructions = vec![];
                for instruction in instructions.iter() {
                    fixed_instructions.append(&mut fixup_memory_to_memory_operation(instruction));
                }
                AssemblyAst::Program(FunctionDefinition::Function {
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
/// A new vector of instructions with the `AllocateStack` instruction prepended.
fn allocate_stack_space(mut instructions: Vec<Instruction>, stack_offset: i32) -> Vec<Instruction> {
    instructions.insert(0, Instruction::AllocateStack(stack_offset));
    instructions
}

fn fixup_memory_to_memory_operation(asm_instruction: &Instruction) -> Vec<Instruction> {
    let scratch_register_operand = Operand::Register(Register::R10);
    match asm_instruction {
        Instruction::Mov {
            source,
            destination,
        } => match (source, destination) {
            (Operand::Stack(_), Operand::Stack(_)) => {
                let move1 = Instruction::Mov {
                    source: source.clone(),
                    destination: scratch_register_operand.clone(),
                };
                let move2 = Instruction::Mov {
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
/// A `Result` containing the generated `AsmFunctionDefinition` on success,
/// or a `CodegenError` on failure.
fn convert_function(tacky_function: &TackyFunction) -> Result<FunctionDefinition, CodegenError> {
    let function = match tacky_function {
        TackyFunction::Function {
            identifier,
            instructions: tacky_instructions,
        } => FunctionDefinition::Function {
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
/// A `Result` containing a vector of `Instruction`s on success,
/// or a `CodegenError` on failure.
fn convert_instructions(tacky_instructions: &Vec<TackyInstruction>) -> Vec<Instruction> {
    let mut asm_instructions = vec![];
    for tacky_instruction in tacky_instructions.iter() {
        match tacky_instruction {
            TackyInstruction::Return(tacky_value) => {
                let mov_instruction = Instruction::Mov {
                    source: convert_operand(&tacky_value),
                    destination: Operand::Register(Register::AX),
                };
                let ret_instruction = Instruction::Ret;
                asm_instructions.push(mov_instruction);
                asm_instructions.push(ret_instruction);
            }
            TackyInstruction::Unary {
                operator,
                source,
                destination,
            } => {
                let mov_instruction = Instruction::Mov {
                    source: convert_operand(&source),
                    destination: convert_operand(&destination),
                };
                let unary_instruction = Instruction::Unary {
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

fn convert_operator(tacky_operator: &TackyUnaryOperator) -> UnaryOp {
    match tacky_operator {
        TackyUnaryOperator::Negate => UnaryOp::Neg,
        TackyUnaryOperator::Complement => UnaryOp::Not,
    }
}

fn convert_operand(tacky_operand: &TackyValue) -> Operand {
    match tacky_operand {
        TackyValue::Constant(value) => Operand::Imm(*value),
        TackyValue::Variable(name) => Operand::Pseudo(name.clone()),
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
            TackyInstruction::Return(TackyValue::Variable(identifier.clone())),
        ];
        let result = convert_instructions(&tacky_instructions);
        assert_eq!(
            result,
            vec![
                Instruction::Mov {
                    source: Operand::Imm(1),
                    destination: Operand::Pseudo(identifier.clone()),
                },
                Instruction::Unary {
                    op: UnaryOp::Neg,
                    operand: Operand::Pseudo(identifier.clone()),
                },
                Instruction::Mov {
                    source: Operand::Pseudo(identifier.clone()),
                    destination: Operand::Register(Register::AX),
                },
                Instruction::Ret,
            ]
        );
    }

    #[test]
    fn test_replace_pseudo_registers_success() {
        let identifier = "main".to_string();
        let pseudo_register_name = "tmp.0".to_string();
        let mut asm_ast = AssemblyAst::Program(FunctionDefinition::Function {
            identifier: identifier.clone(),
            instructions: vec![
                Instruction::Mov {
                    source: Operand::Imm(1),
                    destination: Operand::Pseudo(pseudo_register_name),
                },
                Instruction::Ret,
            ],
        });
        let offset = replace_pseudo_registers(&mut asm_ast);
        assert_eq!(offset, -4);
        assert_eq!(
            asm_ast,
            AssemblyAst::Program(FunctionDefinition::Function {
                identifier,
                instructions: vec![
                    Instruction::Mov {
                        source: Operand::Imm(1),
                        destination: Operand::Stack(-4),
                    },
                    Instruction::Ret,
                ],
            })
        );
    }
}
