use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YololExpr, YololStmt};
use crate::error::YolkError;

use std::str::FromStr;

/// Represents a value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(Scalar),
    Vector(Vector),
}

impl Value {
    /// Applies a prefix operation to a value.
    pub fn apply_prefix_op(&self, op: &PrefixOp) -> Value {
        match self {
            Value::Scalar(s) => Value::Scalar(s.apply_prefix_op(op)),
            Value::Vector(v) => Value::Vector(v.apply_prefix_op(op)),
        }
    }

    /// Applies an infix operation to two values.
    ///
    /// If one value is a scalar and the other is a vector, the scalar will be
    /// repeated to produce another vector of the same length.
    ///
    /// If both values are vectors, they must have the same length.
    pub fn apply_infix_op(&self, op: &InfixOp, other: &Value) -> Result<Value, YolkError> {
        match (self, other) {
            (Value::Scalar(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Scalar(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Vector(lhs), Value::Scalar(rhs)) => {
                let rhs = Vector {
                    scalars: vec![rhs.clone(); lhs.scalars.len()],
                };
                Ok(Value::Vector(lhs.apply_infix_op(op, &rhs)))
            }
            (Value::Scalar(lhs), Value::Vector(rhs)) => {
                let lhs = Vector {
                    scalars: vec![lhs.clone(); rhs.scalars.len()],
                };
                Ok(Value::Vector(lhs.apply_infix_op(op, rhs)))
            }
            (Value::Vector(lhs), Value::Vector(rhs)) => {
                if lhs.scalars.len() != rhs.scalars.len() {
                    Err(YolkError::MismatchedArrays)
                } else {
                    Ok(Value::Vector(lhs.apply_infix_op(op, &rhs)))
                }
            }
        }
    }

    /// Left-folds values to a single value.
    pub fn left_fold(values: &[Value], op: &InfixOp, start: &Scalar) -> Value {
        let mut result = start.clone();
        for value in values.iter() {
            match value {
                Value::Scalar(s) => result = result.apply_infix_op(op, s),
                Value::Vector(v) => {
                    for s in v.scalars.iter() {
                        result = result.apply_infix_op(op, s);
                    }
                }
            }
        }
        Value::Scalar(result)
    }
}

/// Represents a scalar value.
#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    expr: YololExpr,
}

impl FromStr for Scalar {
    type Err = YolkError;

    /// Parses a scalar from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Scalar {
            expr: YololExpr::Ident(s.to_string()),
        })
    }
}

impl From<YololNumber> for Scalar {
    /// Converts a Yolol number to a scalar.
    fn from(y: YololNumber) -> Self {
        Scalar {
            expr: YololExpr::Literal(y),
        }
    }
}

impl Scalar {
    /// Converts a scalar to a Yolol assign statement.
    pub fn to_assign_stmt(&self, ident: &str) -> YololStmt {
        YololStmt::Assign {
            ident: ident.to_string(),
            expr: Box::new(self.as_expr()),
        }
    }

    /// Returns a scalar as a Yolol expression.
    fn as_expr(&self) -> YololExpr {
        self.expr.clone()
    }

    /// Applies a prefix operation to a scalar.
    fn apply_prefix_op(&self, op: &PrefixOp) -> Self {
        Scalar {
            expr: YololExpr::Prefix {
                op: *op,
                expr: Box::new(self.as_expr()),
            },
        }
    }

    /// Applies an infix operation to two scalars.
    fn apply_infix_op(&self, op: &InfixOp, other: &Scalar) -> Self {
        Scalar {
            expr: YololExpr::Infix {
                lhs: Box::new(self.as_expr()),
                op: *op,
                rhs: Box::new(other.as_expr()),
            },
        }
    }
}

/// Represents a vector of scalars.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    scalars: Vec<Scalar>,
}

impl From<Vec<Scalar>> for Vector {
    /// Converts scalars to a vector.
    fn from(scalars: Vec<Scalar>) -> Self {
        Vector { scalars: scalars }
    }
}

impl Vector {
    /// Creates a vector from an expanded identifier.
    pub fn from_expanded_ident(ident: &str, size: usize) -> Self {
        let mut scalars = Vec::new();
        for i in 0..size {
            // Parsing can never fail here
            let s: Scalar = format!("{}_{}", ident, i).parse().unwrap();
            scalars.push(s);
        }
        Vector { scalars: scalars }
    }

    /// Converts a vector to Yolol assign statements using a given identifier.
    pub fn to_assign_stmts(&self, ident: &str) -> Vec<YololStmt> {
        let mut stmts = Vec::new();
        for (elem_index, scalar) in self.scalars.iter().enumerate() {
            stmts.push(YololStmt::Assign {
                ident: format!("{}_{}", ident, elem_index),
                expr: Box::new(scalar.as_expr()),
            });
        }
        stmts
    }

    /// Applies a prefix operation to a vector.
    fn apply_prefix_op(&self, op: &PrefixOp) -> Self {
        Vector {
            scalars: self.scalars.iter().map(|s| s.apply_prefix_op(op)).collect(),
        }
    }

    /// Applies an infix operation to two vectors.
    fn apply_infix_op(&self, op: &InfixOp, other: &Vector) -> Self {
        Vector {
            scalars: self
                .scalars
                .iter()
                .zip(other.scalars.iter())
                .map(|(m, n)| m.apply_infix_op(op, n))
                .collect(),
        }
    }
}
