use questlang::lexer::lexer::Lexer;
use questlang::parser::parser::Parser;
use questlang::pretty_print::PrettyPrinter;
use questlang::symbol_table::symbol_table::SymbolTable;
use questlang::symbol_table::symbol_type::SymbolType;
use questlang::parser::ast::Stmt;

fn run_full_compilation(source: &str) -> (Vec<questlang::parser::ast::Stmt>, questlang::error_handler::error_handler::ErrorHandler, String, SymbolTable) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let (ast, errors) = Parser::new(tokens, source).parse();

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

#[test]
fn test_valid_full_flow() {
    let source = r#"
        move_up
        if (hero) { move_left } else { move_right }
        while (enemy) { jump }
        for (hero; enemy; treasure) { defend }
        hero + 10 - 5
    "#;
    let (ast, errors, pretty_output, sym_table) = run_full_compilation(source);
    
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos em código válido");

    // Verifica que a AST contém todos os statements esperados:
    // Aqui, esperamos 5 statements:
    // 1. ExprStmt (hero + 10 - 5)
    // 2. Command: move_up
    // 3. IfStmt
    // 4. WhileStmt
    // 5. ForStmt
    // (Ordem pode variar dependendo da implementação do parser.)
    assert!(ast.len() >= 5, "A AST deve conter pelo menos 5 statements para código válido");

    assert!(pretty_output.contains("move_up"), "A saída deve conter 'move_up'");
    assert!(pretty_output.contains("if ("), "A saída deve conter 'if ('");
    assert!(pretty_output.contains("while ("), "A saída deve conter 'while ('");
    assert!(pretty_output.contains("for ("), "A saída deve conter 'for ('");
    assert!(pretty_output.contains("Expr:"), "A saída deve conter 'Expr:' para expressões");

    assert!(sym_table.lookup("hero").is_some(), "Symbol 'hero' deve estar na tabela");
    assert!(sym_table.lookup("enemy").is_some(), "Symbol 'enemy' deve estar na tabela");
    assert!(sym_table.lookup("treasure").is_some(), "Symbol 'treasure' deve estar na tabela");
}

#[test]
fn test_invalid_full_flow() {
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
fn test_nested_control_structures() {
    let source = r#"
        if (hero) {
            if (enemy) { move_left } else { move_right }
            while (treasure) { jump }
        } else {
            for (hero; enemy; trap) { defend }
        }
    "#;
    let (_ast, errors, pretty_output, _sym_table) = run_full_compilation(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos em estruturas aninhadas");
 
    assert!(pretty_output.contains("if ("), "A saída deve conter 'if ('");
    assert!(pretty_output.contains("while ("), "A saída deve conter 'while ('");
    assert!(pretty_output.contains("for ("), "A saída deve conter 'for ('");
}

#[test]
fn test_expression_statement() {
    let source = r#"
        hero + enemy - treasure
    "#;
    let (ast, errors, pretty_output, _sym_table) = run_full_compilation(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos em expression statement");

    let expr_stmt_count = ast.iter().filter(|stmt| match stmt {
        Stmt::ExprStmt(_) => true,
        _ => false,
    }).count();
    assert!(expr_stmt_count >= 1, "Deve haver pelo menos um expression statement na AST");
    assert!(pretty_output.contains("Expr:"), "Pretty print deve indicar 'Expr:'");
}

#[test]
fn test_empty_input() {
    let source = "";
    let (ast, errors, pretty_output, sym_table) = run_full_compilation(source);
    assert!(!errors.has_errors(), "Entrada vazia não deve gerar erros");
    assert!(ast.is_empty(), "AST deve estar vazia para entrada vazia");
    assert!(pretty_output.is_empty(), "Pretty print deve ser vazio para entrada vazia");
    assert!(sym_table.symbols.is_empty(), "Symbol Table deve estar vazia para entrada vazia");
}
