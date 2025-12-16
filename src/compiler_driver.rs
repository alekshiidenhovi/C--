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
/// * `source_file_path`: The path to the input C source file. Must have a `.c` extension.
/// * `preprocessed_file_path`: The path to the output preprocessed C source file. Must have an `.i` extension.
///
/// # Returns
///
/// Returns `Ok(())` on successful preprocessing, or an `anyhow::Error` if:
/// - GCC preprocessing fails or is not found.
pub fn run_gcc_preprocessor(
    source_file_path: &Path,
    preprocessed_file_path: &Path,
) -> anyhow::Result<()> {
    println!("Invoking GCC Preprocessor...");

    let status = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(source_file_path)
        .arg("-o")
        .arg(preprocessed_file_path)
        .status()
        .context("Failed to execute GCC preprocessing. Is it installed and in your PATH?")?;

    if status.success() {
        println!(
            "Preprocessed file created at: {}",
            preprocessed_file_path.display()
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "GCC Preprocessor failed with exit code: {:?}",
            status.code()
        ))
    }
}

/// Run the GCC linker to create an executable from an assembly file.
///
/// This function invokes `gcc -o` to perform linking, and forming the final executable.
///
/// # Arguments
///
/// * `assembly_file_path`: A reference to the `Path` of the assembly file to link.
/// * `executable_path`: A reference to the `Path` where the executable should be created.
///
/// # Returns
///
/// Returns `Ok(())` if the linking process is successful.
/// Returns an `anyhow::Result` with an error if the GCC linker fails to execute or fails during the linking process.
pub fn run_gcc_linker(assembly_file_path: &Path, executable_path: &Path) -> anyhow::Result<()> {
    println!("Invoking GCC Linker...");

    let status = Command::new("gcc")
        .arg(assembly_file_path)
        .arg("-o")
        .arg(executable_path)
        .status()
        .context("Failed to execute GCC Linker. Is it installed and in your PATH?")?;

    if status.success() {
        println!("Executable file created at: {}", executable_path.display());
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "GCC Linker failed with exit code: {:?}",
            status.code()
        ))
    }
}
