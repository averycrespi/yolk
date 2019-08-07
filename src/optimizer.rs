use crate::ast::YololNode;

use std::collections::HashMap;

/// Optimizes Yolol assign statements.
pub fn optimize(stmts: &[YololNode]) -> Vec<YololNode> {
    // TODO: implement
    propagate_constants(stmts)
}

fn propagate_constants(stmts: &[YololNode]) -> Vec<YololNode> {
    let mut propagated = Vec::new();
    let mut variables: HashMap<String, YololNode> = HashMap::new();
    for stmt in stmts.iter() {
        if let YololNode::AssignStmt { ident, expr } = stmt {
            variables.insert(ident.to_string(), *expr.clone());
            propagated.push(propagate(&variables, stmt));
        } else {
            panic!("expected Yolol assign statement, but got: {:?}", stmt);
        }
    }
    propagated
}

fn propagate(variables: &HashMap<String, YololNode>, node: &YololNode) -> YololNode {
    match node {
        YololNode::AssignStmt { ident, expr } => YololNode::AssignStmt {
            ident: ident.to_string(),
            expr: Box::new(propagate(variables, expr)),
        },
        YololNode::PrefixExpr { op, expr } => YololNode::PrefixExpr {
            op: op.clone(),
            expr: Box::new(propagate(variables, expr)),
        },
        YololNode::InfixExpr { lhs, op, rhs } => YololNode::InfixExpr {
            lhs: Box::new(propagate(variables, lhs)),
            op: op.clone(),
            rhs: Box::new(propagate(variables, rhs)),
        },
        YololNode::Ident(s) => match variables.get(s) {
            Some(YololNode::Literal(f)) => YololNode::Literal(f.clone()),
            Some(node) => propagate(variables, node),
            None => panic!("expected Yolol variable to be defined: {:?}", s),
        },
        YololNode::Literal(f) => YololNode::Literal(f.clone()),
    }
}
