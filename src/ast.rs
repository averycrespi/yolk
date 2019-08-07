use std::fmt;

/// Represents a Yolk AST node.
#[derive(Debug, Clone)]
pub enum YolkNode {
    ImportStmt {
        source: String,
        ident: String,
    },
    DefineStmt {
        source: String,
        ident: String,
        params: Vec<String>,
        body: Box<YolkNode>,
    },
    LetStmt {
        source: String,
        ident: String,
        expr: Box<YolkNode>,
    },
    ExportStmt {
        source: String,
        ident: String,
    },
    PrefixExpr {
        op: PrefixOp,
        expr: Box<YolkNode>,
    },
    CallExpr {
        ident: String,
        args: Vec<YolkNode>,
    },
    InfixExpr {
        lhs: Box<YolkNode>,
        op: InfixOp,
        rhs: Box<YolkNode>,
    },
    Ident(String),
    Literal(f64),
    Array(Vec<YolkNode>),
}

//TODO: include source for all Yolk nodes?
impl YolkNode {
    /// Gets the source code of a Yolk AST node.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a statement.
    pub fn source(&self) -> String {
        match self {
            YolkNode::ImportStmt { source, ident: _ } => source.to_string(),
            YolkNode::DefineStmt {
                source,
                ident: _,
                params: _,
                body: _,
            } => source.to_string(),
            YolkNode::LetStmt {
                source,
                ident: _,
                expr: _,
            } => source.to_string(),
            YolkNode::ExportStmt { source, ident: _ } => source.to_string(),
            _ => panic!("expected statement, but got: {:?}", self),
        }
    }
}

/// Represents a Yolol AST node.
#[derive(Debug, Clone, PartialEq)]
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
    Literal(f64),
}

impl fmt::Display for YololNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YololNode::AssignStmt { ident, expr } => write!(f, "{}={}", ident, expr),
            YololNode::PrefixExpr { op, expr } => write!(f, "{}({})", op, expr),
            YololNode::InfixExpr { lhs, op, rhs } => write!(f, "({}){}({})", lhs, op, rhs),
            YololNode::Ident(s) => write!(f, "{}", s),
            YololNode::Literal(n) => write!(f, "{}", n),
        }
    }
}

/// Represents a prefix operation.
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

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrefixOp::Not => write!(f, "not"),
            PrefixOp::Abs => write!(f, "abs"),
            PrefixOp::Sqrt => write!(f, "sqrt"),
            PrefixOp::Sin => write!(f, "sin"),
            PrefixOp::Cos => write!(f, "cos"),
            PrefixOp::Tan => write!(f, "tan"),
            PrefixOp::Asin => write!(f, "asin"),
            PrefixOp::Acos => write!(f, "acos"),
            PrefixOp::Atan => write!(f, "atan"),
        }
    }
}

/// Represents an infix operation.
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

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Sub => write!(f, "-"),
            InfixOp::Mul => write!(f, "*"),
            InfixOp::Div => write!(f, "/"),
            InfixOp::Mod => write!(f, "%"),
            InfixOp::Exp => write!(f, "^"),
            InfixOp::LessThan => write!(f, "<"),
            InfixOp::LessEqual => write!(f, "<="),
            InfixOp::GreaterThan => write!(f, ">"),
            InfixOp::GreaterEqual => write!(f, ">="),
            InfixOp::Equal => write!(f, "=="),
            InfixOp::NotEqual => write!(f, "!="),
            InfixOp::And => write!(f, "and"),
            InfixOp::Or => write!(f, "or"),
        }
    }
}
