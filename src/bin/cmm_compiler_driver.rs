use cmm::compiler_driver::run_compiler_driver;

use std::path::Path;

fn main() {
    let input_path = Path::new("src/compiler_driver.c");
    run_compiler_driver(&input_path).expect("Failed to run compiler driver");
}
