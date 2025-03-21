use crate::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};

#[derive(Default)]
pub struct ASTBuilder;

impl ASTBuilder {
    pub fn new() -> Self {
        ASTBuilder
    }

    /// Cria um Statement do tipo Command.
    pub fn new_command(&self, command: Command) -> Stmt {
        Stmt::Command(command)
    }

    /// Cria um Statement do tipo IfStmt.
    pub fn new_if(&self, condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt>) -> Stmt {
        Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        }
    }

    /// Cria um Statement do tipo WhileStmt.
    pub fn new_while(&self, condition: Expr, body: Vec<Stmt>) -> Stmt {
        Stmt::WhileStmt { condition, body }
    }

    /// Cria um Statement do tipo ForStmt.
    pub fn new_for(&self, init: Expr, condition: Expr, update: Expr, body: Vec<Stmt>) -> Stmt {
        Stmt::ForStmt {
            init,
            condition,
            update,
            body,
        }
    }

    /// Cria um Statement do tipo ExprStmt (quando a expressão é tratada como statement).
    pub fn new_expr_stmt(&self, expr: Expr) -> Stmt {
        Stmt::ExprStmt(expr)
    }

    /// Cria uma expressão de identificador.
    pub fn new_identifier(&self, name: &str) -> Expr {
        Expr::Identifier(name.to_string())
    }

    /// Cria uma expressão numérica.
    pub fn new_number(&self, value: i32) -> Expr {
        Expr::Number(value)
    }

    /// Cria uma expressão binária (operação).
    pub fn new_binop(&self, left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}
