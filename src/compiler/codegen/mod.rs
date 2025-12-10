pub mod asm_ast;
pub mod errors;

use crate::compiler::parser::ast::{Ast, Expression, FunctionDefinition, Statement};
use crate::compiler::tokens::{Token, TokenType};
use asm_ast::{AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand};
use errors::CodegenError;

pub struct AssemblyGenerator {
    c_ast: Ast,
}

impl AssemblyGenerator {
    pub fn new(c_ast: Ast) -> Self {
        Self { c_ast }
    }

    pub fn convert_ast(&self) -> Result<AssemblyAst, CodegenError> {
        let function = match &self.c_ast {
            Ast::Program(c_function) => self.convert_function(&c_function)?,
        };
        Ok(AssemblyAst::Program(function))
    }

    fn convert_function(
        &self,
        c_function: &FunctionDefinition,
    ) -> Result<AsmFunctionDefinition, CodegenError> {
        match c_function {
            FunctionDefinition::Function(identifier, statement) => match identifier {
                Token::Identifier(name) => {
                    let instructions = self.convert_statement(statement)?;
                    Ok(AsmFunctionDefinition::Function(name.clone(), instructions))
                }
                _ => {
                    return Err(CodegenError::UnexpectedToken {
                        expected: TokenType::Identifier,
                        actual: identifier.kind(),
                    });
                }
            },
        }
    }

    fn convert_statement(&self, c_statement: &Statement) -> Result<Vec<Instruction>, CodegenError> {
        match c_statement {
            Statement::Return(expression) => match self.convert_expression(expression) {
                Ok(operand) => Ok(vec![
                    Instruction::Mov(operand, Operand::Register),
                    Instruction::Ret,
                ]),
                Err(error) => Err(error),
            },
        }
    }

    fn convert_expression(&self, c_expression: &Expression) -> Result<Operand, CodegenError> {
        match c_expression {
            Expression::IntegerConstant(token) => match token {
                Token::Constant(value) => Ok(Operand::Imm(*value)),
                _ => Err(CodegenError::UnexpectedToken {
                    expected: TokenType::Constant,
                    actual: token.kind(),
                }),
            },
        }
    }
}
