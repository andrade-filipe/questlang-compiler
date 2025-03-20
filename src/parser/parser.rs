use crate::{
    parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp},
    parser::ast_builder::ASTBuilder,
    error_handler::error_handler::ErrorHandler,
    error_handler::error_type::ErrorType,
    lexer::token::Token,
};

pub struct Parser<'a> {
    tokens: Vec<(Token, &'a str)>,
    pos: usize,
    errors: ErrorHandler,
    builder: ASTBuilder,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<(Token, &'a str)>) -> Self {
        Parser {
            tokens,
            pos: 0,
            errors: ErrorHandler::new(),
            builder: ASTBuilder::new(),
        }
    }

    /// Percorre todos os tokens e constrói a AST, retornando a AST e o ErrorHandler.
    pub fn parse(mut self) -> (Vec<Stmt>, ErrorHandler) {
        while !self.is_at_end() {
            self.parse_stmt();
        }
        let ast = self.builder.build();
        (ast, self.errors)
    }

    fn parse_stmt(&mut self) {
        match self.peek() {
            Token::MoveUp | Token::MoveDown | Token::MoveLeft | Token::MoveRight
            | Token::Jump | Token::Attack | Token::Defend => {
                self.parse_command();
            }
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            _ => {
                self.error("Expected statement");
                self.advance();
            }
        }
    }

    fn parse_command(&mut self) {
        let token = self.advance().0;
        let command = match token {
            Token::MoveUp => Command::Move(MoveCommand::MoveUp),
            Token::MoveDown => Command::Move(MoveCommand::MoveDown),
            Token::MoveLeft => Command::Move(MoveCommand::MoveLeft),
            Token::MoveRight => Command::Move(MoveCommand::MoveRight),
            Token::Jump => Command::Action(ActionCommand::Jump),
            Token::Attack => Command::Action(ActionCommand::Attack),
            Token::Defend => Command::Action(ActionCommand::Defend),
            _ => {
                self.error("Invalid command");
                return;
            }
        };
        self.builder.push_command(command);
    }

    fn parse_if(&mut self) {
        self.consume(Token::If, "Expected 'if'").unwrap_or(());
        self.consume(Token::LParen, "Expected '(' after if").unwrap_or(());
        let condition = match self.parse_expr() {
            Some(expr) => expr,
            None => return,
        };
        self.consume(Token::RParen, "Expected ')' after if condition").unwrap_or(());
        let then_branch = self.parse_block();
        self.consume(Token::Else, "Expected 'else' after if block").unwrap_or(());
        let else_branch = self.parse_block();
        self.builder.push_if(condition, then_branch, else_branch);
    }

    fn parse_while(&mut self) {
        self.consume(Token::While, "Expected 'while'").unwrap_or(());
        self.consume(Token::LParen, "Expected '(' after while").unwrap_or(());
        let condition = match self.parse_expr() {
            Some(expr) => expr,
            None => return,
        };
        self.consume(Token::RParen, "Expected ')' after while condition").unwrap_or(());
        let body = self.parse_block();
        self.builder.push_while(condition, body);
    }

    fn parse_for(&mut self) {
        self.consume(Token::For, "Expected 'for'").unwrap_or(());
        self.consume(Token::LParen, "Expected '(' after for").unwrap_or(());
        let init = match self.parse_expr() {
            Some(expr) => expr,
            None => return,
        };
        self.consume(Token::Semicolon, "Expected ';' after initialization").unwrap_or(());
        let condition = match self.parse_expr() {
            Some(expr) => expr,
            None => return,
        };
        self.consume(Token::Semicolon, "Expected ';' after condition").unwrap_or(());
        let update = match self.parse_expr() {
            Some(expr) => expr,
            None => return,
        };
        self.consume(Token::RParen, "Expected ')' after for clauses").unwrap_or(());
        let body = self.parse_block();
        self.builder.push_for(init, condition, update, body);
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut block = Vec::new();
        if self.consume(Token::LBrace, "Expected '{' to start block").is_none() {
            return block;
        }
        while !self.check(Token::RBrace) && !self.is_at_end() {
            let pos_before = self.pos;
            self.parse_stmt();
            if self.pos == pos_before {
                self.advance(); // Evita loop infinito
            }
        }
        self.consume(Token::RBrace, "Expected '}' to close block");
        // Método simplificado para extrair as statements do bloco.
        self.extract_last_block()
    }

    /// Método simplificado para extrair o bloco atual.
    /// Em uma implementação robusta, utilizaríamos uma pilha de blocos.
    fn extract_last_block(&mut self) -> Vec<Stmt> {
        std::mem::take(&mut self.builder.statements)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance().0 {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            expr = self.builder.make_binop(expr, op, right);
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance() {
            (Token::Identifier, text) => Some(self.builder.make_identifier(text)),
            (Token::Number, text) => {
                let value: i32 = text.parse().unwrap_or_else(|_| {
                    self.error("Invalid number literal");
                    0
                });
                Some(self.builder.make_number(value))
            },
            (tok, text) => {
                self.error(&format!("Unexpected token '{:?}' in expression (found '{}')", tok, text));
                None
            }
        }
    }

    // Métodos utilitários

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).map(|(t, _)| t.clone()).unwrap_or_default()
    }

    fn advance(&mut self) -> (Token, &'a str) {
        let current = self.tokens.get(self.pos).cloned().unwrap_or((Token::Error, ""));
        self.pos += 1;
        current
    }

    fn check(&self, tok: Token) -> bool {
        self.peek() == tok
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn consume(&mut self, expected: Token, msg: &str) -> Option<()> {
        if self.check(expected.clone()) {
            self.advance();
            Some(())
        } else {
            self.error(msg);
            None
        }
    }

    fn error(&mut self, msg: &str) {
        let (token, text) = self.tokens.get(self.pos).cloned().unwrap_or((Token::Error, ""));
        self.errors.add_error(ErrorType::Syntactic, &format!("{} (found '{}')", msg, text), 0, 0);
    }
}