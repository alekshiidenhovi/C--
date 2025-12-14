pub mod errors;
pub mod tacky_ast;

use crate::compiler::parser::cmm_ast::{
    CmmAst, CmmExpression, CmmFunction, CmmStatement, CmmUnaryOperator,
};
use crate::compiler::tokens::{Token, TokenType};
use errors::IRConversionError;
use tacky_ast::{TackyAst, TackyFunction, TackyInstruction, TackyUnaryOperator, TackyValue};

/// Represents an emitter for Tacky, a language or system.
///
/// It holds the C-- AST and a temporary variable counter.
pub struct TackyEmitter {
    /// A counter for temporary variables.
    temp_counter: usize,
}

impl TackyEmitter {
    /// Creates a new `TackyEmitter` instance.
    ///
    /// # Returns
    ///
    /// A new `TackyEmitter` instance initialized with the provided C-- AST.
    pub fn new() -> Self {
        Self { temp_counter: 0 }
    }

    /// Converts the C-- AST into an intermediate TACKY representation.
    ///
    /// # Arguments
    ///
    /// * `c_ast`: A reference to the C-- `Ast` to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyFunction` on success,
    /// or a `CodegenError` on failure.
    pub fn convert_ast(&mut self, cmm_ast: CmmAst) -> Result<TackyAst, IRConversionError> {
        let function = match cmm_ast {
            CmmAst::Program(c_function) => self.convert_function(&c_function)?,
        };
        Ok(TackyAst::Program(function))
    }

    /// Converts a C-- function definition into a TACKY function definition.
    ///
    /// # Arguments
    ///
    /// * `c_function` - A reference to the C-- `FunctionDefinition` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyFunction` on success,
    /// or a `CodegenError` on failure.
    fn convert_function(
        &mut self,
        cmm_function: &CmmFunction,
    ) -> Result<TackyFunction, IRConversionError> {
        match cmm_function {
            CmmFunction::Function(token, statement) => match token {
                Token::Identifier(name) => {
                    let statements = self.convert_statement(statement)?;
                    Ok(TackyFunction::Function {
                        identifier: name.clone(),
                        instructions: statements,
                    })
                }
                _ => {
                    return Err(IRConversionError::UnexpectedToken {
                        expected: TokenType::Identifier,
                        actual: token.kind(),
                    });
                }
            },
        }
    }

    /// Converts a C-- statement into a sequence of TACKY instructions.
    ///
    /// # Arguments
    ///
    /// * `c_statement` - A reference to the C-- `Statement` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `TackyInstruction`s on success,
    /// or a `CodegenError` on failure.
    fn convert_statement(
        &mut self,
        cmm_statement: &CmmStatement,
    ) -> Result<Vec<TackyInstruction>, IRConversionError> {
        match cmm_statement {
            CmmStatement::Return(expression) => {
                let mut tacky_instructions = Vec::new();
                let tacky_value = self.emit_tacky(expression, &mut tacky_instructions)?;
                tacky_instructions.push(TackyInstruction::Return(tacky_value));
                Ok(tacky_instructions)
            }
        }
    }

    /// Converts a C-- expression into a TACKY value.
    ///
    /// Recursively calls itself to convert nested expressions.
    ///
    /// # Arguments
    ///
    /// * `c_expression` - A reference to the C-- `Expression` to convert.
    /// * `tacky_instructions` - A mutable reference to the vector of `TackyInstruction`s to append the generated instructions to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyValue` on success,
    /// or a `CodegenError` on failure.
    fn emit_tacky(
        &mut self,
        cmm_expression: &CmmExpression,
        tacky_instructions: &mut Vec<TackyInstruction>,
    ) -> Result<TackyValue, IRConversionError> {
        match cmm_expression {
            CmmExpression::IntegerConstant(token) => match token {
                Token::Constant(value) => Ok(TackyValue::Constant(*value)),
                _ => Err(IRConversionError::UnexpectedToken {
                    expected: TokenType::Constant,
                    actual: token.kind(),
                }),
            },
            CmmExpression::Unary(c_operator, c_inner_expression) => {
                let source = self.emit_tacky(c_inner_expression, tacky_instructions)?;
                let destination_name = self.make_temporary();
                let destination = TackyValue::Variable(destination_name);
                let operator = self.convert_unary_operator(c_operator);
                tacky_instructions.push(TackyInstruction::Unary {
                    operator,
                    source,
                    destination: destination.clone(),
                });
                Ok(destination)
            }
        }
    }

