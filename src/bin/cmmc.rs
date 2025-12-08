use cmm::compiler::run_cmm_compiler;

use std::path::Path;

fn main() {
    let _ = run_cmm_compiler(Path::new("main.i"), Path::new("main.s"));
}
