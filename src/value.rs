use crate::ast::{InfixOp, PrefixOp, YololNode};
use crate::error::YolkError;

const PREFIX: &str = "_yolk";

/// Represents a Yolk value.
#[derive(Debug, Clone)]
pub enum Value {
    Number(Number),
    Array(Array),
}

impl Value {
    /// Apply a prefix operation to a Yolk value.
    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Value {
        match self {
            Value::Number(n) => Value::Number(n.apply_prefix_op(op)),
            Value::Array(a) => Value::Array(Array {
                numbers: a.numbers.iter().map(|n| n.apply_prefix_op(op)).collect(),
            }),
        }
    }

    /// Apply an infix operation to two Yolk values.
    pub fn apply_infix_op(&self, op: &InfixOp, other: &Value) -> Result<Value, YolkError> {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => {
                Ok(Value::Number(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Array(lhs), Value::Number(rhs)) => {
                // Expand the right-hand side into an array of identical numbers
                let rhs = Array {
                    numbers: vec![rhs.clone(); lhs.numbers.len()],
                };
                Ok(Value::Array(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Number(lhs), Value::Array(rhs)) => {
                // Expand the left-hand side into an array of identical numbers
                let lhs = Array {
                    numbers: vec![lhs.clone(); rhs.numbers.len()],
                };
                Ok(Value::Array(lhs.apply_infix_op(op, rhs)))
            }
            (Value::Array(lhs), Value::Array(rhs)) => {
                if lhs.numbers.len() != rhs.numbers.len() {
                    Err(YolkError::MismatchingArrays(op.clone()))
                } else {
                    Ok(Value::Array(lhs.apply_infix_op(op, &rhs)))
                }
            }
        }
    }
}

/// Represents a Yolk number.
#[derive(Debug, Clone)]
pub struct Number {
    expr: YololNode,
}

impl Number {
    /// Creates a Yolk number from an identifier.
    pub fn from_ident(ident: &str) -> Number {
        Number {
            expr: YololNode::Ident(ident.to_string()),
        }
    }

    // Creates a Yolk number from a float.
    pub fn from_float(float: f64) -> Number {
        Number {
            expr: YololNode::Literal(float),
        }
    }

    /// Creates a Yolk number from an index.
    ///
    /// The number will contain an identifier based on the index.
    pub fn from_index(index: u32) -> Number {
        Number {
            expr: YololNode::Ident(Number::format_ident(index)),
        }
    }

    /// Returns a Yolk number as a Yolol expression.
    pub fn as_expr(&self) -> YololNode {
        self.expr.clone()
    }

    // Converts a Yolk number to a YOLOL assign statement.
    pub fn to_assign_stmt(&self, index: u32) -> YololNode {
        YololNode::AssignStmt {
            ident: Number::format_ident(index),
            expr: Box::new(self.as_expr()),
        }
    }

    fn format_ident(index: u32) -> String {
        format!("{}_{}", PREFIX, index)
    }

    fn apply_prefix_op(&self, op: &PrefixOp) -> Number {
        Number {
            expr: YololNode::PrefixExpr {
                op: op.clone(),
                expr: Box::new(self.as_expr()),
            },
        }
    }

    fn apply_infix_op(&self, op: &InfixOp, other: &Number) -> Number {
        Number {
            expr: YololNode::InfixExpr {
                lhs: Box::new(self.as_expr()),
                op: op.clone(),
                rhs: Box::new(other.as_expr()),
            },
        }
    }
}

/// Represents an array of Yolk numbers.
#[derive(Debug, Clone)]
pub struct Array {
    numbers: Vec<Number>,
}

impl Array {
    // Creates a Yolk array from Yolk numbers.
    pub fn from_numbers(numbers: &[Number]) -> Array {
        Array {
            numbers: numbers.to_vec(),
        }
    }

    /// Creates an indirect Yolk array from an index.
    ///
    /// The array will contain identifiers based on the index.
    pub fn from_index(index: u32, size: usize) -> Array {
        let mut numbers = Vec::new();
        for elem_index in 0..size {
            numbers.push(Number {
                expr: YololNode::Ident(Array::format_ident(index, elem_index)),
            })
        }
        Array { numbers: numbers }
    }

    // Converts a Yolk array to Yolol assign statements.
    //
    // The number of statements will be equal to the length of the array.
    pub fn to_assign_stmts(&self, index: u32) -> Vec<YololNode> {
        let mut assign_stmts = Vec::new();
        for (elem_index, number) in self.numbers.iter().enumerate() {
            assign_stmts.push(YololNode::AssignStmt {
                ident: Array::format_ident(index, elem_index),
                expr: Box::new(number.as_expr()),
            });
        }
        assign_stmts
    }

    fn apply_infix_op(&self, op: &InfixOp, other: &Array) -> Array {
        Array {
            numbers: self
                .numbers
                .iter()
                .zip(other.numbers.iter())
                .map(|(m, n)| m.apply_infix_op(op, n))
                .collect(),
        }
    }

    fn format_ident(index: u32, elem_index: usize) -> String {
        format!("{}_{}_{}", PREFIX, index, elem_index)
    }
}
