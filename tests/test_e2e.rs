use cmm::compiler::{Stage, run_cmm_compiler};

#[test]
fn test_integer_constant() {
    insta::glob!("test_programs/*.c", |path| {
        let source_code = std::fs::read_to_string(path).unwrap();
        let tokens = run_cmm_compiler(&source_code, &Some(Stage::Lex)).unwrap();
        insta::assert_debug_snapshot!("lexer", tokens);

        let cmm_ast = run_cmm_compiler(&source_code, &Some(Stage::Parse)).unwrap();
        insta::assert_debug_snapshot!("parser", cmm_ast);

        let tacky_ast = run_cmm_compiler(&source_code, &Some(Stage::Tacky)).unwrap();
        insta::assert_debug_snapshot!("tacky", tacky_ast);

        let asm_ast = run_cmm_compiler(&source_code, &Some(Stage::Codegen)).unwrap();
        insta::assert_debug_snapshot!("codegen", asm_ast);

        let assembly_code = run_cmm_compiler(&source_code, &None).unwrap();
        insta::assert_debug_snapshot!("assembly_code", assembly_code);
    });
}
