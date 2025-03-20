use std::fmt;
use crate::error_handler::error_type::ErrorType;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub error_type: ErrorType,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl CompilerError {
    pub fn new(error_type: ErrorType, message: &str, line: usize, column: usize) -> Self {
        CompilerError {
            error_type,
            message: message.to_string(),
            line,
            column,
        }
    }
}

/// Implementação para exibir os erros formatados
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{:?} Error] {} (Linha {}, Coluna {})",
            self.error_type, self.message, self.line, self.column
        )
    }
}