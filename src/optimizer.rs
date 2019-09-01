use std::collections::{HashMap, HashSet};

use num_traits::identities::{One, Zero};
use yolol_number::prelude::*;

use crate::ast::{InfixOp, PrefixOp, YololExpr, YololStmt};
use crate::environment::Context;
use crate::graph::DepGraph;

/// Optimizes Yolol assign statements.
///
/// Optimization is idempotent when given the same context.
///
/// # Panics
///
/// Panics if any of the statements are malformed.
pub fn optimize(stmts: &[YololStmt], context: &Context) -> Vec<YololStmt> {
    let mut curr = stmts.to_vec();
    let mut next = reduce_constants(&curr);
    while curr != next {
        curr = next;
        next = reduce_constants(&curr);
    }
    eliminate_dead_code(&next, &context.exported())
}

fn reduce_constants(stmts: &[YololStmt]) -> Vec<YololStmt> {
    let mut reduced = Vec::new();
    let mut variables: HashMap<String, YololExpr> = HashMap::new();
    for stmt in stmts.iter() {
        match stmt {
            YololStmt::Assign { ident, expr } => {
                variables.insert(ident.to_string(), *expr.clone());
                reduced.push(reduce_stmt(&variables, stmt));
            }
        }
    }
    reduced
}

fn reduce_stmt(vars: &HashMap<String, YololExpr>, stmt: &YololStmt) -> YololStmt {
    match stmt {
        YololStmt::Assign { ident, expr } => YololStmt::Assign {
            ident: ident.to_string(),
            expr: Box::new(reduce_expr(vars, expr)),
        },
    }
}

fn reduce_expr(vars: &HashMap<String, YololExpr>, expr: &YololExpr) -> YololExpr {
    let zero = YololNumber::zero();
    let one = YololNumber::one();
    match expr {
        YololExpr::Prefix { op, expr } => match (op, *expr.clone()) {
            // Fold literals
            (op, YololExpr::Literal(y)) => match op {
                PrefixOp::Neg => YololExpr::Literal(-y),
                PrefixOp::Not => YololExpr::Literal(!y),
                PrefixOp::Abs => YololExpr::Literal(y.abs()),
                PrefixOp::Sqrt if y >= zero => YololExpr::Literal(y.sqrt()),
                PrefixOp::Sin => YololExpr::Literal(y.sin()),
                PrefixOp::Cos => YololExpr::Literal(y.cos()),
                PrefixOp::Tan if y.cos() != zero => YololExpr::Literal(y.tan()),
                PrefixOp::Asin if y.abs() <= one => YololExpr::Literal(y.asin()),
                PrefixOp::Acos if y.abs() <= one => YololExpr::Literal(y.acos()),
                PrefixOp::Atan => YololExpr::Literal(y.atan()),
                _ => YololExpr::Prefix {
                    op: *op,
                    expr: Box::new(*expr.clone()),
                },
            },
            _ => YololExpr::Prefix {
                op: *op,
                expr: Box::new(reduce_expr(vars, expr)),
            },
        },
        YololExpr::Infix { lhs, op, rhs } => match (*lhs.clone(), op, *rhs.clone()) {
            // Fold `0 + n` and `n + 0` to `n`
            (YololExpr::Literal(y), InfixOp::Add, _) if y == zero => *rhs.clone(),
            (_, InfixOp::Add, YololExpr::Literal(y)) if y == zero => *lhs.clone(),
            // Fold `n - 0` to `n`
            (_, InfixOp::Sub, YololExpr::Literal(y)) if y == zero => *lhs.clone(),
            // Fold `0 * n` and `n * 0` to `0`
            (YololExpr::Literal(y), InfixOp::Mul, _) if y == zero => *lhs.clone(),
            (_, InfixOp::Mul, YololExpr::Literal(y)) if y == zero => *rhs.clone(),
            // Fold `1 * n` and `n * 1` to `n`
            (YololExpr::Literal(y), InfixOp::Mul, _) if y == one => *rhs.clone(),
            (_, InfixOp::Mul, YololExpr::Literal(y)) if y == one => *lhs.clone(),
            // Fold `n / 1` to `n`
            (_, InfixOp::Div, YololExpr::Literal(y)) if y == one => *lhs.clone(),
            // Fold `1 ^ n` to `1`
            (YololExpr::Literal(y), InfixOp::Exp, _) if y == one => *lhs.clone(),
            // Fold `n ^ 1` to `n`
            (_, InfixOp::Exp, YololExpr::Literal(y)) if y == one => *lhs.clone(),
            // Fold literals
            (YololExpr::Literal(y), op, YololExpr::Literal(z)) => match op {
                InfixOp::Add => YololExpr::Literal(y.yolol_add(z)),
                InfixOp::Sub => YololExpr::Literal(y.yolol_sub(z)),
                InfixOp::Mul => YololExpr::Literal(y.yolol_mul(z)),
                //TODO: prevent panic here
                InfixOp::Div if !z.is_zero() => YololExpr::Literal(y.yolol_div(z).unwrap()),
                InfixOp::Mod if !z.is_zero() => YololExpr::Literal(y.yolol_mod(z)),
                InfixOp::Exp if !(y.is_zero() && z.is_zero()) => YololExpr::Literal(y.pow(z)),
                //TODO: replace with (y < z).into()
                InfixOp::LessThan => YololExpr::Literal(From::from(y < z)),
                InfixOp::LessEqual => YololExpr::Literal(From::from(y <= z)),
                InfixOp::GreaterThan => YololExpr::Literal(From::from(y > z)),
                InfixOp::GreaterEqual => YololExpr::Literal(From::from(y >= z)),
                InfixOp::Equal => YololExpr::Literal(From::from(y == z)),
                InfixOp::NotEqual => YololExpr::Literal(From::from(y != z)),
                InfixOp::And => YololExpr::Literal(From::from((y != zero) && (z != zero))),
                InfixOp::Or => YololExpr::Literal(From::from((y != zero) || (z != zero))),
                _ => YololExpr::Infix {
                    lhs: Box::new(*lhs.clone()),
                    op: *op,
                    rhs: Box::new(*rhs.clone()),
                },
            },
            _ => YololExpr::Infix {
                lhs: Box::new(reduce_expr(vars, lhs)),
                op: *op,
                rhs: Box::new(reduce_expr(vars, rhs)),
            },
        },
        // Propagate literals
        YololExpr::Ident(s) => match vars.get(s) {
            Some(YololExpr::Literal(y)) => YololExpr::Literal(y.clone()),
            Some(node) => reduce_expr(vars, node),
            // Ignore imported variable
            None => expr.clone(),
        },
        YololExpr::Literal(_) => expr.clone(),
    }
}

fn eliminate_dead_code(stmts: &[YololStmt], exported: &HashSet<String>) -> Vec<YololStmt> {
    let graph = DepGraph::from_statements(stmts);
    let exported = graph.search_from(exported);
    let mut living = Vec::new();
    for stmt in stmts.iter() {
        match stmt {
            YololStmt::Assign { ident, expr: _ } => {
                if exported.contains(ident) {
                    living.push(stmt.clone());
                }
            }
        }
    }
    living
}
