pub mod code_emission;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod tokens;

use parser::Parser;
use std::path::Path;

/// Represents the different stages of a compilation process.
pub enum Stage {
    Lex,
    Parse,
    Codegen,
}

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
