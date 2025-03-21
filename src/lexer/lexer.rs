use crate::lexer::token::Token;
use logos::Logos;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            lexer: Token::lexer(input),
        }
    }

    pub fn tokenize(&mut self) -> Vec<(Token, &str, usize)> {
        let source = self.lexer.source();
        self.lexer
            .clone()
            .spanned()
            .map(|(res, span)| {
                let token = res.unwrap_or(Token::Error);
                let slice = &source[span.clone()];
                (token, slice, span.start)
            })
            .collect()
    } 
}
