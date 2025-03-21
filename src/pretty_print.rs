use crate::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};

pub struct PrettyPrinter {
    lines: Vec<String>,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        PrettyPrinter { lines: Vec::new() }
    }

    pub fn print_stmts(&mut self, stmts: &[Stmt]) -> String {
        for stmt in stmts {
            self.print_stmt(stmt, 0);
        }
        let mut output = self.lines.join("\n");
        if !output.is_empty() {
            output.push('\n');
        }
        output
    }

    fn print_stmt(&mut self, stmt: &Stmt, level: usize) {
        let indent = if level > 0 { "  ".repeat(level) } else { String::new() };
        match stmt {
            Stmt::Command(cmd) => {
                self.lines.push(format!("{}Command: {}", indent, self.format_command(cmd)));
            }
            Stmt::IfStmt { condition, then_branch, else_branch } => {
                self.lines.push(format!("{}if ({}) {{", indent, self.format_expr(condition)));
                for s in then_branch { self.print_stmt(s, level + 1); }
                self.lines.push(format!("{}}} else {{", indent));
                for s in else_branch { self.print_stmt(s, level + 1); }
                self.lines.push(format!("{}}}", indent));
            }
            Stmt::WhileStmt { condition, body } => {
                self.lines.push(format!("{}while ({}) {{", indent, self.format_expr(condition)));
                for s in body { self.print_stmt(s, level + 1); }
                self.lines.push(format!("{}}}", indent));
            }
            Stmt::ForStmt { init, condition, update, body } => {
                self.lines.push(format!("{}for ({}; {}; {}) {{", indent, 
                    self.format_expr(init), 
                    self.format_expr(condition), 
                    self.format_expr(update)
                ));
                for s in body { self.print_stmt(s, level + 1); }
                self.lines.push(format!("{}}}", indent));
            }
            Stmt::ExprStmt(expr) => {
                self.lines.push(format!("{}Expr: {}", indent, self.format_expr(expr)));
            }
        }
    }

    fn format_command(&self, cmd: &Command) -> String {
        match cmd {
            Command::Move(m) => match m {
                MoveCommand::MoveUp => "move_up".into(),
                MoveCommand::MoveDown => "move_down".into(),
                MoveCommand::MoveLeft => "move_left".into(),
                MoveCommand::MoveRight => "move_right".into(),
            },
            Command::Action(a) => match a {
                ActionCommand::Jump => "jump".into(),
                ActionCommand::Attack => "attack".into(),
                ActionCommand::Defend => "defend".into(),
            },
        }
    }

    fn format_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Identifier(s) => s.clone(),
            Expr::Number(n) => n.to_string(),
            Expr::BinaryOp { left, op, right } => {
                let op_str = match op {
                    BinOp::Add => " + ",
                    BinOp::Sub => " - ",
                };
                format!("({}{}{})", self.format_expr(left), op_str, self.format_expr(right))
            }
        }
    }
}