use crate::ast::{InfixOp, PrefixOp, YolkNode};
use crate::number::Number;

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
}
