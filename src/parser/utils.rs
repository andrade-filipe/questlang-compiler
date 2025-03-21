use crate::{
    error_handler::error_type::ErrorType,
    lexer::token::Token,
};

use super::parser::Parser;

impl<'a> Parser<'a> {

    pub fn skip_newlines(&mut self) {
        while self.check(Token::Newline) {
            self.advance();
        }
    }

    pub fn peek(&self) -> Token {
        self.tokens
        .get(self.pos)
        .map(|(t, _, _)| t.clone())
        .unwrap_or_default()
    }

    pub fn advance(&mut self) -> (Token, &'a str, usize) {
        let current =
         self.tokens
         .get(self.pos)
         .cloned()
         .unwrap_or((Token::Error, "", 0));
        
        self.pos += 1;
        current
    }

    pub fn check(&self, tok: Token) -> bool {
        self.peek() == tok
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn consume(&mut self, expected: Token, msg: &str) -> Option<()> {
        if self.check(expected.clone()) {
            self.advance();
            Some(())
        } else {
            self.error(msg);
            None
        }
    }

    pub fn error(&mut self, msg: &str) {
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

pub fn offset_to_line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, ch) in src.char_indices() {
        if i == offset { break; }
        if ch == '\n' { line += 1; col = 1; } else { col += 1; }
    }
    (line, col)
}