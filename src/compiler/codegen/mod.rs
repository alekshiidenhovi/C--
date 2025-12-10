pub mod asm_ast;
pub mod errors;

use crate::compiler::parser::ast::{Ast, Expression, FunctionDefinition, Statement};
use crate::compiler::tokens::{Token, TokenType};
use asm_ast::{AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand};
use errors::CodegenError;

/// Generates assembly code from an Abstract Syntax Tree (AST).
///
/// This struct takes a C-like AST and converts it into a lower-level
/// assembly-like AST, suitable for further processing or interpretation.
pub struct AssemblyGenerator {
    /// The C-like AST to be converted.
    c_ast: Ast,
}

impl AssemblyGenerator {
    /// Creates a new `AssemblyGenerator` instance.
    ///
    /// # Arguments
    ///
    /// * `c_ast` - The C-like Abstract Syntax Tree to process.
    ///
    /// # Returns
    ///
    /// A new `AssemblyGenerator` initialized with the provided AST.
    pub fn new(c_ast: Ast) -> Self {
        Self { c_ast }
    }

    /// Converts the entire C-like AST into an assembly AST.
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
    /// # use cmm::compiler::parser::ast::{Ast, FunctionDefinition, Statement, Expression};
    /// # use cmm::compiler::codegen::AssemblyGenerator;
    /// # use cmm::compiler::codegen::asm_ast::{AssemblyAst, FunctionDefinition as AsmFunctionDefinition, Instruction, Operand};
    /// # use cmm::compiler::codegen::errors::CodegenError;
    ///
    /// let c_ast = Ast::Program(FunctionDefinition::Function(Token::Identifier("main".to_string()), Statement::Return(Expression::IntegerConstant(Token::Constant(1)))));
    /// let asm_generator = AssemblyGenerator::new(c_ast);
    /// let assembly_ast = asm_generator.convert_ast()?;
    /// assert_eq!(assembly_ast, AssemblyAst::Program(AsmFunctionDefinition::Function { identifier: "main".to_string(), instructions: vec![Instruction::Mov { source: Operand::Imm(1), destination: Operand::Register }, Instruction::Ret] }));
    ///
    /// # Ok::<(), CodegenError>(())
    /// ```
    pub fn convert_ast(&self) -> Result<AssemblyAst, CodegenError> {
        let function = match &self.c_ast {
            Ast::Program(c_function) => self.convert_function(&c_function)?,
        };
        Ok(AssemblyAst::Program(function))
    }

    /// Converts a C-like function definition into an assembly function definition.
    ///
    /// # Arguments
    ///
    /// * `c_function` - A reference to the C-like `FunctionDefinition` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `AsmFunctionDefinition` on success,
    /// or a `CodegenError` on failure.
    fn convert_function(
        &self,
        c_function: &FunctionDefinition,
    ) -> Result<AsmFunctionDefinition, CodegenError> {
        match c_function {
            FunctionDefinition::Function(identifier, statement) => match identifier {
                Token::Identifier(name) => {
                    let instructions = self.convert_statement(statement)?;
                    Ok(AsmFunctionDefinition::Function {
                        identifier: name.clone(),
                        instructions,
                    })
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

    /// Converts a C-like statement into a sequence of assembly instructions.
    ///
    /// # Arguments
    ///
    /// * `c_statement` - A reference to the C-like `Statement` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Instruction`s on success,
    /// or a `CodegenError` on failure.
    fn convert_statement(&self, c_statement: &Statement) -> Result<Vec<Instruction>, CodegenError> {
        match c_statement {
            Statement::Return(expression) => match self.convert_expression(expression) {
                Ok(operand) => Ok(vec![
                    Instruction::Mov {
                        source: operand,
                        destination: Operand::Register,
                    },
                    Instruction::Ret,
                ]),
                Err(error) => Err(error),
            },
        }
    }

    /// Converts a C-like expression into an assembly operand.
    ///
    /// # Arguments
    ///
    /// * `c_expression` - A reference to the C-like `Expression` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `Operand` on success,
    /// or a `CodegenError` on failure.
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
