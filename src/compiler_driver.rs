use crate::common::validation::validate_preprocessor_paths;
use anyhow::Context;
use std::path::Path;
use std::process::Command;

/// Runs the GCC preprocessor on a C source file.
///
/// This function invokes `gcc -E -P` to perform preprocessing, expanding
/// macros and handling include directives, but stopping before compilation.
///
/// # Arguments
///
/// * `input_path`: The path to the input C source file. Must have a `.c` extension.
/// * `output_path`: The path to the output preprocessed C source file. Must have an `.i` extension.
///
/// # Returns
///
/// Returns `Ok(())` on successful preprocessing, or an `anyhow::Error` if:
/// - GCC preprocessing fails or is not found.
pub fn run_gcc_preprocessor(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    println!("Invoking GCC Preprocessor...");

    let status = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .status()
        .context("Failed to execute GCC preprocessing. Is it installed and in your PATH?")?;

    if status.success() {
        println!("Preprocessed file created at: {}", output_path.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "GCC Preprocessor failed with exit code: {:?}",
            status.code()
        ))
    }
}

pub fn run_compiler_driver(input_path: &Path) -> anyhow::Result<()> {
    let (input_path, output_path) = validate_preprocessor_paths(input_path, None)?;
    run_gcc_preprocessor(&input_path, &output_path)
}
