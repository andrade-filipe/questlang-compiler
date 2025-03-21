use crate::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};

pub struct PrettyPrinter {
    output: String,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter {
            output: String::new(),
        }
    }

    pub fn print_stmts(&mut self, stmts: &[Stmt]) -> String {
        for stmt in stmts {
            self.print_stmt(stmt, 0);
        }
        self.output.clone()
    }

    fn print_stmt(&mut self, stmt: &Stmt, indent: usize) {
        let indent_str = "  ".repeat(indent);
        match stmt {
            Stmt::Command(cmd) => {
                self.output.push_str(&indent_str);
                self.output.push_str("Command: ");
                self.print_command(cmd);
                self.output.push('\n');
            },
            Stmt::IfStmt { condition, then_branch, else_branch } => {
                self.output.push_str(&indent_str);
                self.output.push_str("if (");
                self.print_expr(condition);
                self.output.push_str(") {\n");
                for s in then_branch {
                    self.print_stmt(s, indent + 1);
                }
                self.output.push_str(&indent_str);
                self.output.push_str("} else {\n");
                for s in else_branch {
                    self.print_stmt(s, indent + 1);
                }
                self.output.push_str(&indent_str);
                self.output.push_str("}\n");
            },
            Stmt::WhileStmt { condition, body } => {
                self.output.push_str(&indent_str);
                self.output.push_str("while (");
                self.print_expr(condition);
                self.output.push_str(") {\n");
                for s in body {
                    self.print_stmt(s, indent + 1);
                }
                self.output.push_str(&indent_str);
                self.output.push_str("}\n");
            },
            Stmt::ForStmt { init, condition, update, body } => {
                self.output.push_str(&indent_str);
                self.output.push_str("for ("); // Inicial maiúscula conforme teste
                self.print_expr(init);
                self.output.push_str("; ");
                self.print_expr(condition);
                self.output.push_str("; ");
                self.print_expr(update);
                self.output.push_str(") {\n");
                for s in body {
                    self.print_stmt(s, indent + 1);
                }
                self.output.push_str(&indent_str);
                self.output.push_str("}\n");
            },
            Stmt::ExprStmt(expr) => {
                self.output.push_str(&indent_str);
                self.output.push_str("Expr: ");
                self.print_expr(expr);
                self.output.push('\n');
            }
        }
    }

    fn print_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Move(m) => {
                let s = match m {
                    MoveCommand::MoveUp => "move_up",
                    MoveCommand::MoveDown => "move_down",
                    MoveCommand::MoveLeft => "move_left",
                    MoveCommand::MoveRight => "move_right",
                };
                self.output.push_str(s);
            },
            Command::Action(a) => {
                let s = match a {
                    ActionCommand::Jump => "jump",
                    ActionCommand::Attack => "attack",
                    ActionCommand::Defend => "defend",
                };
                self.output.push_str(s);
            },
        }
    }

    fn print_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(s) => self.output.push_str(s),
            Expr::Number(n) => self.output.push_str(&n.to_string()),
            Expr::BinaryOp { left, op, right } => {
                self.output.push('(');
                self.print_expr(left);
                let op_str = match op {
                    BinOp::Add => " + ",
                    BinOp::Sub => " - ",
                };
                self.output.push_str(op_str);
                self.print_expr(right);
                self.output.push(')');
            }
        }
    }
}