use cmm::common::validation;
use cmm::compiler::{CompilerResult, Stage, run_cmm_compiler};
use cmm::compiler_driver::{run_gcc_linker, run_gcc_preprocessor};

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = "CMM Compiler Driver")]
struct CliArgs {
    /// Input file to process.
    c_file_path: PathBuf,

    /// Tokenizes the C-- source code into tokens
    #[clap(long, conflicts_with_all = &["parse", "codegen", "tacky"], group = "operation")]
    lex: bool,

    /// Parses tokens into an AST
    #[clap(long, conflicts_with_all = &["lex", "codegen", "tacky"], group = "operation")]
    parse: bool,

    /// Emits a TACKY IR from the AST
    #[clap(long, conflicts_with_all = &["lex", "parse", "codegen"], group = "operation")]
    tacky: bool,

    /// Generates machine code from TACKY IR
    #[clap(long, conflicts_with_all = &["lex", "parse", "tacky"], group = "operation")]
    codegen: bool,

    /// Stops the compiler after assembly code generation.
    #[clap(short = 'S', conflicts_with_all = &["lex", "parse", "codegen", "tacky"], group = "operation")]
    stop_after_cmm_compiler: bool,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let c_file_path = args.c_file_path;

    if !c_file_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "Input file '{}' does not exist or is not a file",
                c_file_path.display()
            ),
        )
        .into());
    }

    let process_until = match (args.lex, args.parse, args.tacky, args.codegen) {
        (true, false, false, false) => Some(Stage::Lex),
        (false, true, false, false) => Some(Stage::Parse),
        (false, false, true, false) => Some(Stage::Tacky),
        (false, false, false, true) => Some(Stage::Codegen),
        _ => None,
    };

    let (preprocessor_input_path, preprocessor_output_path) =
        validation::validate_preprocessor_paths(Path::new(&c_file_path), None)?;
    let _ = run_gcc_preprocessor(&preprocessor_input_path, &preprocessor_output_path);

    let (compiler_input_path, compiler_output_path) =
        validation::validate_compiler_paths(&preprocessor_output_path, None)?;
    let compilation_result =
        run_cmm_compiler(&compiler_input_path, &compiler_output_path, &process_until);
    std::fs::remove_file(&preprocessor_output_path)?;

    match compilation_result {
        Ok(CompilerResult::Lexer(tokens)) => {
            println!("Lexer output: {:?}", tokens);
            return Ok(());
        }
        Ok(CompilerResult::Parser(ast)) => {
            println!("Parser output: {:?}", ast);
            return Ok(());
        }
        Ok(CompilerResult::Tacky(tacky_ast)) => {
            println!("TACKY IR output: {:?}", tacky_ast);
            return Ok(());
        }
        Ok(CompilerResult::Codegen(assembly_ast)) => {
            println!("Codegen output: {:?}", assembly_ast);
            return Ok(());
        }
        Ok(CompilerResult::Final(_)) => {}
        Err(e) => {
            return Err(e);
        }
    }

    if args.stop_after_cmm_compiler {
        match compilation_result {
            Ok(CompilerResult::Final(assembly_code)) => {
                println!("Assembly code output: {:?}", assembly_code);
                return Ok(());
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Assembly code output not available! Something went wrong during compilation."
                ));
            }
        }
    }

    let (linker_input_path, linker_output_path) =
        validation::validate_linker_paths(&compiler_output_path, None)?;
    let _ = run_gcc_linker(&linker_input_path, &linker_output_path);
    std::fs::remove_file(&compiler_output_path)?;

    Ok(())
}
