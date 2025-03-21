use questlang::lexer::lexer::Lexer;
use questlang::lexer::token::Token;

fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize().into_iter().map(|(token, _, _)| token).collect()
}

#[test]
fn test_keywords() {
    let input = "move_up move_down attack if else while for";
    let tokens = lex(input);
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
        ],
        "Falha ao tokenizar palavras-chave: {}",
        input
    );
}

#[test]
fn test_operators() {
    let input = "+ - * / && || !";
    let tokens = lex(input);
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
        ],
        "Falha ao tokenizar operadores: {}",
        input
    );
}

#[test]
fn test_identifiers_numbers() {
    let input = "hero enemy 123 456";
    let tokens = lex(input);
    assert_eq!(
        tokens,
        vec![
            Token::Identifier,
            Token::Identifier,
            Token::Number,
            Token::Number,
        ],
        "Falha ao tokenizar identificadores e números: {}",
        input
    );
}

#[test]
fn test_symbols() {
    // Testa os símbolos: parênteses e chaves.
    let input = "( ) { }";
    let tokens = lex(input);
    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
        ],
        "Falha ao tokenizar símbolos: {}",
        input
    );
}

#[test]
fn test_invalid_tokens() {
    let input = "@ $ % ^";
    let tokens = lex(input);
    assert_eq!(
        tokens,
        vec![
            Token::Error,
            Token::Error,
            Token::Error,
            Token::Error,
        ],
        "Tokens inválidos não foram marcados corretamente: {}",
        input
    );
}

#[test]
fn test_mixed_tokens() {
    // Testa um código realista com várias instruções, sem semicolons.
    let input = "move_up attack if (hero) { move_left } else { move_right }";
    let tokens = lex(input);
    let expected = vec![
        Token::MoveUp,
        Token::Attack,
        Token::If,
        Token::LParen,
        Token::Identifier,
        Token::RParen,
        Token::LBrace,
        Token::MoveLeft,
        Token::RBrace,
        Token::Else,
        Token::LBrace,
        Token::MoveRight,
        Token::RBrace,
    ];
    assert_eq!(
        tokens, expected,
        "Falha ao tokenizar código misto: {}",
        input
    );
}

#[test]
fn test_whitespace_and_newlines() {
    let input = "\n  move_up  \n move_down \n\n attack \n";
    let tokens = lex(input);
    // Supondo que o lexer emita Token::Newline onde houver '\n'
    let expected = vec![
        Token::Newline,
        Token::MoveUp,
        Token::Newline,
        Token::MoveDown,
        Token::Newline,
        Token::Newline,
        Token::Attack,
        Token::Newline,
    ];
    assert_eq!(
        tokens, expected,
        "Falha ao tokenizar espaços e quebras de linha: {}",
        input
    );
}

#[test]
fn test_comments() {
    let input = "\n move_up // este é um comentário\n attack // outro comentário\n";
    let tokens = lex(input);
    // Comentários devem ser ignorados, mas quebras de linha devem ser mantidas.
    let expected = vec![
        Token::Newline,
        Token::MoveUp,
        Token::Newline,
        Token::Attack,
        Token::Newline,
    ];
    assert_eq!(
        tokens, expected,
        "Falha ao ignorar comentários: {}",
        input
    );
}

#[test]
fn test_complex_expressions() {
    let input = "hero + 10 * enemy / (5 - 2)";
    let tokens = lex(input);
    let expected = vec![
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
    ];
    assert_eq!(
        tokens, expected,
        "Falha ao tokenizar expressões complexas: {}",
        input
    );
}

#[test]
fn test_mixed_with_invalid_tokens() {
    let input = "move_up @ attack # 123abc";
    let tokens = lex(input);
    let expected = vec![
        Token::MoveUp,
        Token::Error, // '@'
        Token::Attack,
        Token::Error, // '#'
        Token::Error, // '123abc' inválido
    ];
    assert_eq!(
        tokens, expected,
        "Falha ao tokenizar código misto com tokens inválidos: {}",
        input
    );
}