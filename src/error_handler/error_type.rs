/// Tipos de erro no compilador
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Lexical,
    Syntactic,
    Semantic,
}