
#[derive(Debug, Clone)]
pub enum Command {
    Move(MoveCommand),
    Action(ActionCommand),
}

#[derive(Debug, Clone)]
pub enum MoveCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Debug, Clone)]
pub enum ActionCommand {
    Jump,
    Attack,
    Defend,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Command(Command),

    IfStmt {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },

    WhileStmt {
        condition: Expr,
        body: Vec<Stmt>,
    },

    ForStmt {
        init: Expr,
        condition: Expr,
        update: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    Number(i32),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}
