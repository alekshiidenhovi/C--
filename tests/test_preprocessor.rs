use cmm::compiler_driver::run_gcc_preprocessor;
use std::path::Path;

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
