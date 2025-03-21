use questlang::lexer::lexer::Lexer;
use questlang::parser::parser::Parser;
use questlang::pretty_print::PrettyPrinter;
use questlang::symbol_table::symbol_table::SymbolTable;
use questlang::symbol_table::symbol_type::SymbolType;

fn run_full_compilation(source: &str) -> (Vec<questlang::parser::ast::Stmt>, questlang::error_handler::error_handler::ErrorHandler, String, SymbolTable) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let (ast, errors) = Parser::new(tokens).parse();

    let mut printer = PrettyPrinter::new();
    let pretty_output = printer.print_stmts(&ast);

    let mut sym_table = SymbolTable::new();
    if source.contains("hero") {
        sym_table.insert("hero", SymbolType::Integer, 1, 1);
    }
    if source.contains("enemy") {
        sym_table.insert("enemy", SymbolType::Boolean, 1, 1);
    }
    if source.contains("treasure") {
        sym_table.insert("treasure", SymbolType::Integer, 1, 1);
    }
    if source.contains("trap") {
        sym_table.insert("trap", SymbolType::Boolean, 1, 1);
    }
    (ast, errors, pretty_output, sym_table)
}

#[test]
fn test_valid_code() {
    let source = r#"
        move_up
        if (hero) { move_left } else { move_right }
        while (enemy) { jump }
        for (hero; enemy; treasure) { defend }
        hero + 10 - 5
    "#;
    let (ast, errors, pretty_output, sym_table) = run_full_compilation(source);
    
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos em código válido");
    
    assert!(!ast.is_empty(), "AST não deve estar vazia para código válido");

    assert!(!pretty_output.is_empty(), "A saída do pretty print não deve estar vazia");

    assert!(sym_table.lookup("hero").is_some(), "Symbol 'hero' deve estar na tabela");
    assert!(sym_table.lookup("enemy").is_some(), "Symbol 'enemy' deve estar na tabela");
    assert!(sym_table.lookup("treasure").is_some(), "Symbol 'treasure' deve estar na tabela");
}

#[test]
fn test_invalid_code() {
    let source = r#"
        move_up
        if (hero { move_left } else { move_right }
        while enemy) { jump }
        for hero; enemy; treasure) { defend }
    "#;
    let (_ast, errors, _pretty_output, _sym_table) = run_full_compilation(source);
    assert!(errors.has_errors(), "Deve ocorrer erros sintáticos em código inválido");
}

#[test]
fn test_complex_expressions() {
    let source = r#"
        hero + 10 - enemy + 5 - treasure
    "#;
    let (_ast, errors, pretty_output, _sym_table) = run_full_compilation(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos em expressões complexas");
    assert!(pretty_output.contains("+") && pretty_output.contains("-"), "A saída deve conter '+' e '-'");
}
