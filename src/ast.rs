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

impl fmt::Display for YololStmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Assign { ident, expr } => write!(f, "{}={}", ident, expr.to_string()),
        }
    }
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

impl fmt::Display for YololExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (s, _) = self.format(0);
        write!(f, "{}", s)
    }
}

impl YololExpr {
    fn format(&self, parent_prec: u32) -> (String, bool) {
        match self {
            Self::Prefix { op, expr } => {
                let prec = op.to_precedence();
                let (expr, child_wrapped) = expr.format(prec);
                let wrapped = prec < parent_prec;
                // Alphabetic ops must be surrounded with whitespace or parentheses
                let is_alpha = op.to_string().chars().all(char::is_alphabetic);
                let spaced = !child_wrapped && is_alpha;
                (
                    format!(
                        "{lparen}{op}{space}{expr}{rparen}",
                        lparen = if wrapped { "(" } else { "" },
                        op = op.to_string(),
                        space = if spaced { " " } else { "" },
                        expr = expr,
                        rparen = if wrapped { ")" } else { "" },
                    ),
                    wrapped,
                )
            }
            Self::Infix { lhs, op, rhs } => {
                let prec = op.to_precedence();
                let (lhs, lhs_wrapped) = lhs.format(prec);
                let (rhs, rhs_wrapped) = rhs.format(prec);
                // If the op is associative, we can reduce "(a+b)+c" to "a+b+c"
                let wrapped = if op.is_associative() {
                    prec < parent_prec
                } else {
                    prec <= parent_prec
                };
                // Alphabetic ops must be surrounded with whitespace or parentheses
                let is_alpha = op.to_string().chars().all(char::is_alphabetic);
                let lhs_spaced = !lhs_wrapped && is_alpha;
                let rhs_spaced = !rhs_wrapped && is_alpha;
                (
                    format!(
                        "{lparen}{lhs}{lhs_space}{op}{rhs_space}{rhs}{rparen}",
                        lparen = if wrapped { "(" } else { "" },
                        lhs = lhs,
                        lhs_space = if lhs_spaced { " " } else { "" },
                        op = op.to_string(),
                        rhs_space = if rhs_spaced { " " } else { "" },
                        rhs = rhs,
                        rparen = if wrapped { ")" } else { "" },
                    ),
                    wrapped,
                )
            }
            Self::Ident(s) => (s.to_string(), false),
            Self::Literal(y) => (y.to_string(), false),
        }
    }
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
    fn to_precedence(&self) -> u32 {
        match self {
            Self::Neg => 100,
            _ => 90,
        }
    }
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "not"),
            Self::Abs => write!(f, "abs"),
            Self::Sqrt => write!(f, "sqrt"),
            Self::Sin => write!(f, "sin"),
            Self::Cos => write!(f, "cos"),
            Self::Tan => write!(f, "tan"),
            Self::Asin => write!(f, "asin"),
            Self::Acos => write!(f, "acos"),
            Self::Atan => write!(f, "atan"),
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
    fn to_precedence(&self) -> u32 {
        match self {
            Self::Exp => 80,
            Self::Mul | Self::Div | Self::Mod => 70,
            Self::Add | Self::Sub => 60,
            Self::LessThan | Self::LessEqual | Self::GreaterThan | Self::GreaterEqual => 50,
            Self::Equal | Self::NotEqual => 40,
            Self::Or => 30,
            Self::And => 20,
        }
    }

    pub fn is_associative(&self) -> bool {
        match self {
            Self::Add | Self::Mul => true,
            _ => false,
        }
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Exp => write!(f, "^"),
            Self::LessThan => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
        }
    }
}
