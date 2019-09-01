use std::collections::HashMap;

use num_traits::identities::{One, Zero};
use yolol_number::prelude::*;

use crate::ast::{InfixOp, PrefixOp, YololExpr, YololStmt};

/// Optimizes Yolol statements.
///
/// This function is idempotent.
pub fn optimize(stmts: Vec<YololStmt>) -> Vec<YololStmt> {
    let mut curr = stmts;
    loop {
        //TODO: hash instead of clone
        let before = curr.clone();
        let vars = find_literal_vars(&curr);
        curr = curr.into_iter().map(|s| reduce_stmt(s, &vars)).collect();
        if before == curr {
            break;
        }
    }
    curr
}

/// Find variables that have literal values.
///
/// These literals are used for constant propagation.
fn find_literal_vars(stmts: &[YololStmt]) -> HashMap<String, YololExpr> {
    let mut state = HashMap::new();
    for stmt in stmts.iter() {
        match stmt {
            YololStmt::Assign { ident, expr } => {
                if let YololExpr::Literal(y) = **expr {
                    state.insert(ident.to_string(), YololExpr::Literal(y));
                }
            }
        }
    }
    state
}

/// Reduce expressions in a Yolol statement.
fn reduce_stmt(stmt: YololStmt, vars: &HashMap<String, YololExpr>) -> YololStmt {
    match stmt {
        YololStmt::Assign { ident, expr } => YololStmt::Assign {
            ident: ident.to_string(),
            expr: Box::new(reduce_expr(*expr, vars)),
        },
    }
}

/// Reduce expressions in a Yolol expression.
fn reduce_expr(expr: YololExpr, vars: &HashMap<String, YololExpr>) -> YololExpr {
    match expr {
        YololExpr::Prefix { op, expr } => match (op, &*expr) {
            // Apply prefix operations literals
            (op, YololExpr::Literal(y)) => match op {
                PrefixOp::Neg => YololExpr::Literal(-*y),
                PrefixOp::Not => YololExpr::Literal(!*y),
                PrefixOp::Abs => YololExpr::Literal(y.abs()),
                PrefixOp::Sqrt if y >= &YololNumber::zero() => YololExpr::Literal(y.sqrt()),
                PrefixOp::Sin => YololExpr::Literal(y.sin()),
                PrefixOp::Cos => YololExpr::Literal(y.cos()),
                PrefixOp::Tan if !y.cos().is_zero() => YololExpr::Literal(y.tan()),
                PrefixOp::Asin if y.abs() <= YololNumber::one() => YololExpr::Literal(y.asin()),
                PrefixOp::Acos if y.abs() <= YololNumber::one() => YololExpr::Literal(y.acos()),
                PrefixOp::Atan => YololExpr::Literal(y.atan()),
                _ => YololExpr::Prefix { op: op, expr: expr },
            },
            _ => YololExpr::Prefix {
                op: op,
                expr: Box::new(reduce_expr(*expr, vars)),
            },
        },
        YololExpr::Infix { lhs, op, rhs } => match (&*lhs, op, &*rhs) {
            // Reduce identity operations
            (YololExpr::Literal(y), InfixOp::Add, _) if y.is_zero() => *rhs,
            (_, InfixOp::Add, YololExpr::Literal(y)) if y.is_zero() => *lhs,
            (_, InfixOp::Sub, YololExpr::Literal(y)) if y.is_zero() => *lhs,
            (_, InfixOp::Mul, YololExpr::Literal(y)) | (YololExpr::Literal(y), InfixOp::Mul, _)
                if y.is_zero() =>
            {
                YololExpr::Literal(YololNumber::zero())
            }
            (_, InfixOp::Mul, YololExpr::Literal(y)) if y.is_one() => *lhs,
            (YololExpr::Literal(y), InfixOp::Mul, _) if y.is_one() => *rhs,

            (_, InfixOp::Div, YololExpr::Literal(y)) if y.is_one() => *lhs,
            (YololExpr::Literal(y), InfixOp::Exp, _) if y.is_one() => {
                YololExpr::Literal(YololNumber::one())
            }
            (_, InfixOp::Exp, YololExpr::Literal(y)) if y.is_one() => *rhs,
            // Apply infix operations to literals
            (YololExpr::Literal(y), op, YololExpr::Literal(z)) => match op {
                InfixOp::Add => YololExpr::Literal(y.yolol_add(*z)),
                InfixOp::Sub => YololExpr::Literal(y.yolol_sub(*z)),
                InfixOp::Mul => YololExpr::Literal(y.yolol_mul(*z)),
                // Unwrap cannot panic because z cannot be zero
                InfixOp::Div if !z.is_zero() => YololExpr::Literal(y.yolol_div(*z).unwrap()),
                InfixOp::Mod if !z.is_zero() => YololExpr::Literal(y.yolol_mod(*z)),
                InfixOp::Exp if !y.is_zero() || !z.is_zero() => YololExpr::Literal(y.pow(*z)),
                InfixOp::LessThan => YololExpr::Literal((y < z).into()),
                InfixOp::LessEqual => YololExpr::Literal((y <= z).into()),
                InfixOp::GreaterThan => YololExpr::Literal((y > z).into()),
                InfixOp::GreaterEqual => YololExpr::Literal((y >= z).into()),
                InfixOp::Equal => YololExpr::Literal((y == z).into()),
                InfixOp::NotEqual => YololExpr::Literal((y != z).into()),
                InfixOp::And => YololExpr::Literal((!y.is_zero() && !z.is_zero()).into()),
                InfixOp::Or => YololExpr::Literal((!y.is_zero() || !z.is_zero()).into()),
                _ => YololExpr::Infix {
                    lhs: Box::new(*lhs),
                    op: op,
                    rhs: Box::new(*rhs),
                },
            },
            _ => YololExpr::Infix {
                lhs: Box::new(reduce_expr(*lhs, vars)),
                op: op,
                rhs: Box::new(reduce_expr(*rhs, vars)),
            },
        },
        // Propagate literals
        YololExpr::Ident(s) => match vars.get(&s) {
            Some(YololExpr::Literal(y)) => YololExpr::Literal(*y),
            _ => YololExpr::Ident(s),
        },
        YololExpr::Literal(y) => YololExpr::Literal(y),
    }
}
