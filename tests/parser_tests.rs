use questlang::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr};
use questlang::error_handler::error_handler::ErrorHandler;
use questlang::lexer::lexer::Lexer;
use questlang::parser::parser::Parser;

/// Função auxiliar que processa a fonte e retorna a AST e o ErrorHandler.
fn parse_source(source: &str) -> (Vec<Stmt>, ErrorHandler) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let (ast, errors) = Parser::new(tokens, source).parse();
    (ast, errors)
}

#[test]
fn test_command_statement() {
    // Testa um comando simples (move_up)
    let source = "move_up";
    let (ast, errors) = parse_source(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos");
    assert_eq!(ast.len(), 1, "Deve haver 1 statement");
    match &ast[0] {
        Stmt::Command(cmd) => match cmd {
            Command::Move(mv) => assert_eq!(mv, &MoveCommand::MoveUp, "Deve ser move_up"),
            _ => panic!("Esperado comando de movimento"),
        },
        _ => panic!("Esperado statement do tipo Command"),
    }
}

#[test]
fn test_if_statement() {
    // Testa a estrutura if: if (hero) { move_left } else { move_right }
    let source = "if (hero) { move_left } else { move_right }";
    let (ast, errors) = parse_source(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos");
    assert_eq!(ast.len(), 1, "Deve haver 1 statement");
    match &ast[0] {
        Stmt::IfStmt { condition, then_branch, else_branch } => {
            // Condição deve ser um identificador "hero"
            match condition {
                Expr::Identifier(s) => assert_eq!(s, "hero", "Condição deve ser 'hero'"),
                _ => panic!("Condição do if deve ser um identificador"),
            }
            // then_branch: espera um comando: move_left
            assert_eq!(then_branch.len(), 1, "Then branch deve ter 1 statement");
            match &then_branch[0] {
                Stmt::Command(cmd) => match cmd {
                    Command::Move(mv) => assert_eq!(mv, &MoveCommand::MoveLeft, "Then branch deve ser move_left"),
                    _ => panic!("Then branch deve ser um comando de movimento"),
                },
                _ => panic!("Then branch deve ser um statement do tipo Command"),
            }
            // else_branch: espera um comando: move_right
            assert_eq!(else_branch.len(), 1, "Else branch deve ter 1 statement");
            match &else_branch[0] {
                Stmt::Command(cmd) => match cmd {
                    Command::Move(mv) => assert_eq!(mv, &MoveCommand::MoveRight, "Else branch deve ser move_right"),
                    _ => panic!("Else branch deve ser um comando de movimento"),
                },
                _ => panic!("Else branch deve ser um statement do tipo Command"),
            }
        }
        _ => panic!("Esperado statement do tipo IfStmt"),
    }
}

#[test]
fn test_while_statement() {
    // Testa a estrutura while: while (enemy) { jump }
    let source = "while (enemy) { jump }";
    let (ast, errors) = parse_source(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos");
    assert_eq!(ast.len(), 1, "Deve haver 1 statement");
    match &ast[0] {
        Stmt::WhileStmt { condition, body } => {
            // Condição deve ser "enemy"
            match condition {
                Expr::Identifier(s) => assert_eq!(s, "enemy", "Condição deve ser 'enemy'"),
                _ => panic!("Condição do while deve ser um identificador"),
            }
            // Body: espera comando: jump
            assert_eq!(body.len(), 1, "Body deve ter 1 statement");
            match &body[0] {
                Stmt::Command(cmd) => match cmd {
                    Command::Action(act) => assert_eq!(act, &ActionCommand::Jump, "Body deve ser jump"),
                    _ => panic!("Body do while deve ser um comando de ação"),
                },
                _ => panic!("Body do while deve ser um statement do tipo Command"),
            }
        }
        _ => panic!("Esperado statement do tipo WhileStmt"),
    }
}

#[test]
fn test_for_statement() {
    // Testa a estrutura for: for (hero; enemy; treasure) { defend }
    let source = "for (hero; enemy; treasure) { defend }";
    let (ast, errors) = parse_source(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos");
    assert_eq!(ast.len(), 1, "Deve haver 1 statement");
    match &ast[0] {
        Stmt::ForStmt { init, condition, update, body } => {
            // Verifica init, condition e update
            match init {
                Expr::Identifier(s) => assert_eq!(s, "hero", "Init deve ser 'hero'"),
                _ => panic!("Init do for deve ser um identificador"),
            }
            match condition {
                Expr::Identifier(s) => assert_eq!(s, "enemy", "Condition deve ser 'enemy'"),
                _ => panic!("Condition do for deve ser um identificador"),
            }
            match update {
                Expr::Identifier(s) => assert_eq!(s, "treasure", "Update deve ser 'treasure'"),
                _ => panic!("Update do for deve ser um identificador"),
            }
            // Body: espera comando: defend
            assert_eq!(body.len(), 1, "Body deve ter 1 statement");
            match &body[0] {
                Stmt::Command(cmd) => match cmd {
                    Command::Action(act) => assert_eq!(act, &ActionCommand::Defend, "Body deve ser defend"),
                    _ => panic!("Body do for deve ser um comando de ação"),
                },
                _ => panic!("Body do for deve ser um statement do tipo Command"),
            }
        }
        _ => panic!("Esperado statement do tipo ForStmt"),
    }
}

#[test]
fn test_multiple_statements() {
    let source = r#"
        move_up
        if (hero) { move_left } else { move_right }
        while (enemy) { jump }
    "#;
    let (ast, errors) = parse_source(source);
    assert!(!errors.has_errors(), "Não devem ocorrer erros sintáticos");
    assert_eq!(ast.len(), 3, "Deve haver 3 statements");
}
