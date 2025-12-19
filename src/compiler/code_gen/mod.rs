pub mod assembly_ast;
pub mod constants;
pub mod errors;

use crate::compiler::ir_gen::tacky_ast::{
    TackyAst, TackyBinaryOperator, TackyFunction, TackyInstruction, TackyUnaryOperator, TackyValue,
};
use assembly_ast::{
    AssemblyAst, AssemblyBinaryOperator, AssemblyFunction, AssemblyInstruction, AssemblyOperand,
    AssemblyRegister, AssemblyUnaryOperator,
};
use errors::CodegenError;
use std::collections::HashMap;

/// Converts the entire TACKY IR into an assembly AST.
///
/// This is the main entry point for the conversion process.
///
/// # Returns
///
/// A `Result` containing the generated `AssemblyAst` on success, or a `CodegenError` on failure.
///
/// # Examples
///
/// ```
/// # use cmm::compiler::ir_gen::tacky_ast::{TackyFunction, TackyAst, TackyInstruction, TackyUnaryOperator, TackyValue};
/// # use cmm::compiler::code_gen::convert_ast;
/// # use cmm::compiler::code_gen::assembly_ast::{AssemblyAst, AssemblyFunction, AssemblyInstruction, AssemblyOperand, AssemblyUnaryOperator, AssemblyRegister};
/// # use cmm::compiler::code_gen::errors::CodegenError;
/// let identifier = "main".to_string();
/// let temp_0_name = "tmp.0".to_string();
/// let temp_1_name = "tmp.1".to_string();
/// let tacky_ast = TackyAst::Program{ function: TackyFunction::Function {
///     identifier: identifier.clone(),
///     instructions: vec![
///         TackyInstruction::Unary {
///             operator: TackyUnaryOperator::Negate,
///             source: TackyValue::Constant(1),
///             destination: TackyValue::Variable(temp_0_name.clone()),
///         },
///         TackyInstruction::Unary {
///             operator: TackyUnaryOperator::Complement,
///             source: TackyValue::Variable(temp_0_name),
///             destination: TackyValue::Variable(temp_1_name.clone()),
///         },
///         TackyInstruction::Return { value: TackyValue::Variable(temp_1_name) },
///     ],
/// } };
/// let assembly_ast = convert_ast(tacky_ast)?;
/// assert_eq!(assembly_ast, AssemblyAst::Program{ function: AssemblyFunction::Function {
///     identifier,
///     instructions: vec![
///         AssemblyInstruction::AllocateStack { stack_offset: -8 },
///         AssemblyInstruction::Mov {
///             source: AssemblyOperand::Imm(1),
///             destination: AssemblyOperand::Stack(-4),
///         },
///         AssemblyInstruction::Unary {
///             op: AssemblyUnaryOperator::Neg,
///             operand: AssemblyOperand::Stack(-4),
///         },
///         AssemblyInstruction::Mov {
///             source: AssemblyOperand::Stack(-4),
///             destination: AssemblyOperand::Register(AssemblyRegister::R10),
///         },
///         AssemblyInstruction::Mov {
///             source: AssemblyOperand::Register(AssemblyRegister::R10),
///             destination: AssemblyOperand::Stack(-8),
///         },
///         AssemblyInstruction::Unary {
///             op: AssemblyUnaryOperator::Not,
///             operand: AssemblyOperand::Stack(-8),
///         },
///         AssemblyInstruction::Mov {
///             source: AssemblyOperand::Stack(-8),
///             destination: AssemblyOperand::Register(AssemblyRegister::AX),
///         },
///         AssemblyInstruction::Ret,
///     ],
/// } });
/// # Ok::<(), CodegenError>(())
/// ```
pub fn convert_ast(tacky_ast: TackyAst) -> Result<AssemblyAst, CodegenError> {
    match tacky_ast {
        TackyAst::Program { function } => Ok(AssemblyAst::Program {
            function: convert_function(&function)?,
        }),
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
            instructions: convert_instructions(&tacky_instructions),
        },
    };
    Ok(function)
}

