pub mod lexer;
pub mod tokens;

use std::io;
use std::path::Path;

/// Represents the different stages of a compilation process.
pub enum Stage {
    Lex,
    Parse,
    Codegen,
}

pub fn run_cmm_compiler(
    input_path: &Path,
    output_path: &Path,
    process_until: &Option<Stage>,
) -> io::Result<()> {
    let input_str = std::fs::read_to_string(input_path)?;
    let _tokens = lexer::tokenize(&input_str);

    if let Some(Stage::Lex) = process_until {
        return Ok(());
    }

    let _ast = "This would be the AST, when we have implemented the compiler";

    if let Some(Stage::Parse) = process_until {
        return Ok(());
    }

    let assembly_code = String::from(
        "        .section        __TEXT,__text,regular,pure_instructions
        .build_version macos, 15, 0     sdk_version 15, 5
        .globl  _main                           ## -- Begin function main
        .p2align        4, 0x90
_main:                                  ## @main
        .cfi_startproc
## %bb.0:
        pushq   %rbp
        .cfi_def_cfa_offset 16
        .cfi_offset %rbp, -16
        movq    %rsp, %rbp
        .cfi_def_cfa_register %rbp
        movl    $0, -4(%rbp)
        movl    $2, %eax
        popq    %rbp
        retq
        .cfi_endproc
                                        ## -- End function
.subsections_via_symbols",
    );

    if let Some(Stage::Codegen) = process_until {
        return Ok(());
    }

    std::fs::write(output_path, assembly_code)
}
