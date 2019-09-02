use std::fmt;

use yolol_number::YololNumber;

#[derive(Debug, Clone, PartialEq)]
pub enum YolkStmt {
    Import {
        ident: String,
    },
    Define {
        ident: String,
        params: Vec<String>,
        body: Box<YolkExpr>,
    },
    Let {
        ident: String,
        expr: Box<YolkExpr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum YolkExpr {
    Prefix {
        op: PrefixOp,
        expr: Box<YolkExpr>,
    },
    Builtin {
        ident: String,
        args: Vec<YolkExpr>,
    },
    Call {
        ident: String,
        args: Vec<YolkExpr>,
    },
    Infix {
        lhs: Box<YolkExpr>,
        op: InfixOp,
        rhs: Box<YolkExpr>,
    },
    Ident(String),
    Literal(YololNumber),
    Array(Vec<YolkExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum YololStmt {
    Assign { ident: String, expr: Box<YololExpr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum YololExpr {
    Prefix {
        op: PrefixOp,
        expr: Box<YololExpr>,
    },
    Infix {
        lhs: Box<YololExpr>,
        op: InfixOp,
        rhs: Box<YololExpr>,
    },
    Ident(String),
    Literal(YololNumber),
}

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

    pub fn is_commutative(&self) -> bool {
        match self {
            InfixOp::Add
            | InfixOp::Mul
            | InfixOp::Equal
            | InfixOp::NotEqual
            | InfixOp::Or
            | InfixOp::And => true,
            _ => false,
        }
    }

    pub fn is_associative(&self) -> bool {
        match self {
            InfixOp::Add | InfixOp::Mul => true,
            _ => false,
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
