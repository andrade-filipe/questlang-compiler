use crate::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};

/// ASTBuilder é responsável por construir a AST
/// durante o processo de parsing.
#[derive(Default)]
pub struct ASTBuilder {
    /// Guarda os statements do nível atual.
    /// Em uma implementação mais sofisticada, podemos manter
    /// uma pilha de blocos.
    pub statements: Vec<Stmt>,
}

impl ASTBuilder {
    pub fn new() -> Self {
        ASTBuilder {
            statements: Vec::new(),
        }
    }
    
    pub fn push_expr(&mut self, expr: Expr) {
        self.statements.push(Stmt::ExprStmt(expr));
    }

    pub fn push_command(&mut self, command: Command) {
        self.statements.push(Stmt::Command(command));
    }

    pub fn push_if(&mut self, condition: Expr, then_branch: Vec<Stmt>, else_branch: Vec<Stmt>) {
        self.statements.push(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        });
    }

    pub fn push_while(&mut self, condition: Expr, body: Vec<Stmt>) {
        self.statements.push(Stmt::WhileStmt { condition, body });
    }

    pub fn push_for(&mut self, init: Expr, condition: Expr, update: Expr, body: Vec<Stmt>) {
        self.statements.push(Stmt::ForStmt {
            init,
            condition,
            update,
            body,
        });
    }

    // Métodos auxiliares para criar nós de Expressão

    pub fn make_identifier(&self, name: &str) -> Expr {
        Expr::Identifier(name.to_string())
    }

    pub fn make_number(&self, value: i32) -> Expr {
        Expr::Number(value)
    }

    pub fn make_binop(&self, left: Expr, op: BinOp, right: Expr) -> Expr {
        Expr::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    /// Método final: consome o builder e retorna a AST construída.
    pub fn build(self) -> Vec<Stmt> {
        self.statements
    }
}
