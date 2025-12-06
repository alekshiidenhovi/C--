use cmm::common::validation::validate_preprocessor_paths;
use std::fs::File;
use std::path::PathBuf;
use tempfile::tempdir;

fn setup_test_files(dir: &tempfile::TempDir, input_name: &str, input_ext: &str) -> PathBuf {
    let input_path = dir.path().join(format!("{}.{}", input_name, input_ext));
    if !input_path.exists() {
        File::create(&input_path).expect("Failed to create mock input file");
    }
    input_path
}

#[test]
fn test_invalid_input_ext_explicit_output() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = setup_test_files(&temp_dir, "existing_file", "s");

    let result = validate_preprocessor_paths(&input_path, None);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must have a '.c' extension")
    );
}

#[test]
fn test_input_file_does_not_exist() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = temp_dir.path().join("non_existent_file.c");
    let output_path = temp_dir.path().join("output.i");

    let result = validate_preprocessor_paths(&input_path, Some(&output_path));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("does not exist or is not a file")
    );
}

#[test]
fn test_invalid_output_extension() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = setup_test_files(&temp_dir, "existing_file", "c");
    let output_path = temp_dir.path().join("nonexisting_output.s");

    let result = validate_preprocessor_paths(&input_path, Some(&output_path));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must end with '.i' extension")
    );
}

#[test]
fn test_default_output_already_exists() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = setup_test_files(&temp_dir, "main", "c");

    let expected_output = input_path.with_extension("i");
    File::create(&expected_output).expect("Failed to create existing default output file");

    let result = validate_preprocessor_paths(&input_path, None);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Output file already exists")
    );
}

#[test]
fn test_valid_paths_explicit_output() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = setup_test_files(&temp_dir, "existing_file", "c");
    let output_path = temp_dir.path().join("nonexisting_output.i");

    let result = validate_preprocessor_paths(&input_path, Some(&output_path));
    assert!(result.is_ok());
    let (input_out, output_out) = result.unwrap();
    assert_eq!(input_out, input_path);
    assert_eq!(output_out, output_path);
}

#[test]
fn test_valid_paths_default_output() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let input_path = setup_test_files(&temp_dir, "main", "c");
    let result = validate_preprocessor_paths(&input_path, None);

    assert!(result.is_ok());
    let (input_out, output_out) = result.unwrap();

    assert_eq!(input_out, input_path);

    let expected_output = input_path.with_extension("i");
    assert_eq!(output_out, expected_output);
}
