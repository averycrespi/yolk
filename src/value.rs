use crate::ast::{InfixOp, PrefixOp, YolkNode, YololNode};

const PREFIX: &str = "_yovec";

/// Represents a Yolk value.
#[derive(Debug, Clone)]
pub enum Value {
    Number(Number),
    Array(Array),
}

/// Represents a Yolk number.
#[derive(Debug, Clone)]
pub struct Number {
    expr: YololNode,
}

impl Number {
    /// Creates a Yolk number from a Yolk AST node.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a YolkNode::Ident or YolkNode::Number.
    pub fn from_yolk_node(node: &YolkNode) -> Number {
        match node {
            YolkNode::Ident(s) => Number {
                expr: YololNode::Ident(s.to_string()),
            },
            YolkNode::Number(f) => Number {
                expr: YololNode::Number(*f),
            },
            _ => panic!("cannot create Yolk number from Yolk node: {:?}", node),
        }
    }

    /// Applies a prefix operation to a Yolk number.
    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Number {
        Number {
            expr: YololNode::PrefixExpr {
                op: op.clone(),
                expr: Box::new(self.as_expr()),
            },
        }
    }

    /// Applies an infix operation to two Yolk numbers.
    pub fn apply_infix_op(&self, op: &InfixOp, other: &Number) -> Number {
        Number {
            expr: YololNode::InfixExpr {
                lhs: Box::new(self.as_expr()),
                op: op.clone(),
                rhs: Box::new(other.as_expr()),
            },
        }
    }

    /// Returns a Yolk number as a Yolol expression.
    pub fn as_expr(&self) -> YololNode {
        self.expr.clone()
    }

    /// Resolves a Yolk number.
    ///
    /// Returns a simplified Yolk number and its corresponding Yolol assign statement.
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

/// Represents an array of Yolk numbers.
#[derive(Debug, Clone)]
pub struct Array {
    numbers: Vec<Number>,
}

impl Array {
    /// Creates an Yolk array from a Yolk AST node.
    ///
    /// # Panics
    ///
    /// Panics if the node is not a YolkNode::Array.
    pub fn from_yolk_node(node: &YolkNode) -> Array {
        match node.clone() {
            YolkNode::Array(nodes) => Array {
                numbers: nodes.iter().map(Number::from_yolk_node).collect(),
            },
            _ => panic!("cannot create array from node: {:?}", node),
        }
    }

    /// Applies a prefix operation element-wise to a Yolk array.
    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Array {
        Array {
            numbers: self.numbers.iter().map(|n| n.apply_prefix_op(op)).collect(),
        }
    }

    /// Applies an infix operation element-wise to two Yolk arrays.
    ///
    /// # Panics
    ///
    /// Panics if the arrays have different lengths.
    pub fn apply_infix_op(&self, op: &InfixOp, other: &Array) -> Array {
        if self.numbers.len() != other.numbers.len() {
            panic!("cannot apply operation to mismatched Yolk arrays: {:?}", op);
        }
        Array {
            numbers: self
                .numbers
                .iter()
                .zip(other.numbers.iter())
                .map(|(m, n)| m.apply_infix_op(op, &n))
                .collect(),
        }
    }

    /// Resolves an Yolk array.
    ///
    /// Returns a simplified Yolk array and its corresponding Yolol assign statements.
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