/// Converts TACKY instructions into assembly instructions.
///
/// Conversion takes four passes:
/// 1. Convert TACKY instructions into assembly instructions. No physical registers are assigned during this pass.
/// 2. Replace pseudo registers with physical registers in the assembly instructions.
/// 3. Allocate stack space for local variables.
/// 4. Fixup instructions by allocating stack space and resolving memory-to-memory operations.
///
/// # Arguments
///
/// * `tacky_instructions` - A reference to the TACKY `TackyInstruction`s to convert.
///
/// # Returns
///
/// A `Result` containing a vector of `AssemblyInstruction`s on success,
/// or a `CodegenError` on failure.
fn convert_instructions(tacky_instructions: &Vec<TackyInstruction>) -> Vec<AssemblyInstruction> {
    let mut asm_instructions = instruction_conversion_pass(tacky_instructions);
    let stack_offset = pseudoregister_replacement_pass(&mut asm_instructions);
    let mut final_instructions = vec![stack_allocation_pass(&stack_offset)];
    let mut fixed_instructions = instruction_fixup_pass(&mut asm_instructions);
    final_instructions.append(&mut fixed_instructions);
    final_instructions
}

/// Executes the instruction conversion pass of the code generation pipeline.
///
/// Replaces TACKY instructions with equivalent assembly instructions. One TACKY instruction may result in multiple assembly instructions.
///
/// # Arguments
///
/// * `tacky_instructions` - A reference to the TACKY `TackyInstruction`s to convert.
///
/// # Returns
///
/// A `Result` containing a vector of `AssemblyInstruction`s on success,
/// or a `CodegenError` on failure.
fn instruction_conversion_pass(
    tacky_instructions: &Vec<TackyInstruction>,
) -> Vec<AssemblyInstruction> {
    let mut asm_instructions = vec![];
    for tacky_instruction in tacky_instructions.iter() {
        match tacky_instruction {
            TackyInstruction::Return { value } => {
                let mov_instruction = AssemblyInstruction::Mov {
                    source: convert_operand(&value),
                    destination: AssemblyOperand::Register(AssemblyRegister::AX),
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
                    op: convert_unary_operator(&operator),
                    operand: convert_operand(&destination),
                };
                asm_instructions.push(mov_instruction);
                asm_instructions.push(unary_instruction);
            }
            TackyInstruction::Binary {
                operator,
                source1,
                source2,
                destination,
            } => {
                match convert_binary_operator(operator) {
                    Some(asm_binary_operator) => {
                        let mov_instruction = AssemblyInstruction::Mov {
                            source: convert_operand(&source1),
                            destination: convert_operand(&destination),
                        };
                        let binary_instruction = AssemblyInstruction::Binary {
                            op: asm_binary_operator,
                            source: convert_operand(&source2),
                            destination: convert_operand(&destination),
                        };
                        asm_instructions.push(mov_instruction);
                        asm_instructions.push(binary_instruction);
                    }
                    None => {
                        let mov_to_reg_instruction = AssemblyInstruction::Mov {
                            source: convert_operand(&source1),
                            destination: AssemblyOperand::Register(AssemblyRegister::AX),
                        };
                        let cdq_instruction = AssemblyInstruction::Cdq;
                        let idiv_instruction = AssemblyInstruction::Idiv {
                            operand: convert_operand(&source2),
                        };
                        let mov_from_reg_instruction = match operator {
                            // Quotient is stored in %eax
                            TackyBinaryOperator::Divide => AssemblyInstruction::Mov {
                                source: AssemblyOperand::Register(AssemblyRegister::AX),
                                destination: convert_operand(&destination),
                            },
                            // Remainder is stored in %edx
                            TackyBinaryOperator::Remainder => AssemblyInstruction::Mov {
                                source: AssemblyOperand::Register(AssemblyRegister::DX),
                                destination: convert_operand(&destination),
                            },
                            _ => unreachable!(
                                "The other binary operators should have been handled by the previous match arm"
                            ),
                        };
                        asm_instructions.push(mov_to_reg_instruction);
                        asm_instructions.push(cdq_instruction);
                        asm_instructions.push(idiv_instruction);
                        asm_instructions.push(mov_from_reg_instruction);
                    }
                };
            }
        }
    }
    asm_instructions
}