    /// Converts a C-- unary operator into a TACKY unary operator.
    ///
    /// # Arguments
    ///
    /// * `c_operator` - A reference to the C-- `UnaryOperator` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyUnaryOperator` on success,
    /// or a `CodegenError` on failure.
    fn convert_unary_operator(&self, cmm_operator: &CmmUnaryOperator) -> TackyUnaryOperator {
        match cmm_operator {
            CmmUnaryOperator::Complement => TackyUnaryOperator::Complement,
            CmmUnaryOperator::Negate => TackyUnaryOperator::Negate,
        }
    }

    /// Generates a name for a temporary TACKY variable.
    ///
    /// Side effect: increments the temporary variable counter.
    ///
    /// # Returns
    ///
    /// A `String` containing the generated temporary variable name.
    fn make_temporary(&mut self) -> String {
        let temp_name = format!("tmp.{}", self.temp_counter);
        self.temp_counter += 1;
        temp_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_temporary() {
        let mut tacky_emitter = TackyEmitter::new();
        let temp_name = tacky_emitter.make_temporary();
        assert_eq!(temp_name, "tmp.0");
        let temp_name = tacky_emitter.make_temporary();
        assert_eq!(temp_name, "tmp.1");
    }

    #[test]
    fn test_emit_tacky_constant_only() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::IntegerConstant(Token::Constant(1));
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Constant(1)));
        assert_eq!(tacky_instructions, vec![]);
    }

    #[test]
    fn test_emit_tacky_single_negate_expression() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Unary(
            CmmUnaryOperator::Negate,
            Box::new(CmmExpression::IntegerConstant(Token::Constant(1))),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.0"))));
        assert_eq!(
            tacky_instructions,
            vec![TackyInstruction::Unary {
                operator: TackyUnaryOperator::Negate,
                source: TackyValue::Constant(1),
                destination: TackyValue::Variable(String::from("tmp.0")),
            }]
        );
    }

    #[test]
    fn test_emit_tacky_single_complement_expression() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Unary(
            CmmUnaryOperator::Complement,
            Box::new(CmmExpression::IntegerConstant(Token::Constant(1))),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.0"))));
        assert_eq!(
            tacky_instructions,
            vec![TackyInstruction::Unary {
                operator: TackyUnaryOperator::Complement,
                source: TackyValue::Constant(1),
                destination: TackyValue::Variable(String::from("tmp.0")),
            }]
        );
    }

    #[test]
    fn test_emit_tacky_double_unary_expression() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Unary(
            CmmUnaryOperator::Negate,
            Box::new(CmmExpression::Unary(
                CmmUnaryOperator::Complement,
                Box::new(CmmExpression::IntegerConstant(Token::Constant(1))),
            )),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.1"))));
        assert_eq!(
            tacky_instructions,
            vec![
                TackyInstruction::Unary {
                    operator: TackyUnaryOperator::Complement,
                    source: TackyValue::Constant(1),
                    destination: TackyValue::Variable(String::from("tmp.0")),
                },
                TackyInstruction::Unary {
                    operator: TackyUnaryOperator::Negate,
                    source: TackyValue::Variable(String::from("tmp.0")),
                    destination: TackyValue::Variable(String::from("tmp.1")),
                }
            ]
        );
    }

    #[test]
    fn test_emit_ast() {
        let identifier = "main".to_string();
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_ast = CmmAst::Program(CmmFunction::Function(
            Token::Identifier(identifier.clone()),
            CmmStatement::Return(CmmExpression::Unary(
                CmmUnaryOperator::Negate,
                Box::new(CmmExpression::Unary(
                    CmmUnaryOperator::Complement,
                    Box::new(CmmExpression::IntegerConstant(Token::Constant(1))),
                )),
            )),
        ));
        let tacky_ast = tacky_emitter.convert_ast(cmm_ast);
        assert_eq!(
            tacky_ast,
            Ok(TackyAst::Program(TackyFunction::Function {
                identifier,
                instructions: vec![
                    TackyInstruction::Unary {
                        operator: TackyUnaryOperator::Complement,
                        source: TackyValue::Constant(1),
                        destination: TackyValue::Variable(String::from("tmp.0")),
                    },
                    TackyInstruction::Unary {
                        operator: TackyUnaryOperator::Negate,
                        source: TackyValue::Variable(String::from("tmp.0")),
                        destination: TackyValue::Variable(String::from("tmp.1")),
                    },
                    TackyInstruction::Return(TackyValue::Variable(String::from("tmp.1"))),
                ]
            }))
        );
    }
}
