pub mod code_emission;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod tokens;

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
    /// Stop after the code generation stage.
    Codegen,
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
) -> anyhow::Result<()> {
    println!("Compiling with a custom C compiler...");
    let input_str = std::fs::read_to_string(input_path)?;
    let tokens = lexer::tokenize(&input_str);

    if let Some(Stage::Lex) = process_until {
        return Ok(());
    }

    let mut parser = Parser::new(tokens);
    let c_ast = parser.parse_ast()?;

    if let Some(Stage::Parse) = process_until {
        return Ok(());
    }

    let assembly_ast = codegen::convert_ast(c_ast)?;

    if let Some(Stage::Codegen) = process_until {
        return Ok(());
    }

    let assembly_code = code_emission::emit_assembly(&assembly_ast);
    let _ = std::fs::write(output_path, assembly_code);

    println!("Assembly code created at: {}", output_path.display());

    Ok(())
}
