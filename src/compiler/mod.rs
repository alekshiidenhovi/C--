pub mod code_emission;
pub mod code_gen;
pub mod ir_gen;
pub mod lexer;
pub mod parser;
pub mod tokens;

use crate::compiler::tokens::Token;
use parser::Parser;

/// Represents the different stages a C-- compilation can proceed to.
///
/// This enum allows for early termination of the compilation process after a specific stage.
#[derive(Debug)]
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
#[derive(Debug)]
pub enum CompilerResult {
    /// The result of the lexer, a vector of tokens.
    Lexer(Vec<Token>),
    /// The result of the parser, an Abstract Syntax Tree (AST).
    Parser(parser::cmm_ast::CmmAst),
    /// The result of the Tacky intermediate representation generation.
    Tacky(ir_gen::tacky_ast::TackyAst),
    /// The result of the code generator, an assembly AST.
    Codegen(code_gen::assembly_ast::AssemblyAst),
    /// The final emitted code as a string.
    Final(String),
}

/// Compiles a preprocessed C-- source code to assembly code.
///
/// This function orchestrates the entire compilation pipeline, from lexing to assembly emission.
/// It can be configured to stop at a specific stage using the `process_until` argument.
///
/// # Arguments
///
/// * `cmm_source_code`: The source code to compile.
/// * `process_until`: An optional `Stage` to specify the maximum compilation stage to reach.
///
/// # Returns
///
/// Returns `Ok(())` on successful compilation, or an `anyhow::Error` if any stage of the compilation fails.
pub fn run_cmm_compiler(
    cmm_source_code: &str,
    process_until: &Option<Stage>,
) -> anyhow::Result<CompilerResult> {
    println!("Compiling with a custom C compiler...");
    let tokens = lexer::tokenize(cmm_source_code);

    if let Some(Stage::Lex) = process_until {
        return Ok(CompilerResult::Lexer(tokens));
    }

    let mut parser = Parser::new(tokens);
    let cmm_ast = parser.parse_ast()?;

    if let Some(Stage::Parse) = process_until {
        return Ok(CompilerResult::Parser(cmm_ast));
    }

    let mut tacky_emitter = ir_gen::TackyEmitter::new();
    let tacky_ast = tacky_emitter.convert_ast(cmm_ast)?;

    if let Some(Stage::Tacky) = process_until {
        return Ok(CompilerResult::Tacky(tacky_ast));
    }

    let assembly_ast = code_gen::convert_ast(tacky_ast)?;

    if let Some(Stage::Codegen) = process_until {
        return Ok(CompilerResult::Codegen(assembly_ast));
    }

    let assembly_code = code_emission::emit_assembly(&assembly_ast);

    Ok(CompilerResult::Final(assembly_code))
}