/// Converts a `TackyUnaryOperator` to its corresponding `AssemblyUnaryOperator`.
///
/// # Arguments
///
/// * `tacky_operator`: A reference to a `TackyUnaryOperator` enum value.
///
/// # Returns
///
/// An `AssemblyUnaryOperator` enum value that represents the equivalent operation.
fn convert_unary_operator(tacky_unary_operator: &TackyUnaryOperator) -> AssemblyUnaryOperator {
    match tacky_unary_operator {
        TackyUnaryOperator::Negate => AssemblyUnaryOperator::Neg,
        TackyUnaryOperator::Complement => AssemblyUnaryOperator::Not,
    }
}

/// Converts a TackyBinaryOperator to an AssemblyBinaryOperator.
///
/// # Arguments
///
/// * `tacky_binary_operator`: A reference to the TackyBinaryOperator to convert.
///
/// # Returns
///
/// An `Option<AssemblyBinaryOperator>` representing the converted operator, or `None` if the conversion is not supported.
fn convert_binary_operator(
    tacky_binary_operator: &TackyBinaryOperator,
) -> Option<AssemblyBinaryOperator> {
    match tacky_binary_operator {
        TackyBinaryOperator::Add => Some(AssemblyBinaryOperator::Add),
        TackyBinaryOperator::Subtract => Some(AssemblyBinaryOperator::Sub),
        TackyBinaryOperator::Multiply => Some(AssemblyBinaryOperator::Mult),
        TackyBinaryOperator::Divide => None,
        TackyBinaryOperator::Remainder => None,
    }
}

/// Converts a `TackyValue` to its corresponding `AssemblyUnaryOperand`.
///
/// # Arguments
///
/// * `tacky_operand`: The TackyValue to convert.
///
/// # Returns
///
/// An AssemblyUnaryOperand representing the converted value.
fn convert_operand(tacky_operand: &TackyValue) -> AssemblyOperand {
    match tacky_operand {
        TackyValue::Constant(value) => AssemblyOperand::Imm(*value),
        TackyValue::Variable(name) => AssemblyOperand::Pseudo(name.clone()),
    }
}

/// Replaces pseudo registers with physical registers in the assembly instructions.
///
/// The following instructions should replace their pseudo registers with physical registers:
/// * `AssemblyInstruction::Mov`
/// * `AssemblyInstruction::Unary`
/// * `AssemblyInstruction::Binary`
/// * `AssemblyInstruction::Idiv`
///
/// # Arguments
///
/// * `asm_ast` - The assembly AST to be modified.
///
/// # Returns
///
/// The final stack offset after replacing pseudo registers.
fn pseudoregister_replacement_pass(instructions: &mut Vec<AssemblyInstruction>) -> i32 {
    let mut identifier_offsets: HashMap<String, i32> = HashMap::new();
    let mut offset_counter = 0;
    for instruction in instructions.iter_mut() {
        match instruction {
            AssemblyInstruction::Mov {
                source,
                destination,
            } => {
                convert_pseudo_register(source, &mut identifier_offsets, &mut offset_counter);
                convert_pseudo_register(destination, &mut identifier_offsets, &mut offset_counter);
            }
            AssemblyInstruction::Unary { op: _, operand } => {
                convert_pseudo_register(operand, &mut identifier_offsets, &mut offset_counter);
            }
            AssemblyInstruction::Binary {
                op: _,
                source,
                destination,
            } => {
                convert_pseudo_register(source, &mut identifier_offsets, &mut offset_counter);
                convert_pseudo_register(destination, &mut identifier_offsets, &mut offset_counter);
            }
            AssemblyInstruction::Idiv { operand } => {
                convert_pseudo_register(operand, &mut identifier_offsets, &mut offset_counter);
            }
            AssemblyInstruction::Cdq => {}
            AssemblyInstruction::AllocateStack { stack_offset: _ } => {}
            AssemblyInstruction::Ret => {}
        }
    }
    offset_counter
}

