pub mod code_emission;
pub mod code_gen;
pub mod ir_gen;
pub mod lexer;
pub mod parser;
pub mod tokens;

use crate::compiler::tokens::Token;
use parser::Parser;
use std::path::Path;

/// Represents the different stages a C-- compilation can proceed to.
///
/// This enum allows for early termination of the compilation process after a specific stage.
pub enum Stage {
    /// Stop after the lexing stage.
    Lex,
    /// Stop after the parsing stage.
    Parse,
    /// Stop after the TACKY IR stage.
    Tacky,
    /// Stop after the code generation stage.
    Codegen,
}

/// Represents the possible outcomes of a compiler stage.
///
/// Each variant encapsulates the successful result of a specific phase in the compilation process,
/// from lexical analysis to code emission.
pub enum CompilerResult {
    /// The result of the lexer, a vector of tokens.
    Lexer(Vec<Token>),
    /// The result of the parser, an Abstract Syntax Tree (AST).
    Parser(parser::ast::Ast),
    /// The result of the Tacky intermediate representation generation.
    Tacky(ir_gen::tacky_ir::TackyIR),
    /// The result of the code generator, an assembly AST.
    Codegen(code_gen::asm_ast::AssemblyAst),
    /// The final emitted code as a string.
    Final(String),
}

/// Compiles a preprocessed C-- source file to assembly code.
///
/// This function orchestrates the entire compilation pipeline, from lexing to assembly emission.
/// It can be configured to stop at a specific stage using the `process_until` argument.
///
/// # Arguments
///
/// * `input_path`: The path to the C-- source file to compile.
/// * `output_path`: The path where the generated assembly code will be written.
/// * `process_until`: An optional `Stage` to specify the maximum compilation stage to reach.
///
/// # Returns
///
/// Returns `Ok(())` on successful compilation, or an `anyhow::Error` if any stage of the
/// compilation fails.
pub fn run_cmm_compiler(
    input_path: &Path,
    output_path: &Path,
    process_until: &Option<Stage>,
) -> anyhow::Result<CompilerResult> {
    println!("Compiling with a custom C compiler...");
    let input_str = std::fs::read_to_string(input_path)?;
    let tokens = lexer::tokenize(&input_str);

    if let Some(Stage::Lex) = process_until {
        return Ok(CompilerResult::Lexer(tokens));
    }

    let mut parser = Parser::new(tokens);
    let c_ast = parser.parse_ast()?;

    if let Some(Stage::Parse) = process_until {
        return Ok(CompilerResult::Parser(c_ast));
    }

    let mut tacky_emitter = ir_gen::TackyEmitter::new();
    let tacky_ast = tacky_emitter.convert_ast(c_ast)?;

    if let Some(Stage::Tacky) = process_until {
        return Ok(CompilerResult::Tacky(tacky_ast));
    }

    let assembly_ast = code_gen::convert_ast(tacky_ast)?;

    if let Some(Stage::Codegen) = process_until {
        return Ok(CompilerResult::Codegen(assembly_ast));
    }

    let assembly_code = code_emission::emit_assembly(&assembly_ast);
    let _ = std::fs::write(output_path, assembly_code.clone());

    println!("Assembly code created at: {}", output_path.display());

    Ok(CompilerResult::Final(assembly_code))
}
