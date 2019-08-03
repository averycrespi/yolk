use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};

#[derive(Debug, Clone)]
pub struct Number {
    expr: YololNode,
}

impl Number {
    pub fn from_yolk_node(node: &YolkNode) -> Number {
        match node {
            YolkNode::Ident(s) => Number {
                expr: YololNode::Ident(s.to_string()),
            },
            YolkNode::Number(f) => Number {
                expr: YololNode::Number(*f),
            },
            _ => panic!("cannot create number from node: {:?}", node),
        }
    }

    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Number {
        Number {
            expr: YololNode::PrefixExpr {
                op: op.to_owned(),
                expr: Box::new(self.as_expr()),
            },
        }
    }

    pub fn apply_infix_op(&self, op: &InfixOp, other: &Number) -> Number {
        Number {
            expr: YololNode::InfixExpr {
                lhs: Box::new(self.as_expr()),
                op: op.to_owned(),
                rhs: Box::new(other.as_expr()),
            },
        }
    }

    pub fn as_expr(&self) -> YololNode {
        self.expr.to_owned()
    }
}
