use crate::{
    parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp},
    parser::ast_builder::ASTBuilder,
    error_handler::{error_handler::ErrorHandler, error_type::ErrorType},
    lexer::token::Token,
};

pub struct Parser<'a> {
    tokens: Vec<(Token, &'a str, usize)>,
    source: &'a str,
    pos: usize,
    errors: ErrorHandler,
    builder: ASTBuilder,
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
    
    fn skip_newlines(&mut self) {
        while self.check(Token::Newline) {
            self.advance();
        }
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::MoveUp | Token::MoveDown | Token::MoveLeft | Token::MoveRight
            | Token::Jump | Token::Attack | Token::Defend => {
                self.parse_command()
            }
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            _ => {
                if let Some(expr) = self.parse_expr() {
                    Some(self.builder.new_expr_stmt(expr))
                } else {
                    self.error("Expected statement");
                    self.advance();
                    None
                }
            }
        }
    }

    fn parse_command(&mut self) -> Option<Stmt> {
        let token = self.advance().0;
        let stmt = match token {
            Token::MoveUp => self.builder.new_command(Command::Move(MoveCommand::MoveUp)),
            Token::MoveDown => self.builder.new_command(Command::Move(MoveCommand::MoveDown)),
            Token::MoveLeft => self.builder.new_command(Command::Move(MoveCommand::MoveLeft)),
            Token::MoveRight => self.builder.new_command(Command::Move(MoveCommand::MoveRight)),
            Token::Jump => self.builder.new_command(Command::Action(ActionCommand::Jump)),
            Token::Attack => self.builder.new_command(Command::Action(ActionCommand::Attack)),
            Token::Defend => self.builder.new_command(Command::Action(ActionCommand::Defend)),
            _ => {
                self.error("Invalid command");
                return None;
            }
        };
        Some(stmt)
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.consume(Token::If, "Expected 'if'")?;
        self.consume(Token::LParen, "Expected '(' after if")?;
        let condition = self.parse_expr()?;
        self.consume(Token::RParen, "Expected ')' after if condition")?;
        let then_branch = self.parse_block();
        self.consume(Token::Else, "Expected 'else' after if block")?;
        let else_branch = self.parse_block();
        Some(self.builder.new_if(condition, then_branch, else_branch))
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.consume(Token::While, "Expected 'while'")?;
        self.consume(Token::LParen, "Expected '(' after while")?;
        let condition = self.parse_expr()?;
        self.consume(Token::RParen, "Expected ')' after while condition")?;
        let body = self.parse_block();
        Some(self.builder.new_while(condition, body))
    }

    fn parse_for(&mut self) -> Option<Stmt> {
        self.consume(Token::For, "Expected 'for'")?;
        self.consume(Token::LParen, "Expected '(' after for")?;
        let init = self.parse_expr()?;
        self.consume(Token::Semicolon, "Expected ';' after initialization")?;
        let condition = self.parse_expr()?;
        self.consume(Token::Semicolon, "Expected ';' after condition")?;
        let update = self.parse_expr()?;
        self.consume(Token::RParen, "Expected ')' after for clauses")?;
        let body = self.parse_block();
        Some(self.builder.new_for(init, condition, update, body))
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        if self.consume(Token::LBrace, "Expected '{' to start block").is_none() {
            return statements;
        }
        while !self.check(Token::RBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(Token::RBrace) { break; }
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }
        self.consume(Token::RBrace, "Expected '}' to close block");
        statements
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
            expr = self.builder.new_binop(expr, op, right);
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance() {
            (Token::Identifier, text, _) => Some(self.builder.new_identifier(text)),
            (Token::Number, text, _) => {
                let value: i32 = text.parse().unwrap_or_else(|_| {
                    self.error("Invalid number literal");
                    0
                });
                Some(self.builder.new_number(value))
            },
            (tok, text, _) => {
                self.error(&format!("Unexpected token '{:?}' in expression (found '{}')", tok, text));
                None
            }
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).map(|(t, _, _)| t.clone()).unwrap_or_default()
    }

    fn advance(&mut self) -> (Token, &'a str, usize) {
        let current = self.tokens.get(self.pos).cloned().unwrap_or((Token::Error, "", 0));
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
        let (_token, text, offset) = self
            .tokens
            .get(self.pos)
            .cloned()
            .unwrap_or((Token::Error, "", 0));
        let (line, col) = offset_to_line_col(self.source, offset);
        self.errors
            .add_error(ErrorType::Syntactic, &format!("{} (found '{}')", msg, text), line, col);
    }
}

fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in src.char_indices() {
        if i == offset { break; }
        if ch == '\n' { line += 1; col = 1; } else { col += 1; }
    }
    (line, col)
}