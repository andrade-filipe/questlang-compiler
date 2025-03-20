use questlang::lexer::lexer::Lexer;
use questlang::lexer::token::Token;

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize().into_iter().map(|(token, _)| token).collect()
}

#[test]
fn test_keywords() {
    let tokens = lex("move_up move_down attack if else while for");
    assert_eq!(
        tokens,
        vec![
            Token::MoveUp,
            Token::MoveDown,
            Token::Attack,
            Token::If,
            Token::Else,
            Token::While,
            Token::For,
        ]
    );
}

#[test]
fn test_operators() {
    let tokens = lex("+ - * / && || !");
    assert_eq!(
        tokens,
        vec![
            Token::Plus,
            Token::Minus,
            Token::Mul,
            Token::Div,
            Token::LogicalAnd,
            Token::LogicalOr,
            Token::LogicalNot,
        ]
    );
}

#[test]
fn test_identifiers_numbers() {
    let tokens = lex("hero enemy 123 456");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier,
            Token::Identifier,
            Token::Number,
            Token::Number,
        ]
    );
}

#[test]
fn test_symbols_and_syntax() {
    let tokens = lex("( ) { } ;");
    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_invalid_tokens() {
    let tokens = lex("@ $ % ^");
    assert_eq!(
        tokens,
        vec![
            Token::Error,
            Token::Error,
            Token::Error,
            Token::Error,
        ]
    );
}

/// ✅ Testa um código realista com várias instruções combinadas
#[test]
fn test_mixed_tokens() {
    let tokens = lex("move_up; attack; if (hero) { move_left; } else { move_right; }");
    assert_eq!(
        tokens,
        vec![
            Token::MoveUp,
            Token::Semicolon,
            Token::Attack,
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Identifier,
            Token::RParen,
            Token::LBrace,
            Token::MoveLeft,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::MoveRight,
            Token::Semicolon,
            Token::RBrace,
        ]
    );
}

/// ✅ Testa espaços em branco e quebras de linha
#[test]
fn test_whitespace_and_newlines() {
    let tokens = lex("
        move_up   
        move_down

        attack
    ");
    assert_eq!(
        tokens,
        vec![
            Token::MoveUp,
            Token::MoveDown,
            Token::Attack,
        ]
    );
}

/// ✅ Testa se o Lexer ignora corretamente comentários (`//`)
#[test]
fn test_comments() {
    let tokens = lex("
        move_up; // Isso é um comentário
        attack;  // Outro comentário
    ");
    assert_eq!(
        tokens,
        vec![
            Token::MoveUp,
            Token::Semicolon,
            Token::Attack,
            Token::Semicolon,
        ]
    );
}

/// ✅ Testa expressões matemáticas mais complexas
#[test]
fn test_complex_expressions() {
    let tokens = lex("hero + 10 * enemy / (5 - 2);");
    assert_eq!(
        tokens,
        vec![
            Token::Identifier,
            Token::Plus,
            Token::Number,
            Token::Mul,
            Token::Identifier,
            Token::Div,
            Token::LParen,
            Token::Number,
            Token::Minus,
            Token::Number,
            Token::RParen,
            Token::Semicolon,
        ]
    );
}

/// ✅ Testa código misturado com tokens inválidos
#[test]
fn test_mixed_with_invalid_tokens() {
    let tokens = lex("move_up @ attack # 123abc");
    assert_eq!(
        tokens,
        vec![
            Token::MoveUp,
            Token::Error, // '@' inválido
            Token::Attack,
            Token::Error, // '#' inválido
            Token::Error, // '123abc' inválido (número seguido de letras)
        ]
    );
}


