use crate::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};

#[derive(Default)]
pub struct ASTBuilder;

impl ASTBuilder {
    pub fn new() -> Self {
        ASTBuilder
    }

    pub fn new_command(&self, command: Command) -> Stmt {
        Stmt::Command(command)
    }

    pub fn new_if(&self, condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt>) -> Stmt {
        Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn new_while(&self, condition: Expr, body: Vec<Stmt>) -> Stmt {
        Stmt::WhileStmt { condition, body }
    }

    pub fn new_for(&self, init: Expr, condition: Expr, update: Expr, body: Vec<Stmt>) -> Stmt {
        Stmt::ForStmt {
            init,
            condition,
            update,
            body,
        }
    }

    pub fn new_expr_stmt(&self, expr: Expr) -> Stmt {
        Stmt::ExprStmt(expr)
    }

    pub fn new_identifier(&self, name: &str) -> Expr {
        Expr::Identifier(name.to_string())
    }

    pub fn new_number(&self, value: i32) -> Expr {
        Expr::Number(value)
    }
    
    pub fn new_binop(&self, left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}
