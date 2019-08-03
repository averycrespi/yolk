use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};

#[derive(Debug, Clone)]
pub struct Number {
    value: Value,
    queue: Vec<Operation>,
}

#[derive(Debug, Clone)]
enum Value {
    Ident(String),
    Float(f64),
}

#[derive(Debug, Clone)]
enum Operation {
    Prefix(PrefixOp),
    Infix { op: InfixOp, other: Number },
}

impl Number {
    pub fn from_node(node: &YolkNode) -> Number {
        match node.to_owned() {
            YolkNode::Ident(s) => Number {
                value: Value::Ident(s),
                queue: Vec::new(),
            },
            YolkNode::Number(f) => Number {
                value: Value::Float(f),
                queue: Vec::new(),
            },
            _ => panic!("cannot create number from node: {:?}", node),
        }
    }

    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Number {
        let mut result = self.to_owned();
        result.queue.push(Operation::Prefix(op.to_owned()));
        result
    }

    pub fn apply_infix_op(&self, op: &InfixOp, other: &Number) -> Number {
        let mut result = self.to_owned();
        result.queue.push(Operation::Infix {
            op: op.to_owned(),
            other: other.to_owned(),
        });
        result
    }

    pub fn to_expr(&self) -> YololNode {
        let mut expr = match self.value.to_owned() {
            Value::Ident(s) => YololNode::Ident(s),
            Value::Float(f) => YololNode::Number(f),
        };
        for op in self.queue.iter() {
            match op.to_owned() {
                Operation::Prefix(op) => {
                    expr = YololNode::PrefixExpr {
                        op: op,
                        expr: Box::new(expr),
                    };
                }
                Operation::Infix { op, other } => {
                    expr = YololNode::InfixExpr {
                        lhs: Box::new(expr),
                        op: op,
                        rhs: Box::new(other.to_expr()),
                    };
                }
            }
        }
        expr
    }
}
