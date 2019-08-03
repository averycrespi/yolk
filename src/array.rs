use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};
use crate::number::Number;

const PREFIX: &str = "_yovec";

#[derive(Debug, Clone)]
pub struct Array {
    numbers: Vec<Number>,
}

impl Array {
    pub fn from_node(node: &YolkNode) -> Array {
        match node.to_owned() {
            YolkNode::Array(nodes) => Array {
                numbers: nodes.iter().map(Number::from_node).collect(),
            },
            _ => panic!("cannot create array from node: {:?}", node),
        }
    }

    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Array {
        Array {
            numbers: self
                .to_owned()
                .numbers
                .iter()
                .map(|n| n.apply_prefix_op(op))
                .collect(),
        }
    }

    pub fn apply_infix_op(&self, op: &InfixOp, other: &Array) -> Array {
        Array {
            numbers: self
                .to_owned()
                .numbers
                .iter()
                .zip(other.to_owned().numbers.iter())
                .map(|(m, n)| m.apply_infix_op(op, n))
                .collect(),
        }
    }

    pub fn apply_join(&self, other: &Array) -> Array {
        let mut numbers = self.to_owned().numbers;
        numbers.extend(other.to_owned().numbers);
        Array { numbers: numbers }
    }

    pub fn to_assign_stmts(&self, index: u32) -> Vec<YololNode> {
        let mut stmts = Vec::new();
        for (i, number) in self.to_owned().numbers.iter().enumerate() {
            stmts.push(YololNode::AssignStmt {
                ident: format!("{}_{}_{}", PREFIX, index, i),
                expr: Box::new(number.to_expr()),
            })
        }
        stmts
    }
}
