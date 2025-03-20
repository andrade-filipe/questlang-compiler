mod lexer;
mod symbol_table;
mod error_handler;

use error_handler::error_handler::ErrorHandler;
use error_handler::error_type::ErrorType;
use symbol_table::symbol_table::SymbolTable;
use symbol_table::symbol_type::SymbolType;
use lexer::lexer::Lexer;

fn main() {
    let mut error_handler = ErrorHandler::new();

    // Adicionando erros de diferentes tipos
    error_handler.add_error(ErrorType::Lexical, "Token inválido encontrado", 2, 10);
    error_handler.add_error(ErrorType::Syntactic, "Esperado ';' após a instrução", 4, 5);
    error_handler.add_error(ErrorType::Semantic, "Variável não declarada", 6, 15);

    // Adicionando warnings
    error_handler.add_warning("Uso de variável não inicializada", 8, 3);

    // Exibir os erros acumulados
    error_handler.report();

    // Limpar os erros
    error_handler.clear();

    // Teste após limpar
    println!("Após limpar:");
    error_handler.report();
}
