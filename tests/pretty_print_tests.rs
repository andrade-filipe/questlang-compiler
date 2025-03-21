use questlang::parser::ast::{Stmt, Command, MoveCommand, ActionCommand, Expr, BinOp};
use questlang::pretty_print::PrettyPrinter;

fn build_sample_ast() -> Vec<Stmt> {
    vec![
        // Statement 1: move_up
        Stmt::Command(Command::Move(MoveCommand::MoveUp)),
        // Statement 2: if (hero) { move_left } else { move_right }
        Stmt::IfStmt {
            condition: Expr::Identifier("hero".to_string()),
            then_branch: vec![Stmt::Command(Command::Move(MoveCommand::MoveLeft))],
            else_branch: vec![Stmt::Command(Command::Move(MoveCommand::MoveRight))],
        },
        // Statement 3: while (enemy) { jump }
        Stmt::WhileStmt {
            condition: Expr::Identifier("enemy".to_string()),
            body: vec![Stmt::Command(Command::Action(ActionCommand::Jump))],
        },
    ]
}

#[test]
fn test_pretty_print() {
    let ast = build_sample_ast();
    let mut printer = PrettyPrinter::new();
    let output = printer.print_stmts(&ast);
    let expected = "\
Command: move_up
if (hero) {
  Command: move_left
} else {
  Command: move_right
}
while (enemy) {
  Command: jump
}
";
    assert_eq!(output, expected, "O pretty print não corresponde ao esperado");
}
