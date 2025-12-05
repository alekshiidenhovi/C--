use anyhow::{Context, Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run_gcc_preprocessor(input_path: &Path, output_path: Option<&Path>) -> Result<()> {
    println!("Invoking GCC Preprocessor...");

    if input_path.extension().map_or(true, |ext| ext != "c") {
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
            if path.extension().map_or(true, |ext| ext != "i") {
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
