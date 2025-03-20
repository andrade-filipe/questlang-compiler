use questlang::error_handler::error_handler::{ErrorHandler, ErrorType, CompilerError};

#[test]
fn test_add_error() {
    let mut handler = ErrorHandler::new();
    // Inicialmente, não há erros
    assert!(!handler.has_errors(), "Nenhum erro deve existir inicialmente");

    // Adiciona um erro e verifica se foi registrado
    handler.add_error(ErrorType::Lexical, "Token inválido", 1, 5);
    assert!(handler.has_errors(), "Deve haver erros após adicionar um");
}

#[test]
fn test_add_warning() {
    let mut handler = ErrorHandler::new();
    // Warnings não contam como erros, então has_errors() deve retornar false
    handler.add_warning("Uso de variável não inicializada", 2, 10);
    assert!(!handler.has_errors(), "Warnings não devem ser considerados como erros");
}

#[test]
fn test_clear() {
    let mut handler = ErrorHandler::new();
    handler.add_error(ErrorType::Syntactic, "Falta de ';'", 3, 15);
    handler.add_warning("Variável não utilizada", 4, 20);
    
    // Deve haver erros antes de limpar
    assert!(handler.has_errors(), "Deve haver erros antes de limpar");
    
    handler.clear();
    assert!(!handler.has_errors(), "Após limpar, não deve haver erros");
}

#[test]
fn test_display_error() {
    let error = CompilerError::new(ErrorType::Semantic, "Variável não declarada", 5, 25);
    let formatted = format!("{}", error);
    let expected = "[Semantic Error] Variável não declarada (Linha 5, Coluna 25)";
    assert_eq!(formatted, expected, "A formatação do erro deve corresponder ao esperado");
}

/// Testa a função report() para garantir que não haja panics durante sua execução.
/// Para esse teste, apenas chamamos report() e verificamos que a função roda sem causar erros.
#[test]
fn test_report() {
    let mut handler = ErrorHandler::new();
    handler.add_error(ErrorType::Lexical, "Caractere inesperado", 1, 1);
    handler.add_warning("Uso de função obsoleta", 2, 2);
    
    // Apenas garantir que report() roda sem panicar.
    handler.report();
}
