use std::fmt;

use crate::number::YololNumber;

const LINE_LIMIT: usize = 70;

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

impl YololNode {
    /// Format Yolol assign statements as a program.
    pub fn format_as_program(stmts: &[YololNode]) -> String {
        let mut program = String::new();
        let mut line = String::new();
        for stmt in stmts.iter() {
            if let YololNode::AssignStmt { ident, expr } = stmt {
                let formatted = YololNode::format_expr(expr, 0);
                if line.len() + formatted.len() + 1 > LINE_LIMIT {
                    program.push_str(&format!("{}\n{}={}\n", line, ident, formatted));
                    line.clear();
                } else {
                    line.push_str(&format!("{}={} ", ident, formatted));
                }
            } else {
                panic!("expected assign statement, but got: {:?}", stmt);
            }
        }
        program.push_str(line.as_str());
        program.trim().to_string()
    }

    fn format_expr(expr: &YololNode, parent_prec: u32) -> String {
        match expr {
            YololNode::PrefixExpr { op, expr } => {
                let prec = op.to_precedence();
                format!(
                    //TODO: remove extra space after op
                    "{}{} {}{}",
                    if prec < parent_prec { "(" } else { "" },
                    op,
                    YololNode::format_expr(expr, prec),
                    if prec < parent_prec { ")" } else { "" },
                )
            }
            YololNode::InfixExpr { lhs, op, rhs } => {
                let prec = op.to_precedence();
                format!(
                    //TODO: remove extra spaces around op
                    "{}{} {} {}{}",
                    if prec < parent_prec { "(" } else { "" },
                    YololNode::format_expr(lhs, prec),
                    op,
                    YololNode::format_expr(rhs, prec),
                    if prec < parent_prec { ")" } else { "" },
                )
            }
            YololNode::Ident(s) => s.to_string(),
            YololNode::Literal(y) => y.to_string(),
            _ => panic!("expected expression, but got: {:?}", expr),
        }
    }
}

/// Represents a prefix operation.
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
    fn to_precedence(&self) -> u32 {
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
    fn to_precedence(&self) -> u32 {
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
