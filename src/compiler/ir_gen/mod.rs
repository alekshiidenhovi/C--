pub mod errors;
pub mod tacky_ast;

use crate::compiler::parser::cmm_ast::{
    CmmAst, CmmBinaryOperator, CmmExpression, CmmFunction, CmmStatement, CmmUnaryOperator,
};
use errors::IRConversionError;
use tacky_ast::{
    TackyAst, TackyBinaryOperator, TackyFunction, TackyInstruction, TackyUnaryOperator, TackyValue,
};

/// Represents an emitter for Tacky, a language or system.
///
/// It holds the C-- AST and a temporary variable counter.
pub struct TackyEmitter {
    /// A counter for temporary variables.
    temp_counter: usize,
    /// A counter for labels.
    label_counter: usize,
}

impl TackyEmitter {
    /// Creates a new `TackyEmitter` instance.
    ///
    /// # Returns
    ///
    /// A new `TackyEmitter` instance initialized with the provided C-- AST.
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
        }
    }

    /// Converts the C-- AST into an intermediate TACKY representation.
    ///
    /// # Arguments
    ///
    /// * `cmm_ast`: A reference to the C-- `CmmAst` to be converted.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyFunction` on success,
    /// or a `CodegenError` on failure.
    pub fn convert_ast(&mut self, cmm_ast: CmmAst) -> Result<TackyAst, IRConversionError> {
        let function = match cmm_ast {
            CmmAst::Program { function } => self.convert_function(&function)?,
        };
        Ok(TackyAst::Program { function })
    }

    /// Converts a C-- function definition into a TACKY function definition.
    ///
    /// # Arguments
    ///
    /// * `cmm_function` - A reference to the C-- `CmmFunction` to convert.
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
            CmmFunction::Function { identifier, body } => {
                let statements = self.convert_statement(body)?;
                Ok(TackyFunction::Function {
                    identifier: identifier.clone(),
                    instructions: statements,
                })
            }
        }
    }

    /// Converts a C-- statement into a sequence of TACKY instructions.
    ///
    /// # Arguments
    ///
    /// * `cmm_statement` - A reference to the C-- `CmmStatement` to convert.
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
            CmmStatement::Return { expression } => {
                let mut tacky_instructions = Vec::new();
                let tacky_value = self.emit_tacky(expression, &mut tacky_instructions)?;
                tacky_instructions.push(TackyInstruction::Return { value: tacky_value });
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
    /// * `cmm_expression` - A reference to the C-- `Expression` to convert.
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
            CmmExpression::IntegerConstant { value } => Ok(TackyValue::Constant(*value)),
            CmmExpression::Unary {
                operator,
                expression,
            } => {
                let source = self.emit_tacky(expression, tacky_instructions)?;
                let destination_name = self.make_temporary();
                let destination = TackyValue::Variable(destination_name);
                let operator = self.convert_unary_operator(operator);
                tacky_instructions.push(TackyInstruction::Unary {
                    operator,
                    source,
                    destination: destination.clone(),
                });
                Ok(destination)
            }
            CmmExpression::Binary {
                operator,
                left,
                right,
            } => match operator {
                CmmBinaryOperator::And => {
                    let label_false_name = self.make_label("and_false");
                    let label_end_name = self.make_label("and_end");

                    // First condition
                    let source1 = self.emit_tacky(left, tacky_instructions)?;
                    let jump_false1 = TackyInstruction::JumpIfZero {
                        condition: source1,
                        target: label_false_name.clone(),
                    };
                    tacky_instructions.push(jump_false1);

                    // Second condition, unless first condition is zero
                    let source2 = self.emit_tacky(right, tacky_instructions)?;
                    let jump_false2 = TackyInstruction::JumpIfZero {
                        condition: source2,
                        target: label_false_name.clone(),
                    };
                    tacky_instructions.push(jump_false2);

                    let destination_name = self.make_temporary();

                    // Return value if both conditions are non-zero
                    let copy_true = TackyInstruction::Copy {
                        source: TackyValue::Constant(1),
                        destination: TackyValue::Variable(destination_name.clone()),
                    };
                    let jump_end = TackyInstruction::Jump {
                        target: label_end_name.clone(),
                    };

                    // Return value if any condition is zero
                    let label_false = TackyInstruction::Label(label_false_name);
                    let copy_false = TackyInstruction::Copy {
                        source: TackyValue::Constant(0),
                        destination: TackyValue::Variable(destination_name.clone()),
                    };
                    let label_end = TackyInstruction::Label(label_end_name);
                    tacky_instructions.push(copy_true);
                    tacky_instructions.push(jump_end);
                    tacky_instructions.push(label_false);
                    tacky_instructions.push(copy_false);
                    tacky_instructions.push(label_end);

                    Ok(TackyValue::Variable(destination_name))
                }
                CmmBinaryOperator::Or => {
                    let label_true_name = self.make_label("or_true");
                    let label_end_name = self.make_label("or_end");

                    // First condition
                    let source1 = self.emit_tacky(left, tacky_instructions)?;
                    let jump_true1 = TackyInstruction::JumpIfNotZero {
                        condition: source1,
                        target: label_true_name.clone(),
                    };
                    tacky_instructions.push(jump_true1);

                    // Second condition, unless first condition is not zero
                    let source2 = self.emit_tacky(right, tacky_instructions)?;
                    let jump_true2 = TackyInstruction::JumpIfNotZero {
                        condition: source2,
                        target: label_true_name.clone(),
                    };
                    tacky_instructions.push(jump_true2);

                    let destination_name = self.make_temporary();

                    // Return value if both conditions are zero
                    let copy_false = TackyInstruction::Copy {
                        source: TackyValue::Constant(0),
                        destination: TackyValue::Variable(destination_name.clone()),
                    };
                    let jump_end = TackyInstruction::Jump {
                        target: label_end_name.clone(),
                    };

                    // Return value if any condition is non-zero
                    let label_true = TackyInstruction::Label(label_true_name);
                    let copy_true = TackyInstruction::Copy {
                        source: TackyValue::Constant(1),
                        destination: TackyValue::Variable(destination_name.clone()),
                    };
                    let label_end = TackyInstruction::Label(label_end_name);
                    tacky_instructions.push(copy_false);
                    tacky_instructions.push(jump_end);
                    tacky_instructions.push(label_true);
                    tacky_instructions.push(copy_true);
                    tacky_instructions.push(label_end);

                    Ok(TackyValue::Variable(destination_name))
                }
                CmmBinaryOperator::Equal
                | CmmBinaryOperator::NotEqual
                | CmmBinaryOperator::GreaterThan
                | CmmBinaryOperator::LessThan
                | CmmBinaryOperator::GreaterThanEqual
                | CmmBinaryOperator::LessThanEqual
                | CmmBinaryOperator::Add
                | CmmBinaryOperator::Subtract
                | CmmBinaryOperator::Multiply
                | CmmBinaryOperator::Divide
                | CmmBinaryOperator::Remainder => {
                    let source1 = self.emit_tacky(left, tacky_instructions)?;
                    let source2 = self.emit_tacky(right, tacky_instructions)?;
                    let destination_name = self.make_temporary();
                    let destination = TackyValue::Variable(destination_name);
                    let operator = self.convert_binary_operator(operator)?;
                    tacky_instructions.push(TackyInstruction::Binary {
                        operator,
                        source1,
                        source2,
                        destination: destination.clone(),
                    });
                    Ok(destination)
                }
            },
        }
    }

    /// Converts a C-- unary operator into a TACKY unary operator.
    ///
    /// # Arguments
    ///
    /// * `cmm_operator` - A reference to the C-- `CmmUnaryOperator` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyUnaryOperator` on success,
    /// or a `CodegenError` on failure.
    fn convert_unary_operator(&self, cmm_operator: &CmmUnaryOperator) -> TackyUnaryOperator {
        match cmm_operator {
            CmmUnaryOperator::Complement => TackyUnaryOperator::Complement,
            CmmUnaryOperator::Negate => TackyUnaryOperator::Negate,
            CmmUnaryOperator::Not => TackyUnaryOperator::Not,
        }
    }

    /// Converts a C-- binary operator into a TACKY binary operator.
    ///
    /// # Arguments
    ///
    /// * `cmm_operator` - A reference to the C-- `CmmBinaryOperator` to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated `TackyBinaryOperator` on success,
    /// or a `CodegenError` on failure.
    fn convert_binary_operator(
        &self,
        cmm_operator: &CmmBinaryOperator,
    ) -> Result<TackyBinaryOperator, IRConversionError> {
        match cmm_operator {
            CmmBinaryOperator::Add => Ok(TackyBinaryOperator::Add),
            CmmBinaryOperator::Subtract => Ok(TackyBinaryOperator::Subtract),
            CmmBinaryOperator::Multiply => Ok(TackyBinaryOperator::Multiply),
            CmmBinaryOperator::Divide => Ok(TackyBinaryOperator::Divide),
            CmmBinaryOperator::Remainder => Ok(TackyBinaryOperator::Remainder),
            CmmBinaryOperator::Equal => Ok(TackyBinaryOperator::Equal),
            CmmBinaryOperator::NotEqual => Ok(TackyBinaryOperator::NotEqual),
            CmmBinaryOperator::GreaterThan => Ok(TackyBinaryOperator::GreaterThan),
            CmmBinaryOperator::LessThan => Ok(TackyBinaryOperator::LessThan),
            CmmBinaryOperator::GreaterThanEqual => Ok(TackyBinaryOperator::GreaterThanEqual),
            CmmBinaryOperator::LessThanEqual => Ok(TackyBinaryOperator::LessThanEqual),
            CmmBinaryOperator::And | CmmBinaryOperator::Or => {
                Err(IRConversionError::UnsupportedBinaryOperatorConversion {
                    operator: cmm_operator.clone(),
                })
            }
        }
    }

    /// Generates a unique name for a temporary TACKY variable.
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

    /// Generates a unique label string by appending a counter to a base name.
    ///
    /// Side effect: increments the label counter.
    ///
    /// # Arguments
    ///
    /// * `label_name`: The base name for the label.
    ///
    /// # Returns
    ///
    /// A unique label string (e.g., "myLabel0", "myLabel1").
    fn make_label(&mut self, label_name: &str) -> String {
        let label = format!("{}{}", label_name, self.label_counter);
        self.label_counter += 1;
        label
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
        let cmm_expression = CmmExpression::IntegerConstant { value: 1 };
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Constant(1)));
        assert_eq!(tacky_instructions, vec![]);
    }

    #[test]
    fn test_emit_tacky_single_negate_expression() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Unary {
            operator: CmmUnaryOperator::Negate,
            expression: Box::new(CmmExpression::IntegerConstant { value: 1 }),
        };
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
        let cmm_expression = CmmExpression::Unary {
            operator: CmmUnaryOperator::Complement,
            expression: Box::new(CmmExpression::IntegerConstant { value: 1 }),
        };
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
        let cmm_expression = CmmExpression::Unary {
            operator: CmmUnaryOperator::Negate,
            expression: Box::new(CmmExpression::Unary {
                operator: CmmUnaryOperator::Complement,
                expression: Box::new(CmmExpression::IntegerConstant { value: 1 }),
            }),
        };
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
    fn test_emit_tacky_binary_operation() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Binary {
            operator: CmmBinaryOperator::Add,
            left: Box::new(CmmExpression::IntegerConstant { value: 1 }),
            right: Box::new(CmmExpression::IntegerConstant { value: 2 }),
        };
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.0"))));
        assert_eq!(
            tacky_instructions,
            vec![TackyInstruction::Binary {
                operator: TackyBinaryOperator::Add,
                source1: TackyValue::Constant(1),
                source2: TackyValue::Constant(2),
                destination: TackyValue::Variable(String::from("tmp.0")),
            }]
        );
    }

    #[test]
    fn test_emit_tacky_and_operation() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Binary {
            operator: CmmBinaryOperator::And,
            left: Box::new(CmmExpression::IntegerConstant { value: 1 }),
            right: Box::new(CmmExpression::IntegerConstant { value: 2 }),
        };
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.0"))));
        assert_eq!(
            tacky_instructions,
            vec![
                TackyInstruction::JumpIfZero {
                    condition: TackyValue::Constant(1),
                    target: String::from("and_false0"),
                },
                TackyInstruction::JumpIfZero {
                    condition: TackyValue::Constant(2),
                    target: String::from("and_false0"),
                },
                TackyInstruction::Copy {
                    source: TackyValue::Constant(1),
                    destination: TackyValue::Variable(String::from("tmp.0")),
                },
                TackyInstruction::Jump {
                    target: String::from("and_end1"),
                },
                TackyInstruction::Label(String::from("and_false0")),
                TackyInstruction::Copy {
                    source: TackyValue::Constant(0),
                    destination: TackyValue::Variable(String::from("tmp.0")),
                },
                TackyInstruction::Label(String::from("and_end1")),
            ]
        );
    }

    #[test]
    fn test_emit_tacky_or_operation() {
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_expression = CmmExpression::Binary {
            operator: CmmBinaryOperator::Or,
            left: Box::new(CmmExpression::Unary {
                operator: CmmUnaryOperator::Negate,
                expression: Box::new(CmmExpression::IntegerConstant { value: 1 }),
            }),
            right: Box::new(CmmExpression::IntegerConstant { value: 2 }),
        };
        let mut tacky_instructions = vec![];
        let tacky_value = tacky_emitter.emit_tacky(&cmm_expression, &mut tacky_instructions);

        assert_eq!(tacky_value, Ok(TackyValue::Variable(String::from("tmp.1"))));
        assert_eq!(
            tacky_instructions,
            vec![
                TackyInstruction::Unary {
                    operator: TackyUnaryOperator::Negate,
                    source: TackyValue::Constant(1),
                    destination: TackyValue::Variable(String::from("tmp.0")),
                },
                TackyInstruction::JumpIfNotZero {
                    condition: TackyValue::Variable(String::from("tmp.0")),
                    target: String::from("or_true0"),
                },
                TackyInstruction::JumpIfNotZero {
                    condition: TackyValue::Constant(2),
                    target: String::from("or_true0"),
                },
                TackyInstruction::Copy {
                    source: TackyValue::Constant(0),
                    destination: TackyValue::Variable(String::from("tmp.1")),
                },
                TackyInstruction::Jump {
                    target: String::from("or_end1"),
                },
                TackyInstruction::Label(String::from("or_true0")),
                TackyInstruction::Copy {
                    source: TackyValue::Constant(1),
                    destination: TackyValue::Variable(String::from("tmp.1")),
                },
                TackyInstruction::Label(String::from("or_end1")),
            ]
        );
    }

    #[test]
    fn test_emit_ast() {
        let identifier = "main".to_string();
        let mut tacky_emitter = TackyEmitter::new();
        let cmm_ast = CmmAst::Program {
            function: CmmFunction::Function {
                identifier: identifier.clone(),
                body: CmmStatement::Return {
                    expression: CmmExpression::Unary {
                        operator: CmmUnaryOperator::Negate,
                        expression: Box::new(CmmExpression::Unary {
                            operator: CmmUnaryOperator::Complement,
                            expression: Box::new(CmmExpression::IntegerConstant { value: 1 }),
                        }),
                    },
                },
            },
        };
        let tacky_ast = tacky_emitter.convert_ast(cmm_ast);
        assert_eq!(
            tacky_ast,
            Ok(TackyAst::Program {
                function: TackyFunction::Function {
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
                        TackyInstruction::Return {
                            value: TackyValue::Variable(String::from("tmp.1"))
                        },
                    ]
                }
            })
        );
    }
}
