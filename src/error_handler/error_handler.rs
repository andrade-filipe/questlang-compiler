use crate::error_handler::error_type::ErrorType;
use crate::error_handler::compiler_error::CompilerError;

#[derive(Default)]
pub struct ErrorHandler {
    errors: Vec<CompilerError>,
    warnings: Vec<CompilerError>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn add_error(&mut self, error_type: ErrorType, message: &str, line: usize, column: usize) {
        self.errors.push(CompilerError::new(error_type, message, line, column));
    }

    pub fn add_warning(&mut self, message: &str, line: usize, column: usize) {
        self.warnings.push(CompilerError::new(ErrorType::Semantic, message, line, column));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

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

    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }
}
