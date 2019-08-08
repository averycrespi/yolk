use std::collections::{HashMap, HashSet};

use num_traits::identities::{One, Zero};
use yolol_number::YololNumber;

use crate::ast::{InfixOp, PrefixOp, YololNode};
use crate::graph::DepGraph;

/// Optimizes Yolol assign statements.
pub fn optimize(stmts: &[YololNode], saved: &HashSet<String>) -> Vec<YololNode> {
    let mut curr = stmts.to_vec();
    loop {
        let reduced = reduce_constants(&curr);
        if reduced == curr {
            break;
        }
        curr = reduced;
    }
    eliminate_dead_code(&curr, saved)
}

fn reduce_constants(stmts: &[YololNode]) -> Vec<YololNode> {
    let mut reduced = Vec::new();
    let mut variables: HashMap<String, YololNode> = HashMap::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr } = stmt {
            variables.insert(ident.to_string(), *expr.clone());
            reduced.push(reduce_node(&variables, stmt));
        } else {
            panic!("expected Yolol assign statement, but got: {:?}", stmt);
        }
    }
    reduced
}

fn reduce_node(vars: &HashMap<String, YololNode>, node: &YololNode) -> YololNode {
    let zero = YololNumber::zero();
    let one = YololNumber::one();
    match node {
        YololNode::AssignStmt { ident, expr } => YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(reduce_node(vars, expr)),
        },
        YololNode::PrefixExpr { op, expr } => match (op, *expr.clone()) {
            // Fold literals
            (op, YololNode::Literal(y)) => match op {
                PrefixOp::Neg => YololNode::Literal(-y),
                PrefixOp::Not => YololNode::Literal(!y),
                PrefixOp::Abs => YololNode::Literal(y.abs()),
                PrefixOp::Sqrt => YololNode::Literal(y.sqrt()),
                PrefixOp::Sin => YololNode::Literal(y.sin()),
                PrefixOp::Cos => YololNode::Literal(y.cos()),
                PrefixOp::Tan => YololNode::Literal(y.tan()),
                PrefixOp::Asin => YololNode::Literal(y.arcsin()),
                PrefixOp::Acos => YololNode::Literal(y.arccos()),
                PrefixOp::Atan => YololNode::Literal(y.arctan()),
            },
            _ => YololNode::PrefixExpr {
                op: op.clone(),
                expr: Box::new(reduce_node(vars, expr)),
            },
        },
        YololNode::InfixExpr { lhs, op, rhs } => match (*lhs.clone(), op, *rhs.clone()) {
            // Fold `0 + n` and `n + 0` to `n`
            (YololNode::Literal(y), InfixOp::Add, _) if y == zero => *rhs.clone(),
            (_, InfixOp::Add, YololNode::Literal(y)) if y == zero => *lhs.clone(),
            // Fold `n - 0` to `n`
            (_, InfixOp::Sub, YololNode::Literal(y)) if y == zero => *lhs.clone(),
            // Fold `0 * n` and `n * 0` to `0`
            (YololNode::Literal(y), InfixOp::Mul, _) if y == zero => *lhs.clone(),
            (_, InfixOp::Mul, YololNode::Literal(y)) if y == zero => *rhs.clone(),
            // Fold `1 * n` and `n * 1` to `n`
            (YololNode::Literal(y), InfixOp::Mul, _) if y == one => *rhs.clone(),
            (_, InfixOp::Mul, YololNode::Literal(y)) if y == one => *lhs.clone(),
            // Fold `n / 1` to `n`
            (_, InfixOp::Div, YololNode::Literal(y)) if y == one => *lhs.clone(),
            // Fold `1 ^ n` to `1`
            (YololNode::Literal(y), InfixOp::Exp, _) if y == one => *lhs.clone(),
            // Fold `n ^ 1` to `n`
            (_, InfixOp::Exp, YololNode::Literal(y)) if y == one => *lhs.clone(),
            // Fold literals
            (YololNode::Literal(y), op, YololNode::Literal(z)) => match op {
                InfixOp::Add => YololNode::Literal(y + z),
                InfixOp::Sub => YololNode::Literal(y - z),
                InfixOp::Mul => YololNode::Literal(y * z),
                InfixOp::Div => YololNode::Literal(y / z),
                InfixOp::Mod => YololNode::Literal(y % z),
                InfixOp::Exp => YololNode::Literal(y.pow(z)),
                //TODO: handle comparison and boolean
                _ => node.clone(),
            },
            _ => YololNode::InfixExpr {
                lhs: Box::new(reduce_node(vars, lhs)),
                op: op.clone(),
                rhs: Box::new(reduce_node(vars, rhs)),
            },
        },
        // Propagate literals
        YololNode::Ident(s) => match vars.get(s) {
            Some(YololNode::Literal(y)) => YololNode::Literal(y.clone()),
            Some(node) => reduce_node(vars, node),
            // Ignore imported variable
            None => node.clone(),
        },
        YololNode::Literal(_) => node.clone(),
    }
}

fn eliminate_dead_code(stmts: &[YololNode], saved: &HashSet<String>) -> Vec<YololNode> {
    let graph = DepGraph::from_assign_stmts(stmts);
    let saved = graph.search_from(saved);
    let mut living = Vec::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr: _ } = stmt {
            if saved.contains(ident) {
                living.push(stmt.clone());
            }
        } else {
            panic!("expected assign statement, but got: {:?}", stmt)
        }
    }
    living
}
