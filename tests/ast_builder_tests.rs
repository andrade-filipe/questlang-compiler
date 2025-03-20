use questlang::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};
use questlang::parser::ast_builder::ASTBuilder;

#[test]
fn test_push_command() {
    let mut builder = ASTBuilder::new();
    builder.push_command(Command::Move(MoveCommand::MoveUp));
    let ast = builder.build();
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
fn test_push_if() {
    let mut builder = ASTBuilder::new();
    let condition = builder.make_identifier("hero");
    let then_branch = vec![Stmt::Command(Command::Move(MoveCommand::MoveLeft))];
    let else_branch = vec![Stmt::Command(Command::Move(MoveCommand::MoveRight))];
    builder.push_if(condition, then_branch.clone(), else_branch.clone());
    let ast = builder.build();
    assert_eq!(ast.len(), 1, "Deve haver 1 statement");
    match &ast[0] {
        Stmt::IfStmt { condition: cond, then_branch: tb, else_branch: eb } => {
            match cond {
                Expr::Identifier(s) => assert_eq!(s, "hero", "Condição deve ser 'hero'"),
                _ => panic!("Esperado identificador na condição"),
            }
            assert_eq!(tb, &then_branch, "Then branch incorreto");
            assert_eq!(eb, &else_branch, "Else branch incorreto");
        },
        _ => panic!("Esperado statement do tipo IfStmt"),
    }
}

#[test]
fn test_make_binop() {
    let builder = ASTBuilder::new();
    let left = builder.make_number(5);
    let right = builder.make_number(3);
    let expr = builder.make_binop(left, BinOp::Add, right);
    match expr {
        Expr::BinaryOp { left: l, op, right: r } => {
            match *l {
                Expr::Number(n) => assert_eq!(n, 5, "Esquerda deve ser 5"),
                _ => panic!("Esperado número na esquerda"),
            }
            match op {
                BinOp::Add => {},
                _ => panic!("Esperado operador de adição"),
            }
            match *r {
                Expr::Number(n) => assert_eq!(n, 3, "Direita deve ser 3"),
                _ => panic!("Esperado número na direita"),
            }
        },
        _ => panic!("Esperado expressão binária"),
    }
}