/// Converts a pseudo-register operand to a stack operand.
///
/// This function takes an `Operand` and attempts to convert it from a `Pseudo` variant (representing an identifier) to a `Stack` variant (representing a memory offset).
///
/// # Arguments
///
/// * `operand`: A mutable reference to the `Operand` to be converted. If it's a `Pseudo` variant, it will be modified in place to become a `Stack` variant.
/// * `identifier_offsets`: A mutable reference to a `HashMap` that maps identifier strings to their allocated stack offsets (`i32`).
/// * `offset_counter`: A mutable reference to an `i32` that acts as a counter for allocating new stack offsets. It is decremented for each new identifier.
///
/// # Returns
///
/// This function does not return a value, but it modifies the `operand` argument in place.
fn convert_pseudo_register(
    operand: &mut AssemblyOperand,
    identifier_offsets: &mut HashMap<String, i32>,
    offset_counter: &mut i32,
) -> () {
    match operand {
        AssemblyOperand::Pseudo(identifier) => {
            if let Some(offset) = identifier_offsets.get(identifier) {
                *operand = AssemblyOperand::Stack(*offset);
                return;
            }
            *offset_counter -= constants::STACK_ADDRESS_OFFSET;
            identifier_offsets.insert(identifier.clone(), *offset_counter);
            *operand = AssemblyOperand::Stack(*offset_counter);
        }
        _ => {}
    }
}

/// Fixes up instructions by resolving memory-to-memory operations.
///
/// # Arguments
///
/// * `instructions`: The `AssemblyInstruction`s to process.
/// * `stack_offset`: The total stack space in bytes to allocate for the function.
///
/// # Returns
///
/// A new `AssemblyAst` with the instructions fixed up.
fn instruction_fixup_pass(instructions: &Vec<AssemblyInstruction>) -> Vec<AssemblyInstruction> {
    let mut fixed_instructions = vec![];
    for instruction in instructions.iter() {
        fixed_instructions.append(&mut fixup_memory_to_memory_operation(instruction));
    }
    fixed_instructions
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
fn stack_allocation_pass(stack_offset: &i32) -> AssemblyInstruction {
    AssemblyInstruction::AllocateStack {
        stack_offset: *stack_offset,
    }
}

/// Fixes up memory-to-memory `Mov` operations by using a scratch register.
///
/// When a `Mov` instruction involves two memory operands, it's not directly supported by many architectures. This function replaces such an operation with two instructions: the first moves the source memory to a scratch register (R10),and the second moves the scratch register to the destination memory.
///
/// # Arguments
///
/// * `asm_instruction`: The `AssemblyInstruction` to potentially fix up.
///
/// # Returns
///
/// A `Vec<AssemblyInstruction>` containing the original instruction if no fixup was needed, or the sequence of two instructions if a memory-to-memory `Mov` was encountered.
fn fixup_memory_to_memory_operation(
    asm_instruction: &AssemblyInstruction,
) -> Vec<AssemblyInstruction> {
    let scratch_register_operand = AssemblyOperand::Register(AssemblyRegister::R10);
    match asm_instruction {
        AssemblyInstruction::Mov {
            source,
            destination,
        } => match (source, destination) {
            (AssemblyOperand::Stack(_), AssemblyOperand::Stack(_)) => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_conversion_pass_success() {
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
        let result = instruction_conversion_pass(&tacky_instructions);
        assert_eq!(
            result,
            vec![
                AssemblyInstruction::Mov {
                    source: AssemblyOperand::Imm(1),
                    destination: AssemblyOperand::Pseudo(identifier.clone()),
                },
                AssemblyInstruction::Unary {
                    op: AssemblyUnaryOperator::Neg,
                    operand: AssemblyOperand::Pseudo(identifier.clone()),
                },
                AssemblyInstruction::Mov {
                    source: AssemblyOperand::Pseudo(identifier.clone()),
                    destination: AssemblyOperand::Register(AssemblyRegister::AX),
                },
                AssemblyInstruction::Ret,
            ]
        );
    }

    #[test]
    fn test_pseudoregister_replacement_pass_success() {
        let pseudo_register_name = "tmp.0".to_string();
        let mut instructions = vec![
            AssemblyInstruction::Mov {
                source: AssemblyOperand::Imm(1),
                destination: AssemblyOperand::Pseudo(pseudo_register_name),
            },
            AssemblyInstruction::Ret,
        ];
        let offset = pseudoregister_replacement_pass(&mut instructions);
        assert_eq!(offset, -4);
        assert_eq!(
            instructions,
            vec![
                AssemblyInstruction::Mov {
                    source: AssemblyOperand::Imm(1),
                    destination: AssemblyOperand::Stack(-4),
                },
                AssemblyInstruction::Ret,
            ]
        );
    }
}
