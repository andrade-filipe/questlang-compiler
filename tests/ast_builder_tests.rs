use questlang::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};
use questlang::parser::ast_builder::ASTBuilder;

#[test]
fn test_new_command() {
    let builder = ASTBuilder::new();
    let stmt = builder.new_command(Command::Move(MoveCommand::MoveUp));
    match stmt {
        Stmt::Command(cmd) => match cmd {
            Command::Move(mv) => {
                assert_eq!(mv, MoveCommand::MoveUp, "Deve ser move_up");
            },
            _ => panic!("Esperado comando de movimento"),
        },
        _ => panic!("Esperado statement do tipo Command"),
    }
}

#[test]
fn test_new_if() {
    let builder = ASTBuilder::new();
    let condition = builder.new_identifier("hero");
    let then_branch = vec![builder.new_command(Command::Move(MoveCommand::MoveLeft))];
    let else_branch = vec![builder.new_command(Command::Move(MoveCommand::MoveRight))];
    let stmt = builder.new_if(condition, then_branch.clone(), else_branch.clone());
    
    match stmt {
        Stmt::IfStmt { condition: cond, then_branch: tb, else_branch: eb } => {
            match cond {
                Expr::Identifier(s) => assert_eq!(s, "hero", "Condição deve ser 'hero'"),
                _ => panic!("Condição do if deve ser um identificador"),
            }
            assert_eq!(tb, then_branch, "Then branch incorreto");
            assert_eq!(eb, else_branch, "Else branch incorreto");
        },
        _ => panic!("Esperado statement do tipo IfStmt"),
    }
}

#[test]
fn test_new_while() {
    let builder = ASTBuilder::new();
    let condition = builder.new_identifier("enemy");
    let body = vec![builder.new_command(Command::Action(ActionCommand::Jump))];
    let stmt = builder.new_while(condition, body.clone());
    
    match stmt {
        Stmt::WhileStmt { condition: cond, body: b } => {
            match cond {
                Expr::Identifier(s) => assert_eq!(s, "enemy", "Condição deve ser 'enemy'"),
                _ => panic!("Condição do while deve ser um identificador"),
            }
            assert_eq!(b, body, "Corpo do while incorreto");
        },
        _ => panic!("Esperado statement do tipo WhileStmt"),
    }
}

#[test]
fn test_new_for() {
    let builder = ASTBuilder::new();
    let init = builder.new_identifier("hero");
    let condition = builder.new_identifier("enemy");
    let update = builder.new_identifier("treasure");
    let body = vec![builder.new_command(Command::Action(ActionCommand::Defend))];
    let stmt = builder.new_for(init, condition, update, body.clone());
    
    match stmt {
        Stmt::ForStmt { init: i, condition: cond, update: u, body: b } => {
            match i {
                Expr::Identifier(s) => assert_eq!(s, "hero", "Init deve ser 'hero'"),
                _ => panic!("Esperado identificador para init do for"),
            }
            match cond {
                Expr::Identifier(s) => assert_eq!(s, "enemy", "Condition deve ser 'enemy'"),
                _ => panic!("Esperado identificador para condition do for"),
            }
            match u {
                Expr::Identifier(s) => assert_eq!(s, "treasure", "Update deve ser 'treasure'"),
                _ => panic!("Esperado identificador para update do for"),
            }
            assert_eq!(b, body, "Corpo do for incorreto");
        },
        _ => panic!("Esperado statement do tipo ForStmt"),
    }
}

#[test]
fn test_new_expr_stmt() {
    let builder = ASTBuilder::new();
    let left = builder.new_number(10);
    let right = builder.new_number(3);
    let bin_expr = builder.new_binop(left, BinOp::Sub, right);
    let stmt = builder.new_expr_stmt(bin_expr);
    
    match stmt {
        Stmt::ExprStmt(expr) => match expr {
            Expr::BinaryOp { left: l, op, right: r } => {
                match *l {
                    Expr::Number(n) => assert_eq!(n, 10, "Esquerda deve ser 10"),
                    _ => panic!("Esperado número na esquerda"),
                }
                match op {
                    BinOp::Sub => {},
                    _ => panic!("Esperado operador de subtração"),
                }
                match *r {
                    Expr::Number(n) => assert_eq!(n, 3, "Direita deve ser 3"),
                    _ => panic!("Esperado número na direita"),
                }
            },
            _ => panic!("Esperado nó binário na expressão"),
        },
        _ => panic!("Esperado statement do tipo ExprStmt"),
    }
}

#[test]
fn test_new_identifier() {
    let builder = ASTBuilder::new();
    let expr = builder.new_identifier("hero");
    match expr {
        Expr::Identifier(s) => assert_eq!(s, "hero", "Deve criar identificador 'hero'"),
        _ => panic!("Esperado nó de identificador"),
    }
}

#[test]
fn test_new_number() {
    let builder = ASTBuilder::new();
    let expr = builder.new_number(42);
    match expr {
        Expr::Number(n) => assert_eq!(n, 42, "Deve criar número 42"),
        _ => panic!("Esperado nó de número"),
    }
}

#[test]
fn test_new_binop() {
    let builder = ASTBuilder::new();
    let left = builder.new_number(5);
    let right = builder.new_number(3);
    let expr = builder.new_binop(left, BinOp::Add, right);
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
        _ => panic!("Esperado nó de expressão binária"),
    }
}