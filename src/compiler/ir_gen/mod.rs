pub mod errors;
pub mod tacky_ir;

use crate::compiler::parser::ast::{Ast, Expression, FunctionDefinition, Statement, UnaryOperator};
use crate::compiler::tokens::{Token, TokenType};
use errors::IRConversionError;
use tacky_ir::{TackyFunction, TackyIR, TackyInstruction, TackyUnaryOperator, TackyValue};

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
    pub fn convert_ast(&mut self, c_ast: Ast) -> Result<TackyIR, IRConversionError> {
        let function = match c_ast {
            Ast::Program(c_function) => self.convert_function(&c_function)?,
        };
        Ok(TackyIR::Program(function))
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
        c_function: &FunctionDefinition,
    ) -> Result<TackyFunction, IRConversionError> {
        match c_function {
            FunctionDefinition::Function(token, statement) => match token {
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
        c_statement: &Statement,
    ) -> Result<Vec<TackyInstruction>, IRConversionError> {
        match c_statement {
            Statement::Return(expression) => {
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
        c_expression: &Expression,
        tacky_instructions: &mut Vec<TackyInstruction>,
    ) -> Result<TackyValue, IRConversionError> {
        match c_expression {
            Expression::IntegerConstant(token) => match token {
                Token::Constant(value) => Ok(TackyValue::Constant(*value)),
                _ => Err(IRConversionError::UnexpectedToken {
                    expected: TokenType::Constant,
                    actual: token.kind(),
                }),
            },
            Expression::Unary(c_operator, c_inner_expression) => {
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
    fn convert_unary_operator(&self, c_operator: &UnaryOperator) -> TackyUnaryOperator {
        match c_operator {
            UnaryOperator::Complement => TackyUnaryOperator::Complement,
            UnaryOperator::Negate => TackyUnaryOperator::Negate,
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
        let c_expression = Expression::IntegerConstant(Token::Constant(1));
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&c_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Constant(1)));
        assert_eq!(tacky_instructions, vec![]);
    }

    #[test]
    fn test_emit_tacky_single_negate_expression() {
        let mut tacky_emitter = TackyEmitter::new();
        let c_expression = Expression::Unary(
            UnaryOperator::Negate,
            Box::new(Expression::IntegerConstant(Token::Constant(1))),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&c_expression, &mut tacky_instructions);

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
        let c_expression = Expression::Unary(
            UnaryOperator::Complement,
            Box::new(Expression::IntegerConstant(Token::Constant(1))),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&c_expression, &mut tacky_instructions);

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
        let c_expression = Expression::Unary(
            UnaryOperator::Negate,
            Box::new(Expression::Unary(
                UnaryOperator::Complement,
                Box::new(Expression::IntegerConstant(Token::Constant(1))),
            )),
        );
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&c_expression, &mut tacky_instructions);

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
        let c_ast = Ast::Program(FunctionDefinition::Function(
            Token::Identifier(identifier.clone()),
            Statement::Return(Expression::Unary(
                UnaryOperator::Negate,
                Box::new(Expression::Unary(
                    UnaryOperator::Complement,
                    Box::new(Expression::IntegerConstant(Token::Constant(1))),
                )),
            )),
        ));
        let tacky_ast = tacky_emitter.convert_ast(c_ast);
        assert_eq!(
            tacky_ast,
            Ok(TackyIR::Program(TackyFunction::Function {
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
