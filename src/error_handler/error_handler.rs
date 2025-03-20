use std::fmt;

/// Tipos de erro no compilador
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    Lexical,
    Syntactic,
    Semantic,
}

/// Estrutura para armazenar um erro
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

/// Estrutura para gerenciar os erros do compilador
pub struct ErrorHandler {
    errors: Vec<CompilerError>,
    warnings: Vec<CompilerError>,
}

impl ErrorHandler {
    /// Cria um novo Error Handler
    pub fn new() -> Self {
        ErrorHandler {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Adiciona um erro à lista de erros
    pub fn add_error(&mut self, error_type: ErrorType, message: &str, line: usize, column: usize) {
        self.errors.push(CompilerError::new(error_type, message, line, column));
    }

    /// Adiciona um warning (não impede a execução)
    pub fn add_warning(&mut self, message: &str, line: usize, column: usize) {
        self.warnings.push(CompilerError::new(ErrorType::Semantic, message, line, column));
    }

    /// Verifica se há erros armazenados
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Exibe todos os erros e warnings acumulados
    pub fn report(&self) {
        if self.has_errors() {
            println!("🛑 Erros encontrados:");
            for error in &self.errors {
                println!("{}", error);
            }
        } else {
            println!("✅ Nenhum erro encontrado.");
        }

        if !self.warnings.is_empty() {
            println!("⚠️ Warnings:");
            for warning in &self.warnings {
                println!("{}", warning);
            }
        }
    }

    /// Limpa todos os erros e warnings armazenados
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }
}
