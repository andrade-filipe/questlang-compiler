use crate::{
    parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp},
    error_handler::error_handler::ErrorHandler,
    error_handler::error_type::ErrorType,
    lexer::token::Token,
};

pub struct Parser<'a> {
    tokens: Vec<(Token, &'a str)>,
    pos: usize,
    errors: ErrorHandler,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<(Token, &'a str)>) -> Self {
        Parser {
            tokens,
            pos: 0,
            errors: ErrorHandler::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::MoveUp | Token::MoveDown | Token::MoveLeft | Token::MoveRight
            | Token::Jump | Token::Attack | Token::Defend => {
                Some(Stmt::Command(self.parse_command()))
            }
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::For => self.parse_for(),
            _ => {
                self.error("Expected statement");
                None
            }
        }
    }

    fn parse_command(&mut self) -> Command {
        let token = self.advance().0;
        match token {
            Token::MoveUp => Command::Move(MoveCommand::MoveUp),
            Token::MoveDown => Command::Move(MoveCommand::MoveDown),
            Token::MoveLeft => Command::Move(MoveCommand::MoveLeft),
            Token::MoveRight => Command::Move(MoveCommand::MoveRight),
            Token::Jump => Command::Action(ActionCommand::Jump),
            Token::Attack => Command::Action(ActionCommand::Attack),
            Token::Defend => Command::Action(ActionCommand::Defend),
            _ => unreachable!(),
        }
    }

    fn parse_if(&mut self) -> Option<Stmt> {
        self.consume(Token::If, "Expected 'if'")?;
        self.consume(Token::LParen, "Expected '(' after if")?;
        let condition = self.parse_expr()?;
        self.consume(Token::RParen, "Expected ')' after if condition")?;
        let then_branch = self.parse_block()?;
        self.consume(Token::Else, "Expected 'else' after if block")?;
        let else_branch = self.parse_block()?;
        Some(Stmt::IfStmt { condition, then_branch, else_branch })
    }

    fn parse_while(&mut self) -> Option<Stmt> {
        self.consume(Token::While, "Expected 'while'")?;
        self.consume(Token::LParen, "Expected '(' after while")?;
        let condition = self.parse_expr()?;
        self.consume(Token::RParen, "Expected ')' after while condition")?;
        let body = self.parse_block()?;
        Some(Stmt::WhileStmt { condition, body })
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
        let body = self.parse_block()?;
        Some(Stmt::ForStmt { init, condition, update, body })
    }

    fn parse_block(&mut self) -> Option<Vec<Stmt>> {
        self.consume(Token::LBrace, "Expected '{' to start block")?;
        let mut statements = Vec::new();
        while !self.check(Token::RBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }
        self.consume(Token::RBrace, "Expected '}' to close block")?;
        Some(statements)
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
            expr = Expr::BinaryOp { left: Box::new(expr), op, right: Box::new(right) };
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance().0 {
            Token::Identifier => Some(Expr::Identifier(self.previous_text().to_string())),
            Token::Number => {
                let value: i32 = self.previous_text().parse().unwrap_or_else(|_| {
                    self.error("Invalid number literal");
                    0
                });
                Some(Expr::Number(value))
            }
            tok => {
                self.error(&format!("Unexpected token '{:?}' in expression", tok));
                None
            }
        }
    }

    // Utility methods

    fn peek(&self) -> Token { self.tokens.get(self.pos).map(|(t, _)| t.clone()).unwrap_or_default() }
    fn advance(&mut self) -> (Token, &'a str) {
        let current = self.tokens.get(self.pos).cloned().unwrap_or((Token::Error, ""));
        self.pos += 1;
        current
    }
    fn previous_text(&self) -> &str { self.tokens.get(self.pos - 1).map(|(_, txt)| *txt).unwrap_or("") }
    fn check(&self, tok: Token) -> bool { self.peek() == tok }
    fn is_at_end(&self) -> bool { self.pos >= self.tokens.len() }
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
        let (tok, text) = self.tokens.get(self.pos).cloned().unwrap_or((Token::Error, ""));
        self.errors.add_error(ErrorType::Syntactic, &format!("{} (found '{}')", msg, text), 0, 0);
    }

    pub fn into_errors(self) -> ErrorHandler { self.errors }
}
