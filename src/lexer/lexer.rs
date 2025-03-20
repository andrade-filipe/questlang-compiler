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

    pub fn tokenize(&mut self) -> Vec<(Token, &str)> {
        let lexer_clone = self.lexer.clone();
        lexer_clone
            .spanned()
            .map(|(token_result, span)| {
                let token = token_result.unwrap_or(Token::Error); // Garante que tokens inválidos sejam tratados corretamente
                (token, &self.lexer.source()[span])
            })
            .collect()
    }    
}
