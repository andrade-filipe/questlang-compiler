mod error_handler;
mod lexer;
mod parser;
mod pretty_print;
mod symbol_table;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use pretty_print::PrettyPrinter;
use symbol_table::symbol_table::SymbolTable;
use symbol_table::symbol_type::SymbolType;

fn main() {
    // Exemplo de código em QuestLang
    let source_code = r#"
        move_up
        if (hero) { move_left } else { move_right }
        while (enemy) { jump }
        for (hero; enemy; treasure) { defend }
    "#;

    // --- Lexical Analysis ---
    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();
    println!("--- Tokens ---");
    for (token, text) in tokens.iter() {
        println!("{:?} -> '{}'", token, text);
    }

    // --- Parsing ---
    // O parser usa o ASTBuilder para construir a árvore sintática.
    // Ele também acumula eventuais erros de parsing.
    let (ast, errors) = Parser::new(tokens).parse();
    if errors.has_errors() {
        println!("\n--- Parsing Errors ---");
        errors.report();
    } else {
        println!("\nParsing concluído sem erros.");
    }

    // --- Pretty Print ---
    // Utiliza o PrettyPrinter para gerar uma representação formatada da AST.
    let mut printer = PrettyPrinter::new();
    let pretty_ast = printer.print_stmts(&ast);
    println!("\n--- Pretty Printed AST ---");
    println!("{}", pretty_ast);

    // --- Symbol Table ---
    // Exemplo simples de uso da Symbol Table, inserindo alguns símbolos.
    let mut sym_table = SymbolTable::new();
    sym_table.insert("hero", SymbolType::Integer, 1, 1);
    sym_table.insert("enemy", SymbolType::Boolean, 2, 1);
    sym_table.insert("treasure", SymbolType::Integer, 3, 1);
    println!("\n--- Symbol Table ---");
    sym_table.print_table();
}
