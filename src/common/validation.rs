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

/// Internal helper for path validation across preprocessor, compiler, and linker stages.
fn validate_paths_internal(
    input_path: &Path,
    input_ext: &str,
    output_path: Option<&Path>,
    output_ext: Option<&str>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    if !is_valid_path_extension(input_path, input_ext) {
        return Err(anyhow!(
            "Input path must have a '.{}' extension: {}",
            input_ext,
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
            if let Some(ext) = output_ext {
                if !is_valid_path_extension(path, ext) {
                    return Err(anyhow!("Output path must end with '.{}' extension", ext));
                }
            } else if path.extension().is_some() {
                return Err(anyhow!(
                    "Output path for linker should typically not have a file extension"
                ));
            }
            path.to_path_buf()
        }
        None => {
            if let Some(ext) = output_ext {
                let path_buf = input_path.with_extension(ext);
                if path_buf.exists() {
                    return Err(anyhow!(
                        "Output file already exists: {}",
                        path_buf.display()
                    ));
                }
                path_buf
            } else {
                let file_stem = input_path
                    .file_stem()
                    .ok_or_else(|| anyhow!("Input path has no file stem"))?;
                let path_buf = PathBuf::from(file_stem);
                if path_buf.exists() {
                    return Err(anyhow!(
                        "Output file already exists: {}",
                        path_buf.display()
                    ));
                }
                path_buf
            }
        }
    };

    Ok(((*input_path).to_path_buf(), final_output_path))
}

/// Validates preprocessor paths and their respective files.
///
/// **Input Requirement:** Must have a `.c` extension.
/// **Output Requirement:** Must have a `.i` extension.
///
/// # Arguments
///
/// * `input_path`: The path to the input C source file.
/// * `output_path`: An optional path for the preprocessed output file.
///
/// # Returns
///
/// Returns `Ok((PathBuf, PathBuf))` containing the validated input and output paths on success,
/// or an `anyhow::Error` if validation fails.
pub fn validate_preprocessor_paths(
    input_path: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    validate_paths_internal(input_path, "c", output_path, Some("i"))
}

/// Validates compiler paths and their respective files.
///
/// **Input Requirement:** Must have an `.i` extension.
/// **Output Requirement:** Must have an `.s` extension.
///
/// # Arguments
///
/// * `input_path`: The path to the input preprocessed file.
/// * `output_path`: An optional path for the compiled assembly output file.
///
/// # Returns
///
/// Returns `Ok((PathBuf, PathBuf))` containing the validated input and output paths on success,
/// or an `anyhow::Error` if validation fails.
pub fn validate_compiler_paths(
    input_path: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    validate_paths_internal(input_path, "i", output_path, Some("s"))
}

/// Validates linker paths and their respective files.
///
/// **Input Requirement:** Must have an `.s` extension.
/// **Output Requirement:** No file extension (the final executable).
///
/// # Arguments
///
/// * `input_path`: The path to the input compiled assembly file.
/// * `output_path`: An optional path for the final executable file.
///
/// # Returns
///
/// Returns `Ok((PathBuf, PathBuf))` containing the validated input and output paths on success,
/// or an `anyhow::Error` if validation fails.
pub fn validate_linker_paths(
    input_path: &Path,
    output_path: Option<&Path>,
) -> anyhow::Result<(PathBuf, PathBuf)> {
    validate_paths_internal(input_path, "s", output_path, None)
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
