use crate::ast::YololNode;

use std::collections::HashMap;

/// Optimizes Yolol assign statements.
pub fn optimize(stmts: &[YololNode]) -> Vec<YololNode> {
    // TODO: implement
    let propagated = propagate_constants(stmts);
    let folded = fold_constants(&propagated);
    folded
}

fn propagate_constants(stmts: &[YololNode]) -> Vec<YololNode> {
    let mut propagated = Vec::new();
    let mut variables: HashMap<String, YololNode> = HashMap::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr } = stmt {
            variables.insert(ident.to_string(), *expr.clone());
            propagated.push(propagate_node(&variables, stmt));
        } else {
            panic!("expected Yolol assign statement, but got: {:?}", stmt);
        }
    }
    propagated
}

fn propagate_node(variables: &HashMap<String, YololNode>, node: &YololNode) -> YololNode {
    match node {
        YololNode::AssignStmt { ident, expr } => YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(propagate_node(variables, expr)),
        },
        YololNode::PrefixExpr { op, expr } => YololNode::PrefixExpr {
            op: op.clone(),
            expr: Box::new(propagate_node(variables, expr)),
        },
        YololNode::InfixExpr { lhs, op, rhs } => YololNode::InfixExpr {
            lhs: Box::new(propagate_node(variables, lhs)),
            op: op.clone(),
            rhs: Box::new(propagate_node(variables, rhs)),
        },
        YololNode::Ident(s) => match variables.get(s) {
            Some(YololNode::Literal(f)) => YololNode::Literal(f.clone()),
            Some(node) => propagate_node(variables, node),
            None => panic!("expected Yolol variable to be defined: {:?}", s),
        },
        YololNode::Literal(f) => YololNode::Literal(f.clone()),
    }
}

fn fold_constants(stmts: &[YololNode]) -> Vec<YololNode> {
    let mut folded = Vec::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident: _, expr: _ } = stmt {
            folded.push(fold_node(stmt));
        } else {
            panic!("expected Yolol assign statement, but got: {:?}", stmt);
        }
    }
    folded
}

fn fold_node(node: &YololNode) -> YololNode {
    match node {
        YololNode::AssignStmt { ident, expr } => YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(fold_node(expr)),
        },
        //TODO: implement prefix folding
        YololNode::PrefixExpr { op, expr } => YololNode::PrefixExpr {
            op: op.clone(),
            expr: Box::new(fold_node(expr)),
        },
        //TODO: implement infix folding
        YololNode::InfixExpr { lhs, op, rhs } => YololNode::InfixExpr {
            lhs: Box::new(fold_node(lhs)),
            op: op.clone(),
            rhs: Box::new(fold_node(rhs)),
        },
        YololNode::Ident(s) => YololNode::Ident(s.to_string()),
        YololNode::Literal(f) => YololNode::Literal(*f),
    }
}
