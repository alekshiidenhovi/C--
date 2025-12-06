use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Checks if a given path has a specific file extension.
///
/// # Arguments
///
/// * `path`: The path to check.
/// * `extension`: The desired file extension (e.g., "c", "i").
///
/// # Returns
///
/// `true` if the path has the specified extension, `false` otherwise.
fn is_valid_path_extension(path: &Path, extension: &str) -> bool {
    path.extension().map_or(false, |ext| ext == extension)
}

/// Runs the GCC preprocessor on a C source file.
///
/// This function invokes `gcc -E -P` to perform preprocessing, expanding
/// macros and handling include directives, but stopping before compilation.
///
/// # Arguments
///
/// * `input_path`: The path to the input C source file. Must have a `.c` extension.
/// * `output_path`: An optional path for the preprocessed output file. If `None`, the output file
///   will be created in the same directory as the input file with the extension `.i`.
///
/// # Returns
///
/// Returns `Ok(())` on successful preprocessing, or an `anyhow::Error` if:
/// - The input path does not have a `.c` extension.
/// - The input path does not exist or is not a file.
/// - The output path (if provided) does not have a `.i` extension.
/// - The output file already exists when no explicit `output_path` is given.
/// - GCC preprocessing fails or is not found.
pub fn run_gcc_preprocessor(input_path: &Path, output_path: Option<&Path>) -> Result<()> {
    println!("Invoking GCC Preprocessor...");

    if !is_valid_path_extension(input_path, "c") {
        return Err(anyhow!(
            "Input path must have a '.c' extension: {}",
            input_path.display()
        ));
    }

    if !input_path.is_file() {
        return Err(anyhow!(
            "Input file does not exist or is not a file: {}",
            input_path.display()
        ));
    }

    let final_output_path: PathBuf = match output_path {
        Some(path) => {
            if !is_valid_path_extension(path, "i") {
                return Err(anyhow!("Output path must end with '.i' extension"));
            }
            path.to_path_buf()
        }
        None => {
            let input_file_stem = input_path.file_stem().ok_or_else(|| {
                anyhow!(
                    "Failed to get file stem from input path: {}",
                    input_path.display()
                )
            })?;

            let mut path_buf = PathBuf::from(input_file_stem);
            path_buf.set_extension("i");

            if path_buf.is_file() {
                return Err(anyhow!(
                    "Output file already exists: {}",
                    path_buf.display()
                ));
            }
            path_buf
        }
    };

    let status = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input_path)
        .arg("-o")
        .arg(&final_output_path)
        .status()
        .context("Failed to execute GCC preprocessing. Is it installed and in your PATH?")?;

    if status.success() {
        println!(
            "Preprocessed file created at: {}",
            final_output_path.display()
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "GCC Preprocessor failed with exit code: {:?}",
            status.code()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path_extension() {
        let path = Path::new("src/compiler_driver.c");
        assert!(is_valid_path_extension(path, "c"));
    }

    #[test]
    fn test_invalid_path_extension() {
        let path = Path::new("src/compiler_driver.rs");
        assert!(!is_valid_path_extension(path, "c"));
    }

    #[test]
    fn test_no_path_extension() {
        let path = Path::new("src/compiler_driver");
        assert!(!is_valid_path_extension(path, "c"));
    }

    #[test]
    fn test_preprocessor_invalid_input_file_extension() {
        let input_path = Path::new("src/compiler_driver.rs");
        let output_path = None;
        let result = run_gcc_preprocessor(input_path, output_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_preprocessor_invalid_output_file_extension() {
        let input_path = Path::new("src/compiler_driver.c");
        let output_path = Some(Path::new("src/compiler_driver.rs"));
        let result = run_gcc_preprocessor(input_path, output_path);
        assert!(result.is_err());
    }
}
