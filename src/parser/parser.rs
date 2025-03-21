use crate::{
    parser::ast::Stmt,
    parser::ast_builder::ASTBuilder,
    error_handler::error_handler::ErrorHandler,
    lexer::token::Token,
};

pub struct Parser<'a> {
    pub tokens: Vec<(Token, &'a str, usize)>,
    pub source: &'a str,
    pub pos: usize,
    pub errors: ErrorHandler,
    pub builder: ASTBuilder,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<(Token, &'a str, usize)>, source: &'a str) -> Self {

        Parser {
            tokens,
            source,
            pos: 0,
            errors: ErrorHandler::new(),
            builder: ASTBuilder::new(),
        }
    }

    pub fn parse(mut self) -> (Vec<Stmt>, ErrorHandler) {
    
        let mut statements = Vec::new();

        while !self.is_at_end() {
            
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }
        (statements, std::mem::take(&mut self.errors))
    }
}