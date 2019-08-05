use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};

const PREFIX: &str = "_yovec";

#[derive(Debug, Clone)]
pub enum Value {
    Number(Number),
    Array(Array),
}

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
                op: op.clone(),
                expr: Box::new(self.as_expr()),
            },
        }
    }

    pub fn apply_infix_op(&self, op: &InfixOp, other: &Number) -> Number {
        Number {
            expr: YololNode::InfixExpr {
                lhs: Box::new(self.as_expr()),
                op: op.clone(),
                rhs: Box::new(other.as_expr()),
            },
        }
    }

    pub fn as_expr(&self) -> YololNode {
        self.expr.clone()
    }

    pub fn resolve(&self, index: u32) -> (Number, YololNode) {
        let ident = format!("{}_{}", PREFIX, index);
        let number = Number {
            expr: YololNode::Ident(ident.to_string()),
        };
        let assign = YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(self.as_expr()),
        };
        (number, assign)
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    numbers: Vec<Number>,
}

impl Array {
    pub fn from_yolk_node(node: &YolkNode) -> Array {
        match node.clone() {
            YolkNode::Array(nodes) => Array {
                numbers: nodes.iter().map(Number::from_yolk_node).collect(),
            },
            _ => panic!("cannot create array from node: {:?}", node),
        }
    }

    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Array {
        Array {
            numbers: self.numbers.iter().map(|n| n.apply_prefix_op(op)).collect(),
        }
    }

    pub fn apply_infix_op(&self, op: &InfixOp, other: &Array) -> Array {
        Array {
            numbers: self
                .numbers
                .iter()
                .zip(other.numbers.iter())
                .map(|(m, n)| m.apply_infix_op(op, &n))
                .collect(),
        }
    }

    pub fn resolve(&self, index: u32) -> (Array, Vec<YololNode>) {
        let mut numbers = Vec::new();
        let mut assigns = Vec::new();
        for (elem_index, number) in self.numbers.iter().enumerate() {
            let ident = format!("{}_{}_{}", PREFIX, index, elem_index);
            numbers.push(Number {
                expr: YololNode::Ident(ident.to_string()),
            });
            assigns.push(YololNode::AssignStmt {
                ident: ident.to_string(),
                expr: Box::new(number.as_expr()),
            })
        }
        (Array { numbers: numbers }, assigns)
    }
}
