#[derive(Debug)]
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
        op: YolkPrefixOp,
        expr: Box<YolkNode>,
    },
    MacroExpr {
        ident: String,
        args: Vec<YolkNode>,
    },
    InfixExpr {
        lhs: Box<YolkNode>,
        op: YolkInfixOp,
        rhs: Box<YolkNode>,
    },
    Ident(String),
    Number(f64),
    Array(Vec<YolkNode>),
}

#[derive(Debug)]
pub enum YolkPrefixOp {
    Not,
    Abs,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}

#[derive(Debug)]
pub enum YolkInfixOp {
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
    Join,
}
