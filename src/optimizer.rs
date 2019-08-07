use crate::ast::{InfixOp, YololNode};

use std::collections::{HashMap, HashSet};

/// Optimizes Yolol assign statements.
pub fn optimize(stmts: &[YololNode], saved: &[String]) -> Vec<YololNode> {
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
    match node {
        YololNode::AssignStmt { ident, expr } => YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(reduce_node(vars, expr)),
        },
        //TODO: implement prefix folding
        YololNode::PrefixExpr { op, expr } => YololNode::PrefixExpr {
            op: op.clone(),
            expr: Box::new(reduce_node(vars, expr)),
        },
        YololNode::InfixExpr { lhs, op, rhs } => match (*lhs.clone(), op, *rhs.clone()) {
            // Fold `0 + n` and `n + 0` to `n`
            (YololNode::Literal(f), InfixOp::Add, _) if f == 0.0 => *rhs.clone(),
            (_, InfixOp::Add, YololNode::Literal(f)) if f == 0.0 => *lhs.clone(),
            // Fold `n - 0` to `n`
            (_, InfixOp::Sub, YololNode::Literal(f)) if f == 0.0 => *lhs.clone(),
            // Fold `0 * n` and `n * 0` to `0`
            (YololNode::Literal(f), InfixOp::Mul, _) if f == 0.0 => *lhs.clone(),
            (_, InfixOp::Mul, YololNode::Literal(f)) if f == 0.0 => *rhs.clone(),
            // Fold `1 * n` and `n * 1` to `n`
            (YololNode::Literal(f), InfixOp::Mul, _) if f == 1.0 => *rhs.clone(),
            (_, InfixOp::Mul, YololNode::Literal(f)) if f == 1.0 => *lhs.clone(),
            // Fold `n / 1` to `n`
            (_, InfixOp::Div, YololNode::Literal(f)) if f == 1.0 => *lhs.clone(),
            // Fold `1 ^ n` to `1`
            (YololNode::Literal(f), InfixOp::Exp, _) if f == 1.0 => *lhs.clone(),
            // Fold `n ^ 1` to `n`
            (_, InfixOp::Exp, YololNode::Literal(f)) if f == 1.0 => *lhs.clone(),
            //TODO: implement literal binary folding
            _ => YololNode::InfixExpr {
                lhs: Box::new(reduce_node(vars, lhs)),
                op: op.clone(),
                rhs: Box::new(reduce_node(vars, rhs)),
            },
        },
        // Propagate literals
        YololNode::Ident(s) => match vars.get(s) {
            Some(YololNode::Literal(f)) => YololNode::Literal(f.clone()),
            Some(node) => reduce_node(vars, node),
            // Ignore imported variable
            None => node.clone(),
        },
        YololNode::Literal(f) => YololNode::Literal(*f),
    }
}

fn eliminate_dead_code(stmts: &[YololNode], saved: &[String]) -> Vec<YololNode> {
    let mut dag: HashMap<String, HashSet<String>> = HashMap::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr } = stmt {
            let mut deps = HashSet::new();
            find_dependencies(&mut deps, expr);
            dag.insert(ident.to_string(), deps);
        } else {
            panic!("expected assign statement, but got: {:?}", stmt)
        }
    }
    let saved = expand_saved(&dag, saved);
    filter_dead_stmts(stmts, &saved)
}

fn find_dependencies(deps: &mut HashSet<String>, expr: &YololNode) {
    match expr {
        YololNode::PrefixExpr { op: _, expr } => find_dependencies(deps, expr),
        YololNode::InfixExpr { lhs, op: _, rhs } => {
            find_dependencies(deps, lhs);
            find_dependencies(deps, rhs);
        }
        YololNode::Ident(s) => {
            deps.insert(s.to_string());
        }
        _ => (),
    }
}

fn expand_saved(dag: &HashMap<String, HashSet<String>>, saved: &[String]) -> Vec<String> {
    let mut expanded = Vec::new();
    let mut queue = saved.to_vec();
    while queue.len() > 0 {
        let ident = queue.pop().unwrap();
        if expanded.contains(&ident) {
            continue;
        }
        expanded.push(ident.to_string());
        if let Some(deps) = dag.get(&ident) {
            for dep in deps.iter() {
                queue.push(dep.to_string());
            }
        }
    }
    expanded
}

fn filter_dead_stmts(stmts: &[YololNode], saved: &[String]) -> Vec<YololNode> {
    let mut filtered = Vec::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr: _ } = stmt {
            if saved.contains(ident) {
                filtered.push(stmt.clone());
            }
        } else {
            panic!("expected assign statement, but got: {:?}", stmt)
        }
    }
    filtered
}
