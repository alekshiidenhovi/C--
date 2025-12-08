use cmm::common::validation;
use cmm::compiler::{Stage, run_cmm_compiler};
use cmm::compiler_driver::{run_gcc_linker, run_gcc_preprocessor};

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = "CMM Compiler Driver")]
struct CliArgs {
    /// Input file to process.
    c_file_path: PathBuf,

    /// Tokenizes the source code and prints the tokens.
    #[clap(long, conflicts_with_all = &["parse", "codegen"], group = "operation")]
    lex: Option<bool>,

    /// Parses the tokens into an AST and prints the structure.
    #[clap(long, conflicts_with_all = &["lex", "codegen"], group = "operation")]
    parse: Option<bool>,

    /// Generates machine code from the source and prints assembly.
    #[clap(long, conflicts_with_all = &["lex", "parse"], group = "operation")]
    codegen: Option<bool>,
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

    let process_until = match (args.lex, args.parse, args.codegen) {
        (Some(true), None, None) => Some(Stage::Lex),
        (None, Some(true), None) => Some(Stage::Parse),
        (None, None, Some(true)) => Some(Stage::Codegen),
        _ => None,
    };

    let (preprocessor_input_path, preprocessor_output_path) =
        validation::validate_preprocessor_paths(Path::new(&c_file_path), None)?;
    let _ = run_gcc_preprocessor(&preprocessor_input_path, &preprocessor_output_path);

    let (compiler_input_path, compiler_output_path) =
        validation::validate_compiler_paths(&preprocessor_output_path, None)?;
    let _ = run_cmm_compiler(&compiler_input_path, &compiler_output_path, process_until);
    std::fs::remove_file(&preprocessor_output_path)?;

    let (linker_input_path, linker_output_path) =
        validation::validate_linker_paths(&compiler_output_path, None)?;
    let _ = run_gcc_linker(&linker_input_path, &linker_output_path);
    std::fs::remove_file(&compiler_output_path)?;

    Ok(())
}
