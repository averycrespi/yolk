use std::fmt;

use yolol_number::YololNumber;

/// Represents a Yolk AST node.
#[derive(Debug, Clone, PartialEq)]
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
    BuiltinExpr {
        ident: String,
        args: Vec<YolkNode>,
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
    Literal(YololNumber),
    Array(Vec<YolkNode>),
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
    Literal(YololNumber),
}

/// Represents a prefix operation.
///
/// Yolk and Yolol have the same prefix operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixOp {
    Neg,
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

impl PrefixOp {
    /// Converts a prefix operation to a precedence value.
    pub fn to_precedence(&self) -> u32 {
        match self {
            PrefixOp::Neg => 100,
            _ => 90,
        }
    }
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrefixOp::Neg => write!(f, "-"),
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
///
/// Yolk and Yolol have the same infix operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl InfixOp {
    // Converts an infix operation to a precedence value.
    pub fn to_precedence(&self) -> u32 {
        match self {
            InfixOp::Exp => 80,
            InfixOp::Mul | InfixOp::Div | InfixOp::Mod => 70,
            InfixOp::Add | InfixOp::Sub => 60,
            InfixOp::LessThan
            | InfixOp::LessEqual
            | InfixOp::GreaterThan
            | InfixOp::GreaterEqual => 50,
            InfixOp::Equal | InfixOp::NotEqual => 40,
            InfixOp::Or => 30,
            InfixOp::And => 20,
        }
    }
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
