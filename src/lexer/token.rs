use logos::Logos;

#[derive(Logos, Debug, PartialEq, Default, Clone)]
pub enum Token {
    // Palavras-chave
    #[token("move_up")]
    MoveUp,
    #[token("move_down")]
    MoveDown,
    #[token("move_left")]
    MoveLeft,
    #[token("move_right")]
    MoveRight,
    
    #[token("jump")]
    Jump,
    #[token("attack")]
    Attack,
    #[token("defend")]
    Defend,

    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,

    // Operadores
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,

    // Parênteses e chaves
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(";")]
    Semicolon,

    // Operadores lógicos
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    LogicalNot,

    // Identificadores e números
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 2)]
    Identifier,

    #[regex(r"[0-9]+", priority = 1)]
    Number,
    
    #[token("\n")]
    Newline,

    // Comentários (ignorar)
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    // Espaços em branco (ignorar)
    #[regex(r"[ \t\f\r]+", logos::skip)]
    Whitespace,

    #[default] // Token inválido (usando Default)
    #[regex(r"[0-9]+[a-zA-Z][a-zA-Z0-9_]*", priority = 3)] // variaveis que começam com numeros, exemplo: 123abc
    Error,
}