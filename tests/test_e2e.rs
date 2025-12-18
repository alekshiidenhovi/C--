use cmm::compiler::{CompilerResult, Stage, run_cmm_compiler};

#[test]
fn test_integer_constant() {
    insta::glob!("test_programs/*.c", |path| {
        let source_code = std::fs::read_to_string(path).unwrap();
        let lexer_result = run_cmm_compiler(&source_code, &Some(Stage::Lex)).unwrap();
        let tokens = match lexer_result {
            CompilerResult::Lexer(tokens) => tokens,
            _ => panic!("Expected lexer result"),
        };
        insta::assert_debug_snapshot!("lexer", tokens);

        let parser_result = run_cmm_compiler(&source_code, &Some(Stage::Parse)).unwrap();
        let cmm_ast = match parser_result {
            CompilerResult::Parser(cmm_ast) => cmm_ast,
            _ => panic!("Expected parser result"),
        };
        insta::assert_debug_snapshot!("parser", cmm_ast);

        let ir_gen_result = run_cmm_compiler(&source_code, &Some(Stage::Tacky)).unwrap();
        let tacky_ast = match ir_gen_result {
            CompilerResult::Tacky(tacky_ast) => tacky_ast,
            _ => panic!("Expected tacky result"),
        };
        insta::assert_debug_snapshot!("tacky", tacky_ast);

        let code_gen_result = run_cmm_compiler(&source_code, &Some(Stage::Codegen)).unwrap();
        let asm_ast = match code_gen_result {
            CompilerResult::Codegen(asm_ast) => asm_ast,
            _ => panic!("Expected code gen result"),
        };
        insta::assert_debug_snapshot!("codegen", asm_ast);

        let code_emission_result = run_cmm_compiler(&source_code, &None).unwrap();
        let assembly_code = match code_emission_result {
            CompilerResult::Final(assembly_code) => assembly_code,
            _ => panic!("Expected final result"),
        };
        insta::assert_snapshot!("assembly_code", assembly_code);
    });
}
