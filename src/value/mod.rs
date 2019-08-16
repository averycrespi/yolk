use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YololNode};
use crate::error::TranspileError;

#[cfg(test)]
mod tests;

/// Represents a Yolk value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(NumberExpr),
    Array(ArrayExpr),
}

impl Value {
    /// Apply a prefix operation to a Yolk value.
    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Value {
        match self {
            Value::Number(n) => Value::Number(n.apply_prefix_op(op)),
            Value::Array(a) => Value::Array(a.apply_prefix_op(op)),
        }
    }

    /// Apply an infix operation to two Yolk values.
    pub fn apply_infix_op(&self, op: &InfixOp, other: &Value) -> Result<Value, TranspileError> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Ok(Value::Number(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Array(lhs), Value::Number(rhs)) => {
                // Expand the right-hand side into an array of identical numbers
                let rhs = ArrayExpr {
                    numbers: vec![rhs.clone(); lhs.numbers.len()],
                };
                Ok(Value::Array(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Number(lhs), Value::Array(rhs)) => {
                // Expand the left-hand side into an array of identical numbers
                let lhs = ArrayExpr {
                    numbers: vec![lhs.clone(); rhs.numbers.len()],
                };
                Ok(Value::Array(lhs.apply_infix_op(op, rhs)))
            }
            (Value::Array(lhs), Value::Array(rhs)) => {
                if lhs.numbers.len() != rhs.numbers.len() {
                    Err(TranspileError::MismatchedArrays)
                } else {
                    Ok(Value::Array(lhs.apply_infix_op(op, &rhs)))
                }
            }
        }
    }

    // Reduces Yolk values to a single Yolk number expression.
    pub fn reduce(values: &[Value], op: &InfixOp, start: &NumberExpr) -> Value {
        let mut result = start.clone();
        for value in values.iter() {
            match value {
                Value::Number(number) => result = result.apply_infix_op(op, number),
                Value::Array(array) => {
                    for number in array.numbers.iter() {
                        result = result.apply_infix_op(op, number);
                    }
                }
            }
        }
        Value::Number(result)
    }
}

/// Represents a Yolk number expression.
#[derive(Debug, Clone, PartialEq)]
pub struct NumberExpr {
    expr: YololNode,
}

impl NumberExpr {
    /// Creates a Yolk number expression from an identifier.
    pub fn from_ident(ident: &str) -> NumberExpr {
        NumberExpr {
            expr: YololNode::Ident(ident.to_string()),
        }
    }

    // Creates a Yolk number expression from a Yolol number.
    pub fn from_yolol_number(num: YololNumber) -> NumberExpr {
        NumberExpr {
            expr: YololNode::Literal(num),
        }
    }

    /// Returns a Yolk number expression as a Yolol expression.
    pub fn as_expr(&self) -> YololNode {
        self.expr.clone()
    }

    // Converts a Yolk number expression to a Yolol assign statement.
    pub fn to_assign_stmt(&self, ident: &str) -> YololNode {
        YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(self.as_expr()),
        }
    }

    fn apply_prefix_op(&self, op: &PrefixOp) -> NumberExpr {
        NumberExpr {
            expr: YololNode::PrefixExpr {
                op: *op,
                expr: Box::new(self.as_expr()),
            },
        }
    }

    fn apply_infix_op(&self, op: &InfixOp, other: &NumberExpr) -> NumberExpr {
        NumberExpr {
            expr: YololNode::InfixExpr {
                lhs: Box::new(self.as_expr()),
                op: *op,
                rhs: Box::new(other.as_expr()),
            },
        }
    }
}

/// Represents an array of Yolk number expression.
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayExpr {
    numbers: Vec<NumberExpr>,
}

impl ArrayExpr {
    // Creates a Yolk array expression from an identifier.
    pub fn from_ident(ident: &str, size: usize) -> ArrayExpr {
        let mut numbers = Vec::new();
        for i in 0..size {
            numbers.push(NumberExpr::from_ident(&format!("{}_{}", ident, i)));
        }
        ArrayExpr { numbers: numbers }
    }

    // Creates a Yolk array expression from Yolk number expressions.
    pub fn from_number_exprs(numbers: &[NumberExpr]) -> ArrayExpr {
        ArrayExpr {
            numbers: numbers.to_vec(),
        }
    }

    // Returns a Yolk array expression as a vector of Yolol expressions.
    pub fn as_exprs(&self) -> Vec<YololNode> {
        self.numbers.iter().map(|n| n.as_expr()).collect()
    }

    // Converts a Yolk array expression to Yolol assign statements.
    pub fn to_assign_stmts(&self, ident: &str) -> Vec<YololNode> {
        let mut assign_stmts = Vec::new();
        for (elem_index, number) in self.numbers.iter().enumerate() {
            assign_stmts.push(YololNode::AssignStmt {
                ident: format!("{}_{}", ident, elem_index),
                expr: Box::new(number.as_expr()),
            });
        }
        assign_stmts
    }

    fn apply_prefix_op(&self, op: &PrefixOp) -> ArrayExpr {
        ArrayExpr {
            numbers: self.numbers.iter().map(|n| n.apply_prefix_op(op)).collect(),
        }
    }

    fn apply_infix_op(&self, op: &InfixOp, other: &ArrayExpr) -> ArrayExpr {
        ArrayExpr {
            numbers: self
                .numbers
                .iter()
                .zip(other.numbers.iter())
                .map(|(m, n)| m.apply_infix_op(op, n))
                .collect(),
        }
    }
}
