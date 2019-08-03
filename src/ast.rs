#[derive(Debug, Clone)]
pub enum YolkNode {
    ImportStmt {
        ident: String,
    },
    DefineStmt {
        ident: String,
        params: Vec<String>,
        body: Box<YolkNode>,
    },
    LetStmt {
        ident: String,
        expr: Box<YolkNode>,
    },
    ExportStmt {
        ident: String,
    },
    PrefixExpr {
        op: PrefixOp,
        expr: Box<YolkNode>,
    },
    MacroExpr {
        ident: String,
        args: Vec<YolkNode>,
    },
    InfixExpr {
        lhs: Box<YolkNode>,
        op: InfixOp,
        rhs: Box<YolkNode>,
    },
    Ident(String),
    Number(f64),
    Array(Vec<YolkNode>),
}

#[derive(Debug, Clone)]
pub enum YololNode {
    AssignStmt {
        ident: String,
        expr: Box<YololNode>,
    },
    PrefixExpr {
        op: PrefixOp,
        expr: Box<YololNode>,
    },
    InfixExpr {
        lhs: Box<YololNode>,
        op: InfixOp,
        rhs: Box<YololNode>,
    },
    Ident(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOp {
    Not,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}
