use anyhow::anyhow;
use std::path::{Path, PathBuf};

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

/// Validates preprocessor paths and their respective files.
///
/// # Arguments
///
/// * `input_path`: The path to the input C source file. Must have a `.c` extension.
/// * `output_path`: An optional path for the preprocessed output file.
///
/// # Returns
///
/// Returns `Ok(())` on successful preprocessing, or an `anyhow::Error` if:
/// - The input path does not have a `.c` extension.
/// - The input path does not exist or is not a file.
/// - The output path (if provided) does not have a `.i` extension.
/// - The output file already exists when no explicit `output_path` is given.
pub fn validate_preprocessor_paths(
    input_path: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
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
            let path_buf = input_path.with_extension("i");

            if path_buf.exists() {
                return Err(anyhow!(
                    "Output file already exists: {}",
                    path_buf.display()
                ));
            }
            path_buf
        }
    };

    return Ok(((*input_path).to_path_buf(), final_output_path));
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
}
