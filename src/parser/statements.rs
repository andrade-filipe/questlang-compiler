use crate::{
    parser::ast::Stmt,
    lexer::token::Token,
};

use super::{ast::{ActionCommand, Command, MoveCommand}, parser::Parser};

impl<'a> Parser<'a> {

    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::MoveUp 
            | Token::MoveDown 
            | Token::MoveLeft 
            | Token::MoveRight
            | Token::Jump 
            | Token::Attack 
            | Token::Defend => {
                self.parse_command()
            }
            Token::If => 
                self.parse_if(),
            Token::While => 
                self.parse_while(),
            Token::For => 
                self.parse_for(),

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

    pub fn parse_if(&mut self) -> Option<Stmt> {
        self.consume(Token::If, "Expected 'if'")?;
        self.consume(Token::LParen, "Expected '(' after if")?;
        let condition = self.parse_expr()?;

        self.consume(Token::RParen, "Expected ')' after if condition")?;
        let then_branch = self.parse_block();

        self.consume(Token::Else, "Expected 'else' after if block")?;
        let else_branch = self.parse_block();

        Some(self.builder.new_if(condition, then_branch, else_branch))
    }

    pub fn parse_while(&mut self) -> Option<Stmt> {
        self.consume(Token::While, "Expected 'while'")?;
        self.consume(Token::LParen, "Expected '(' after while")?;
        let condition = self.parse_expr()?;

        self.consume(Token::RParen, "Expected ')' after while condition")?;
        let body = self.parse_block();

        Some(self.builder.new_while(condition, body))
    }

    pub fn parse_for(&mut self) -> Option<Stmt> {
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

    pub fn parse_block(&mut self) -> Vec<Stmt> {
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

    pub fn parse_command(&mut self) -> Option<Stmt> {
        let token = self.advance().0;
        
        let stmt = match token {
            Token::MoveUp => 
                self.builder.new_command(Command::Move(MoveCommand::MoveUp)),
            Token::MoveDown => 
                self.builder.new_command(Command::Move(MoveCommand::MoveDown)),
            Token::MoveLeft => 
                self.builder.new_command(Command::Move(MoveCommand::MoveLeft)),
            Token::MoveRight => 
                self.builder.new_command(Command::Move(MoveCommand::MoveRight)),
            Token::Jump => 
                self.builder.new_command(Command::Action(ActionCommand::Jump)),
            Token::Attack => 
                self.builder.new_command(Command::Action(ActionCommand::Attack)),
            Token::Defend => 
                self.builder.new_command(Command::Action(ActionCommand::Defend)),
            _ => {
                self.error("Invalid command");
                return None;
            }
        };
        Some(stmt)
    }
}