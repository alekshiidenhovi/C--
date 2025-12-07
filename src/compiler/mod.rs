pub mod lexer;

use std::io;
use std::path::Path;

pub fn run_cmm_compiler(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input_str = std::fs::read_to_string(input_path)?;
    let _tokens = lexer::tokenize(&input_str);
    let assembly_code =
        String::from("This would be assembly code, when we have implemented the compiler");
    std::fs::write(output_path, assembly_code)
}
