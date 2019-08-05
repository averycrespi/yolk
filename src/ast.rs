/// Represents a Yolk AST node.
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

impl YolkNode {
    // Recursively finds Yolk AST nodes that satisfy a predicate function.
    pub fn find(&self, nodes: &mut Vec<YolkNode>, p: &Fn(YolkNode) -> bool) {
        if p(self.clone()) {
            nodes.push(self.clone());
        }
        match self {
            YolkNode::LetStmt { ident: _, expr } => expr.find(nodes, p),
            YolkNode::PrefixExpr { op: _, expr } => expr.find(nodes, p),
            YolkNode::CallExpr { ident: _, args } => {
                for arg in args.iter() {
                    arg.find(nodes, p);
                }
            }
            YolkNode::InfixExpr { lhs, op: _, rhs } => {
                lhs.find(nodes, p);
                rhs.find(nodes, p);
            }
            YolkNode::Array(exprs) => {
                for expr in exprs.iter() {
                    expr.find(nodes, p);
                }
            }
            _ => (),
        }
    }
}

/// Represents a Yolol AST node.
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
    Literal(f64),
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
