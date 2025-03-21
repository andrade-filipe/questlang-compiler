use crate::{
    parser::ast::{Expr, BinOp},
    lexer::token::Token,
};

use super::parser::Parser;

impl<'a> Parser<'a> {

    pub fn parse_expr(&mut self) -> Option<Expr> {
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

    pub fn parse_primary(&mut self) -> Option<Expr> {
        match self.advance() {

            (Token::Identifier, text, _) => 
                Some(self.builder.new_identifier(text)),
                
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
}